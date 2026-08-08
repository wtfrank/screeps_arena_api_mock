// Direct translation of Screeps native C++ pf.cc / pf.h pathfinder to Rust
// Original Author: Marcel Laverdet <https://github.com/laverdet>

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::game::pathfinder::{CostMatrix, GoalSpec, Position, SearchPathOptions, SearchResults};
use crate::traits::HasPosition;

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapState {
    f_cost: u32,
    insert_id: u64,
    pos: Position,
}

impl Ord for HeapState {
    fn cmp(&self, other: &Self) -> Ordering {
        // min-heap: lower f_cost pops first; LIFO tie-break on insert_id
        other.f_cost.cmp(&self.f_cost)
            .then_with(|| other.insert_id.cmp(&self.insert_id))
    }
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// (val + 1) % 50 < 2  →  val % 50 == 0 or val % 50 == 49
// For a 100x100 arena map: borders at 0 and 99
fn is_border_pos(val: u8) -> bool {
    val == 0 || val == 99
}

// (val + 2) % 50 < 4  →  val % 50 ∈ {0, 1, 48, 49}
// For 100x100 arena map: near borders at 0, 1, 98, 99
fn is_near_border_pos(val: u8) -> bool {
    val <= 1 || val >= 98
}

// Safe look with bounds check (returns None for out-of-bounds)
#[inline]
fn look_at<L: Fn(u8, u8) -> Option<u32>>(ix: i32, iy: i32, look: &L) -> Option<u32> {
    if ix < 0 || ix >= 100 || iy < 0 || iy >= 100 {
        None
    } else {
        look(ix as u8, iy as u8)
    }
}

// Chebyshev range between two positions (integer)
fn range_to(a: Position, b: Position) -> u32 {
    let dx = (a.x as i32 - b.x as i32).unsigned_abs();
    let dy = (a.y as i32 - b.y as i32).unsigned_abs();
    dx.max(dy)
}

// pf.cc jump_x: horizontal ray scan
// cost = cost of the tiles along the main ray (cost at jump origin)
// pos  = first position to visit (already one step in direction dx)
// Returns None if the ray hits a wall; Some(pos) for the jump point
fn jump_x<L, H>(cost: u32, mut pos: Position, dx: i32, look: &L, heuristic: &H) -> Option<Position>
where
    L: Fn(u8, u8) -> Option<u32>,
    H: Fn(Position) -> u32,
{
    let mut prev_cost_u = look_at(pos.x as i32, pos.y as i32 - 1, look);
    let mut prev_cost_d = look_at(pos.x as i32, pos.y as i32 + 1, look);
    loop {
        if heuristic(pos) == 0 || is_near_border_pos(pos.x) {
            break;
        }
        let nx = pos.x as i32 + dx;
        let cost_u = look_at(nx, pos.y as i32 - 1, look);
        let cost_d = look_at(nx, pos.y as i32 + 1, look);
        if (cost_u.is_some() && prev_cost_u != Some(cost)) ||
           (cost_d.is_some() && prev_cost_d != Some(cost)) {
            break;
        }
        prev_cost_u = cost_u;
        prev_cost_d = cost_d;
        pos.x = nx as u8;
        let jump_cost = look(pos.x, pos.y);
        if jump_cost.is_none() {
            return None;
        } else if jump_cost != Some(cost) {
            break;
        }
    }
    Some(pos)
}

// pf.cc jump_y: vertical ray scan
fn jump_y<L, H>(cost: u32, mut pos: Position, dy: i32, look: &L, heuristic: &H) -> Option<Position>
where
    L: Fn(u8, u8) -> Option<u32>,
    H: Fn(Position) -> u32,
{
    let mut prev_cost_l = look_at(pos.x as i32 - 1, pos.y as i32, look);
    let mut prev_cost_r = look_at(pos.x as i32 + 1, pos.y as i32, look);
    loop {
        if heuristic(pos) == 0 || is_near_border_pos(pos.y) {
            break;
        }
        let ny = pos.y as i32 + dy;
        let cost_l = look_at(pos.x as i32 - 1, ny, look);
        let cost_r = look_at(pos.x as i32 + 1, ny, look);
        if (cost_l.is_some() && prev_cost_l != Some(cost)) ||
           (cost_r.is_some() && prev_cost_r != Some(cost)) {
            break;
        }
        prev_cost_l = cost_l;
        prev_cost_r = cost_r;
        pos.y = ny as u8;
        let jump_cost = look(pos.x, pos.y);
        if jump_cost.is_none() {
            return None;
        } else if jump_cost != Some(cost) {
            break;
        }
    }
    Some(pos)
}

// pf.cc jump_xy: diagonal ray scan
fn jump_xy<L, H>(cost: u32, mut pos: Position, dx: i32, dy: i32, look: &L, heuristic: &H) -> Option<Position>
where
    L: Fn(u8, u8) -> Option<u32>,
    H: Fn(Position) -> u32,
{
    // prev_cost_x = look(pos.x - dx, pos.y)  [behind in x]
    let mut prev_cost_x = look_at(pos.x as i32 - dx, pos.y as i32, look);
    // prev_cost_y = look(pos.x, pos.y - dy)  [behind in y]
    let mut prev_cost_y = look_at(pos.x as i32, pos.y as i32 - dy, look);
    loop {
        if heuristic(pos) == 0 || is_near_border_pos(pos.x) || is_near_border_pos(pos.y) {
            break;
        }
        // Forced neighbor check (pf.cc lines 263-268)
        let diag_x = look_at(pos.x as i32 - dx, pos.y as i32 + dy, look); // (-dx, +dy)
        let diag_y = look_at(pos.x as i32 + dx, pos.y as i32 - dy, look); // (+dx, -dy)
        if (diag_x.is_some() && prev_cost_x != Some(cost)) ||
           (diag_y.is_some() && prev_cost_y != Some(cost)) {
            break;
        }
        // Update prev_cost for next diagonal step (pf.cc lines 269-270)
        prev_cost_x = look_at(pos.x as i32, pos.y as i32 + dy, look); // look(pos.x, pos.y + dy)
        prev_cost_y = look_at(pos.x as i32 + dx, pos.y as i32, look); // look(pos.x + dx, pos.y)
        // Diagonal sub-scans (pf.cc lines 271-276)
        // prev_cost_y = look(pos.x + dx, pos.y), prev_cost_x = look(pos.x, pos.y + dy)
        if (prev_cost_y.is_some() && jump_x(cost, Position { x: (pos.x as i32 + dx) as u8, y: pos.y }, dx, look, heuristic).is_some()) ||
           (prev_cost_x.is_some() && jump_y(cost, Position { x: pos.x, y: (pos.y as i32 + dy) as u8 }, dy, look, heuristic).is_some()) {
            break;
        }
        // Advance diagonally (pf.cc lines 278-279)
        pos.x = (pos.x as i32 + dx) as u8;
        pos.y = (pos.y as i32 + dy) as u8;
        let jump_cost = look(pos.x, pos.y);
        if jump_cost.is_none() {
            return None;
        } else if jump_cost != Some(cost) {
            break;
        }
    }
    Some(pos)
}

// pf.cc jump(): dispatch to appropriate jump function
fn jump<L, H>(cost: u32, pos: Position, dx: i32, dy: i32, look: &L, heuristic: &H) -> Option<Position>
where
    L: Fn(u8, u8) -> Option<u32>,
    H: Fn(Position) -> u32,
{
    if dx != 0 {
        if dy != 0 {
            jump_xy(cost, pos, dx, dy, look, heuristic)
        } else {
            jump_x(cost, pos, dx, look, heuristic)
        }
    } else {
        jump_y(cost, pos, dy, look, heuristic)
    }
}

pub fn search_path_pf_cc(
    origin: &wasm_bindgen::JsValue,
    goal: &wasm_bindgen::JsValue,
    options: Option<&SearchPathOptions>,
) -> SearchResults {
    let start = if let Ok(pos) = serde_wasm_bindgen::from_value::<Position>(origin.clone()) {
        pos
    } else {
        unsafe {
            let go = &*(origin as *const wasm_bindgen::JsValue as *const crate::objects::GameObject);
            go.pos()
        }
    };

    let mut goals: Vec<GoalSpec> = Vec::new();
    if let Ok(single) = serde_wasm_bindgen::from_value::<GoalSpec>(goal.clone()) {
        goals.push(single);
    } else if let Ok(pos) = serde_wasm_bindgen::from_value::<Position>(goal.clone()) {
        goals.push(GoalSpec { pos, range: 0 });
    } else if let Ok(multi) = serde_wasm_bindgen::from_value::<Vec<GoalSpec>>(goal.clone()) {
        goals = multi;
    } else if let Ok(multi_pos) = serde_wasm_bindgen::from_value::<Vec<Position>>(goal.clone()) {
        goals = multi_pos.into_iter().map(|pos| GoalSpec { pos, range: 0 }).collect();
    }

    if goals.is_empty() {
        return SearchResults {
            path: Vec::new(),
            ops: 0,
            cost: 0,
            incomplete: false,
        };
    }

    let plain_cost = options.and_then(|o| o.plain_cost.get()).unwrap_or(2) as u32;
    let swamp_cost = options.and_then(|o| o.swamp_cost.get()).unwrap_or(10) as u32;
    let heuristic_weight = options.and_then(|o| o.heuristic_weight.get()).unwrap_or(1.2);
    let max_ops = options.and_then(|o| o.max_ops.get()).unwrap_or(50000);
    let flee = options.and_then(|o| o.flee.get()).unwrap_or(false);
    let custom_cm: Option<CostMatrix> = options.and_then(|o| o.cost_matrix.borrow().clone());

    // pf.cc look(): returns None for obstacle, Some(cost) for passable
    let look = |x: u8, y: u8| -> Option<u32> {
        if let Some(cm) = custom_cm.as_ref() {
            let c = cm.get(x, y);
            if c == 255 {
                return None;
            } else if c > 0 {
                return Some(c as u32);
            }
        }
        match crate::game::utils::get_terrain_at_pos(x, y) {
            crate::constants::Terrain::Wall => None,
            crate::constants::Terrain::Swamp => Some(swamp_cost),
            _ => Some(plain_cost),
        }
    };

    let heuristic = |pos: Position| -> u32 {
        if flee {
            let mut ret: u32 = 0;
            for g in &goals {
                let dist = range_to(pos, g.pos);
                let r = g.range as u32;
                if dist < r {
                    ret = ret.max(r - dist);
                }
            }
            ret
        } else {
            let mut ret: u32 = u32::MAX;
            for g in &goals {
                let dist = range_to(pos, g.pos);
                let r = g.range as u32;
                if dist > r {
                    ret = ret.min(dist - r);
                } else {
                    return 0;
                }
            }
            if ret == u32::MAX { 0 } else { ret }
        }
    };

    // pf.cc: special case if origin is already at goal
    if heuristic(start) == 0 {
        return SearchResults {
            path: Vec::new(),
            ops: 0,
            cost: 0,
            incomplete: false,
        };
    }

    log::warn!("[pf_cc] search_path_pf_cc start={:?}, goal={:?}, flee={}", start, goals, flee);

    let mut heap: BinaryHeap<HeapState> = BinaryHeap::new();
    // g_score stores the actual path cost to reach each position
    let mut g_score: HashMap<Position, u32> = HashMap::new();
    let mut parents: HashMap<Position, Position> = HashMap::new();
    let mut closed: HashSet<Position> = HashSet::new();
    let mut insert_counter: u64 = 0;

    // pf.cc push_node: insert or update a node in the heap
    let push_node = |
        heap: &mut BinaryHeap<HeapState>,
        g_score: &mut HashMap<Position, u32>,
        parents: &mut HashMap<Position, Position>,
        closed: &HashSet<Position>,
        insert_counter: &mut u64,
        p_from: Position,
        p_to: Position,
        g_cost: u32,
    | {
        if closed.contains(&p_to) {
            return;
        }
        if g_cost < *g_score.get(&p_to).unwrap_or(&u32::MAX) {
            g_score.insert(p_to, g_cost);
            parents.insert(p_to, p_from);
            let h_cost = heuristic(p_to);
            // pf.cc: cost_t h_weighted = heuristic(node) * heuristic_weight; (double→uint32 truncation)
            // pf.cc: cost_t f_cost = h_weighted + g_cost;
            let h_weighted = (h_cost as f64 * heuristic_weight) as u32;
            let f_cost = g_cost.saturating_add(h_weighted);
            *insert_counter += 1;
            heap.push(HeapState {
                f_cost,
                insert_id: *insert_counter,
                pos: p_to,
            });
        }
    };

    // pf.cc astar(): expand all 8 neighbors (used for origin node)
    {
        let dirs: [(i32, i32); 8] = [
            (0, -1), (1, -1), (1, 0), (1, 1),
            (0, 1), (-1, 1), (-1, 0), (-1, -1),
        ];
        for &(ddx, ddy) in &dirs {
            let nx = start.x as i32 + ddx;
            let ny = start.y as i32 + ddy;
            if nx < 0 || nx >= 100 || ny < 0 || ny >= 100 {
                continue;
            }
            let n_pos = Position { x: nx as u8, y: ny as u8 };
            if let Some(n_cost) = look(n_pos.x, n_pos.y) {
                push_node(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, start, n_pos, n_cost);
            }
        }
    }

    let mut ops: u32 = 0;
    // Track the closest node to goal seen so far (for incomplete path)
    let mut min_node: Option<Position> = None;
    let mut min_node_h: u32 = u32::MAX;
    let mut min_node_g: u32 = 0;

    'outer: while let Some(HeapState { f_cost: _, pos, insert_id: _ }) = heap.pop() {
        if closed.contains(&pos) {
            continue;
        }
        closed.insert(pos);

        let g_cost = *g_score.get(&pos).unwrap_or(&0);
        let h_cost = heuristic(pos);

        // pf.cc: reached destination?
        if h_cost == 0 {
            min_node = Some(pos);
            min_node_h = 0;
            min_node_g = g_cost;
            break;
        } else if h_cost < min_node_h {
            min_node = Some(pos);
            min_node_h = h_cost;
            min_node_g = g_cost;
        }

        ops += 1;
        if ops > max_ops {
            break;
        }

        // pf.cc jps(): compute direction from parent
        let parent_opt = parents.get(&pos).copied();
        let (dx, dy) = if let Some(p) = parent_opt {
            ((pos.x as i32 - p.x as i32).signum(), (pos.y as i32 - p.y as i32).signum())
        } else {
            (0, 0)
        };

        let cost = look(pos.x, pos.y).unwrap_or(plain_cost);

        // pf.cc: border handling (for border positions, use special neighbor sets)
        // For Arena 100x100 single room, we skip room-portal logic but keep near-border dx/dy clamping
        let border_dx: i32 = if pos.x == 1 { -1 } else if pos.x == 98 { 1 } else { 0 };
        let border_dy: i32 = if pos.y == 1 { -1 } else if pos.y == 98 { 1 } else { 0 };

        // pf.cc jump_neighbor helper (inlined as closure)
        // Decides whether to call jump() or push directly
        let mut do_jump_neighbor = |
            heap: &mut BinaryHeap<HeapState>,
            g_score: &mut HashMap<Position, u32>,
            parents: &mut HashMap<Position, Position>,
            closed: &HashSet<Position>,
            insert_counter: &mut u64,
            neighbor: Position,
            neighbor_g: u32,
            n_cost: u32,
        | {
            if n_cost == 0 {
                // obstacle check already done before calling
                return;
            }
            if n_cost != cost || is_border_pos(neighbor.x) || is_border_pos(neighbor.y) {
                // Push neighbor directly
                push_node(heap, g_score, parents, closed, insert_counter, pos, neighbor, neighbor_g + n_cost);
            } else {
                // Call jump() to find the jump point
                let ndx = neighbor.x as i32 - pos.x as i32;
                let ndy = neighbor.y as i32 - pos.y as i32;
                if let Some(jump_pt) = jump(n_cost, neighbor, ndx, ndy, &look, &heuristic) {
                    let jump_cost = look(jump_pt.x, jump_pt.y).unwrap_or(plain_cost);
                    let dist = range_to(pos, jump_pt);
                    // pf.cc: g_cost += n_cost * (pos.range_to(neighbor) - 1) + look(neighbor)
                    // here neighbor = jump_pt, pos = current pos
                    let total_g = neighbor_g + n_cost * (dist.saturating_sub(1)) + jump_cost;
                    push_node(heap, g_score, parents, closed, insert_counter, pos, jump_pt, total_g);
                }
            }
        };

        // pf.cc jps() main body
        if dx != 0 {
            // Straight x or diagonal
            let nx = pos.x as i32 + dx;
            if nx >= 0 && nx < 100 {
                let neighbor = Position { x: nx as u8, y: pos.y };
                if let Some(n_cost) = look(neighbor.x, neighbor.y) {
                    if border_dy == 0 {
                        do_jump_neighbor(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, neighbor, g_cost, n_cost);
                    } else {
                        push_node(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, pos, neighbor, g_cost + n_cost);
                    }
                }
            }
        }
        if dy != 0 {
            // Straight y or diagonal
            let ny = pos.y as i32 + dy;
            if ny >= 0 && ny < 100 {
                let neighbor = Position { x: pos.x, y: ny as u8 };
                if let Some(n_cost) = look(neighbor.x, neighbor.y) {
                    if border_dx == 0 {
                        do_jump_neighbor(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, neighbor, g_cost, n_cost);
                    } else {
                        push_node(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, pos, neighbor, g_cost + n_cost);
                    }
                }
            }
        }

        // Forced neighbor rules (pf.cc lines 407-436)
        if dx != 0 {
            if dy != 0 {
                // Diagonal: push diagonal neighbor + forced diagonals
                let nx = pos.x as i32 + dx;
                let ny = pos.y as i32 + dy;
                if nx >= 0 && nx < 100 && ny >= 0 && ny < 100 {
                    let neighbor = Position { x: nx as u8, y: ny as u8 };
                    if let Some(n_cost) = look(neighbor.x, neighbor.y) {
                        do_jump_neighbor(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, neighbor, g_cost, n_cost);
                    }
                }
                // pf.cc line 415: if (look(pos.x - dx, pos.y) != cost)
                if look_at(pos.x as i32 - dx, pos.y as i32, &look) != Some(cost) {
                    let fx = pos.x as i32 - dx;
                    let fy = pos.y as i32 + dy;
                    if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                        let fn_pos = Position { x: fx as u8, y: fy as u8 };
                        if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                            do_jump_neighbor(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, fn_pos, g_cost, n_cost);
                        }
                    }
                }
                // pf.cc line 418: if (look(pos.x, pos.y - dy) != cost)
                if look_at(pos.x as i32, pos.y as i32 - dy, &look) != Some(cost) {
                    let fx = pos.x as i32 + dx;
                    let fy = pos.y as i32 - dy;
                    if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                        let fn_pos = Position { x: fx as u8, y: fy as u8 };
                        if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                            do_jump_neighbor(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, fn_pos, g_cost, n_cost);
                        }
                    }
                }
            } else {
                // Straight x: check forced neighbors above/below
                // pf.cc line 422: border_dy == 1 || look(pos.x, pos.y + 1) != cost
                if border_dy == 1 || look_at(pos.x as i32, pos.y as i32 + 1, &look) != Some(cost) {
                    let fx = pos.x as i32 + dx;
                    let fy = pos.y as i32 + 1;
                    if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                        let fn_pos = Position { x: fx as u8, y: fy as u8 };
                        if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                            do_jump_neighbor(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, fn_pos, g_cost, n_cost);
                        }
                    }
                }
                // pf.cc line 425: border_dy == -1 || look(pos.x, pos.y - 1) != cost
                if border_dy == -1 || look_at(pos.x as i32, pos.y as i32 - 1, &look) != Some(cost) {
                    let fx = pos.x as i32 + dx;
                    let fy = pos.y as i32 - 1;
                    if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                        let fn_pos = Position { x: fx as u8, y: fy as u8 };
                        if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                            do_jump_neighbor(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, fn_pos, g_cost, n_cost);
                        }
                    }
                }
            }
        } else if dy != 0 {
            // Straight y: check forced neighbors left/right
            // pf.cc line 430: border_dx == 1 || look(pos.x + 1, pos.y) != cost
            if border_dx == 1 || look_at(pos.x as i32 + 1, pos.y as i32, &look) != Some(cost) {
                let fx = pos.x as i32 + 1;
                let fy = pos.y as i32 + dy;
                if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                    let fn_pos = Position { x: fx as u8, y: fy as u8 };
                    if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                        do_jump_neighbor(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, fn_pos, g_cost, n_cost);
                    }
                }
            }
            // pf.cc line 433: border_dx == -1 || look(pos.x - 1, pos.y) != cost
            if border_dx == -1 || look_at(pos.x as i32 - 1, pos.y as i32, &look) != Some(cost) {
                let fx = pos.x as i32 - 1;
                let fy = pos.y as i32 + dy;
                if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                    let fn_pos = Position { x: fx as u8, y: fy as u8 };
                    if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                        do_jump_neighbor(&mut heap, &mut g_score, &mut parents, &closed, &mut insert_counter, fn_pos, g_cost, n_cost);
                    }
                }
            }
        }
    }

    let incomplete = min_node_h != 0;
    log::warn!("[pf_cc] search_path_pf_cc {} ops={}", if incomplete { "NO PATH" } else { "FOUND" }, ops);

    let Some(goal_node) = min_node else {
        return SearchResults { path: Vec::new(), ops, cost: 0, incomplete: true };
    };

    // pf.cc path reconstruction (lines 570-594): interpolate between jump points
    let mut path: Vec<Position> = Vec::new();
    let mut pos = goal_node;
    while pos != start {
        path.push(pos);
        let parent = match parents.get(&pos).copied() {
            Some(p) => p,
            None => break,
        };
        // If jump point is more than 1 step away, interpolate intermediate positions
        if range_to(pos, parent) > 1 {
            // direction from pos toward parent (we're reconstructing backwards)
            let ddx = (parent.x as i32 - pos.x as i32).signum();
            let ddy = (parent.y as i32 - pos.y as i32).signum();
            let mut cur = pos;
            while range_to(cur, parent) > 1 {
                cur.x = (cur.x as i32 + ddx) as u8;
                cur.y = (cur.y as i32 + ddy) as u8;
                path.push(cur);
            }
        }
        pos = parent;
    }
    // Path is built backwards (goal→start), reverse to get start→goal
    path.reverse();

    SearchResults {
        path,
        ops,
        cost: min_node_g,
        incomplete,
    }
}
