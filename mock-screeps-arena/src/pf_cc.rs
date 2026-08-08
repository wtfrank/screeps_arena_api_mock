// Direct translation of Screeps native C++ pf.cc / pf.h pathfinder to Rust
// Original Author: Marcel Laverdet <https://github.com/laverdet>

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::game::pathfinder::{CostMatrix, GoalSpec, Position, SearchPathOptions, SearchResults};
use crate::pf_cc_heap::{OpenClosedList, PfCcHeap};
use crate::traits::HasPosition;

#[inline]
fn pos_index(pos: Position) -> usize {
    let room_x = (pos.x / 50) as usize;
    let room_y = (pos.y / 50) as usize;
    let room_index = room_x * 2 + room_y;
    let local_x = (pos.x % 50) as usize;
    let local_y = (pos.y % 50) as usize;
    room_index * 2500 + local_x * 50 + local_y
}

#[inline]
fn pos_from_index(idx: usize) -> Position {
    let room_index = idx / 2500;
    let coord = idx % 2500;
    let room_x = (room_index / 2) as u8;
    let room_y = (room_index % 2) as u8;
    let local_x = (coord / 50) as u8;
    let local_y = (coord % 50) as u8;
    Position {
        x: room_x * 50 + local_x,
        y: room_y * 50 + local_y,
    }
}

fn is_border_pos(val: u8) -> bool {
    (val as u32 + 1) % 50 < 2
}

fn is_near_border_pos(val: u8) -> bool {
    (val as u32 + 2) % 50 < 4
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

fn is_passable(val: Option<u32>) -> bool {
    matches!(val, Some(c) if c != 255)
}

// pf.cc jump_x: horizontal ray scan
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
        if (is_passable(cost_u) && prev_cost_u != Some(cost)) ||
           (is_passable(cost_d) && prev_cost_d != Some(cost)) {
            break;
        }
        prev_cost_u = cost_u;
        prev_cost_d = cost_d;
        pos.x = nx as u8;
        let jump_cost = look(pos.x, pos.y);
        if jump_cost.is_none() || jump_cost == Some(255) {
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
        if (is_passable(cost_l) && prev_cost_l != Some(cost)) ||
           (is_passable(cost_r) && prev_cost_r != Some(cost)) {
            break;
        }
        prev_cost_l = cost_l;
        prev_cost_r = cost_r;
        pos.y = ny as u8;
        let jump_cost = look(pos.x, pos.y);
        if jump_cost.is_none() || jump_cost == Some(255) {
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
    let mut prev_cost_x = look_at(pos.x as i32 - dx, pos.y as i32, look);
    let mut prev_cost_y = look_at(pos.x as i32, pos.y as i32 - dy, look);
    loop {
        if heuristic(pos) == 0 || is_near_border_pos(pos.x) || is_near_border_pos(pos.y) {
            break;
        }
        let diag_x = look_at(pos.x as i32 - dx, pos.y as i32 + dy, look);
        let diag_y = look_at(pos.x as i32 + dx, pos.y as i32 - dy, look);
        if (is_passable(diag_x) && prev_cost_x != Some(cost)) ||
           (is_passable(diag_y) && prev_cost_y != Some(cost)) {
            break;
        }
        prev_cost_x = look_at(pos.x as i32, pos.y as i32 + dy, look);
        prev_cost_y = look_at(pos.x as i32 + dx, pos.y as i32, look);
        if (is_passable(prev_cost_y) && jump_x(cost, Position { x: (pos.x as i32 + dx) as u8, y: pos.y }, dx, look, heuristic).is_some()) ||
           (is_passable(prev_cost_x) && jump_y(cost, Position { x: pos.x, y: (pos.y as i32 + dy) as u8 }, dy, look, heuristic).is_some()) {
            break;
        }
        pos.x = (pos.x as i32 + dx) as u8;
        pos.y = (pos.y as i32 + dy) as u8;
        let jump_cost = look(pos.x, pos.y);
        if jump_cost.is_none() || jump_cost == Some(255) {
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

    let mut heap = PfCcHeap::new();
    let mut open_closed = OpenClosedList::new();
    let mut parents: [usize; 10000] = [0; 10000];

    // pf.cc push_node: exact translation of lines 83-103 in pf.cc
    let mut push_node = |
        heap: &mut PfCcHeap,
        parents: &mut [usize; 10000],
        open_closed: &mut OpenClosedList,
        parent_index: usize,
        node: Position,
        g_cost: u32,
    | {
        let index = pos_index(node);
        if open_closed.is_closed(index) {
            return;
        }
        let h_cost = (heuristic(node) as f64 * heuristic_weight) as u32;
        let f_cost = g_cost.saturating_add(h_cost);

        if open_closed.is_open(index) {
            if heap.priority(index) > f_cost {
                heap.update(index, f_cost);
                parents[index] = parent_index;
            }
        } else {
            heap.insert(index, f_cost);
            open_closed.open(index);
            parents[index] = parent_index;
        }
    };

    let start_index = pos_index(start);
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
                push_node(&mut heap, &mut parents, &mut open_closed, start_index, n_pos, n_cost);
            }
        }
    }
    open_closed.close(start_index);

    let mut ops: u32 = 0;
    let mut min_node: Option<Position> = None;
    let mut min_node_h: u32 = u32::MAX;
    let mut min_node_g: u32 = u32::MAX;

    // pf.cc main A* loop (lines 526-563)
    while !heap.empty() && ops < max_ops {
        let (current_index, current_priority) = heap.pop();
        open_closed.close(current_index);

        let pos = pos_from_index(current_index);
        let h_cost = heuristic(pos);
        let h_weighted = (h_cost as f64 * heuristic_weight) as u32;
        let g_cost = current_priority.saturating_sub(h_weighted);

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

        // pf.cc jps(): compute direction from parent
        let parent_index = parents[current_index];
        let parent = pos_from_index(parent_index);
        let dx = (pos.x as i32 - parent.x as i32).signum();
        let dy = (pos.y as i32 - parent.y as i32).signum();

        let cost = look(pos.x, pos.y).unwrap_or(plain_cost);

        let border_dx: i32 = if pos.x % 50 == 1 { -1 } else if pos.x % 50 == 48 { 1 } else { 0 };
        let border_dy: i32 = if pos.y % 50 == 1 { -1 } else if pos.y % 50 == 48 { 1 } else { 0 };

        let mut do_jump_neighbor = |
            heap: &mut PfCcHeap,
            parents: &mut [usize; 10000],
            open_closed: &mut OpenClosedList,
            neighbor: Position,
            neighbor_g: u32,
            n_cost: u32,
        | {
            if n_cost == 0 {
                return;
            }
            if n_cost != cost || is_border_pos(neighbor.x) || is_border_pos(neighbor.y) {
                push_node(heap, parents, open_closed, current_index, neighbor, neighbor_g + n_cost);
            } else {
                let ndx = neighbor.x as i32 - pos.x as i32;
                let ndy = neighbor.y as i32 - pos.y as i32;
                if let Some(jump_pt) = jump(n_cost, neighbor, ndx, ndy, &look, &heuristic) {
                    let jump_cost = look(jump_pt.x, jump_pt.y).unwrap_or(plain_cost);
                    let dist = range_to(pos, jump_pt);
                    let total_g = neighbor_g + n_cost * (dist.saturating_sub(1)) + jump_cost;
                    push_node(heap, parents, open_closed, current_index, jump_pt, total_g);
                }
            }
        };

        if dx != 0 {
            let nx = pos.x as i32 + dx;
            if nx >= 0 && nx < 100 {
                let neighbor = Position { x: nx as u8, y: pos.y };
                if let Some(n_cost) = look(neighbor.x, neighbor.y) {
                    if border_dy == 0 {
                        do_jump_neighbor(&mut heap, &mut parents, &mut open_closed, neighbor, g_cost, n_cost);
                    } else {
                        push_node(&mut heap, &mut parents, &mut open_closed, current_index, neighbor, g_cost + n_cost);
                    }
                }
            }
        }
        if dy != 0 {
            let ny = pos.y as i32 + dy;
            if ny >= 0 && ny < 100 {
                let neighbor = Position { x: pos.x, y: ny as u8 };
                if let Some(n_cost) = look(neighbor.x, neighbor.y) {
                    if border_dx == 0 {
                        do_jump_neighbor(&mut heap, &mut parents, &mut open_closed, neighbor, g_cost, n_cost);
                    } else {
                        push_node(&mut heap, &mut parents, &mut open_closed, current_index, neighbor, g_cost + n_cost);
                    }
                }
            }
        }

        if dx != 0 {
            if dy != 0 {
                let nx = pos.x as i32 + dx;
                let ny = pos.y as i32 + dy;
                if nx >= 0 && nx < 100 && ny >= 0 && ny < 100 {
                    let neighbor = Position { x: nx as u8, y: ny as u8 };
                    if let Some(n_cost) = look(neighbor.x, neighbor.y) {
                        do_jump_neighbor(&mut heap, &mut parents, &mut open_closed, neighbor, g_cost, n_cost);
                    }
                }
                if look_at(pos.x as i32 - dx, pos.y as i32, &look) != Some(cost) {
                    let fx = pos.x as i32 - dx;
                    let fy = pos.y as i32 + dy;
                    if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                        let fn_pos = Position { x: fx as u8, y: fy as u8 };
                        if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                            do_jump_neighbor(&mut heap, &mut parents, &mut open_closed, fn_pos, g_cost, n_cost);
                        }
                    }
                }
                if look_at(pos.x as i32, pos.y as i32 - dy, &look) != Some(cost) {
                    let fx = pos.x as i32 + dx;
                    let fy = pos.y as i32 - dy;
                    if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                        let fn_pos = Position { x: fx as u8, y: fy as u8 };
                        if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                            do_jump_neighbor(&mut heap, &mut parents, &mut open_closed, fn_pos, g_cost, n_cost);
                        }
                    }
                }
            } else {
                if border_dy == 1 || look_at(pos.x as i32, pos.y as i32 + 1, &look) != Some(cost) {
                    let fx = pos.x as i32 + dx;
                    let fy = pos.y as i32 + 1;
                    if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                        let fn_pos = Position { x: fx as u8, y: fy as u8 };
                        if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                            do_jump_neighbor(&mut heap, &mut parents, &mut open_closed, fn_pos, g_cost, n_cost);
                        }
                    }
                }
                if border_dy == -1 || look_at(pos.x as i32, pos.y as i32 - 1, &look) != Some(cost) {
                    let fx = pos.x as i32 + dx;
                    let fy = pos.y as i32 - 1;
                    if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                        let fn_pos = Position { x: fx as u8, y: fy as u8 };
                        if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                            do_jump_neighbor(&mut heap, &mut parents, &mut open_closed, fn_pos, g_cost, n_cost);
                        }
                    }
                }
            }
        } else if dy != 0 {
            if border_dx == 1 || look_at(pos.x as i32 + 1, pos.y as i32, &look) != Some(cost) {
                let fx = pos.x as i32 + 1;
                let fy = pos.y as i32 + dy;
                if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                    let fn_pos = Position { x: fx as u8, y: fy as u8 };
                    if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                        do_jump_neighbor(&mut heap, &mut parents, &mut open_closed, fn_pos, g_cost, n_cost);
                    }
                }
            }
            if border_dx == -1 || look_at(pos.x as i32 - 1, pos.y as i32, &look) != Some(cost) {
                let fx = pos.x as i32 - 1;
                let fy = pos.y as i32 + dy;
                if fx >= 0 && fx < 100 && fy >= 0 && fy < 100 {
                    let fn_pos = Position { x: fx as u8, y: fy as u8 };
                    if let Some(n_cost) = look(fn_pos.x, fn_pos.y) {
                        do_jump_neighbor(&mut heap, &mut parents, &mut open_closed, fn_pos, g_cost, n_cost);
                    }
                }
            }
        }
    }

    let incomplete = min_node_h != 0;

    let Some(goal_node) = min_node else {
        return SearchResults { path: Vec::new(), ops, cost: 0, incomplete: true };
    };

    // pf.cc path reconstruction (lines 570-594)
    let mut path: Vec<Position> = Vec::new();
    let mut pos = goal_node;
    let mut index = pos_index(pos);
    while pos != start {
        path.push(pos);
        let parent_idx = parents[index];
        let parent = pos_from_index(parent_idx);
        if range_to(pos, parent) > 1 {
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
        index = parent_idx;
    }
    path.reverse();

    SearchResults {
        path,
        ops,
        cost: min_node_g,
        incomplete,
    }
}
