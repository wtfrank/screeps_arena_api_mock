use crate::traits::HasPosition;
use js_sys::{Array, Object};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl HasPosition for Position {
    fn pos(&self) -> Position {
        *self
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<Position> for Object {
    fn from(_pos: Position) -> Self {
        Object
    }
}

impl std::ops::Add<crate::constants::Direction> for Position {
    type Output = Position;
    fn add(self, dir: crate::constants::Direction) -> Position {
        let mut p = self;
        match dir {
            crate::constants::Direction::Top => {
                p.y = p.y.saturating_sub(1);
            }
            crate::constants::Direction::TopRight => {
                p.x = p.x.saturating_add(1);
                p.y = p.y.saturating_sub(1);
            }
            crate::constants::Direction::Right => {
                p.x = p.x.saturating_add(1);
            }
            crate::constants::Direction::BottomRight => {
                p.x = p.x.saturating_add(1);
                p.y = p.y.saturating_add(1);
            }
            crate::constants::Direction::Bottom => {
                p.y = p.y.saturating_add(1);
            }
            crate::constants::Direction::BottomLeft => {
                p.x = p.x.saturating_sub(1);
                p.y = p.y.saturating_add(1);
            }
            crate::constants::Direction::Left => {
                p.x = p.x.saturating_sub(1);
            }
            crate::constants::Direction::TopLeft => {
                p.x = p.x.saturating_sub(1);
                p.y = p.y.saturating_sub(1);
            }
        }
        p
    }
}

#[derive(Debug)]
pub struct SearchPathOptions {
    pub cost_matrix: std::cell::RefCell<Option<CostMatrix>>,
    pub max_ops: std::cell::Cell<Option<u32>>,
    pub heuristic_weight: std::cell::Cell<Option<f64>>,
    pub max_rooms: std::cell::Cell<Option<u32>>,
    pub plain_cost: std::cell::Cell<Option<u8>>,
    pub swamp_cost: std::cell::Cell<Option<u8>>,
    pub flee: std::cell::Cell<Option<bool>>,
}

impl SearchPathOptions {
    pub fn new() -> Self {
        SearchPathOptions {
            cost_matrix: std::cell::RefCell::new(None),
            max_ops: std::cell::Cell::new(None),
            heuristic_weight: std::cell::Cell::new(None),
            max_rooms: std::cell::Cell::new(None),
            plain_cost: std::cell::Cell::new(None),
            swamp_cost: std::cell::Cell::new(None),
            flee: std::cell::Cell::new(None),
        }
    }
    pub fn cost_matrix(&self, cm: &CostMatrix) {
        *self.cost_matrix.borrow_mut() = Some(cm.clone());
    }
    pub fn max_ops(&self, val: u32) {
        self.max_ops.set(Some(val));
    }
    pub fn heuristic_weight(&self, val: f64) {
        self.heuristic_weight.set(Some(val));
    }
    pub fn max_rooms(&self, val: u32) {
        self.max_rooms.set(Some(val));
    }
    pub fn plain_cost(&self, val: u8) {
        self.plain_cost.set(Some(val));
    }
    pub fn swamp_cost(&self, val: u8) {
        self.swamp_cost.set(Some(val));
    }
    pub fn flee(&self, val: bool) {
        self.flee.set(Some(val));
    }
    pub fn get_cost_matrix(&self) -> CostMatrix {
        self.cost_matrix.borrow().clone().unwrap_or_else(CostMatrix::new)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindPathOptions {
    pub cost_matrix: Option<CostMatrix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostMatrix {
    pub bits: Vec<u8>,
}

impl CostMatrix {
    pub fn new() -> Self {
        CostMatrix {
            bits: vec![0; 10000],
        }
    }
    pub fn set(&mut self, x: u8, y: u8, cost: u8) {
        if (x as usize) < 100 && (y as usize) < 100 {
            self.bits[(x as usize) * 100 + (y as usize)] = cost;
        }
    }
    pub fn get(&self, x: u8, y: u8) -> u8 {
        if (x as usize) < 100 && (y as usize) < 100 && !self.bits.is_empty() {
            self.bits[(x as usize) * 100 + (y as usize)]
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub path: Vec<Position>,
    pub ops: u32,
    pub cost: u32,
    pub incomplete: bool,
}

impl SearchResults {
    pub fn path(&self) -> Vec<Position> {
        self.path.clone()
    }
    pub fn incomplete(&self) -> bool {
        self.incomplete
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalSpec {
    pub pos: Position,
    pub range: u8,
}

pub fn search_path_astar(
    origin: &wasm_bindgen::JsValue,
    goal: &wasm_bindgen::JsValue,
    options: Option<&SearchPathOptions>,
) -> SearchResults {
    use std::collections::{BinaryHeap, HashMap, HashSet};
    use std::cmp::Ordering;
    use crate::traits::HasPosition;

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
            incomplete: true,
        };
    }

    let plain_cost = options.and_then(|o| o.plain_cost.get()).unwrap_or(2) as u32;
    let swamp_cost = options.and_then(|o| o.swamp_cost.get()).unwrap_or(10) as u32;
    let heuristic_weight = options.and_then(|o| o.heuristic_weight.get()).unwrap_or(1.2);
    let max_ops = options.and_then(|o| o.max_ops.get()).unwrap_or(50000);
    let flee = options.and_then(|o| o.flee.get()).unwrap_or(false);
    let custom_cm: Option<CostMatrix> = options.and_then(|o| o.cost_matrix.borrow().clone());

    let is_at_goal = |pos: Position| -> bool {
        for g in &goals {
            let dx = pos.x.abs_diff(g.pos.x);
            let dy = pos.y.abs_diff(g.pos.y);
            let range = dx.max(dy);
            if flee {
                if range >= g.range {
                    return true;
                }
            } else {
                if range <= g.range {
                    return true;
                }
            }
        }
        false
    };

    if is_at_goal(start) {
        return SearchResults {
            path: Vec::new(),
            ops: 0,
            cost: 0,
            incomplete: false,
        };
    }

    let heuristic = |pos: Position| -> f64 {
        let mut min_h = f64::MAX;
        for g in &goals {
            let dx = pos.x.abs_diff(g.pos.x) as f64;
            let dy = pos.y.abs_diff(g.pos.y) as f64;
            let dist = dx.max(dy);
            if dist < min_h {
                min_h = dist;
            }
        }
        min_h
    };

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: u32,
        estimated_total: u64,
        pos: Position,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.estimated_total.cmp(&self.estimated_total)
                .then_with(|| other.cost.cmp(&self.cost))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut open_set = BinaryHeap::new();
    let mut g_score: HashMap<Position, u32> = HashMap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    g_score.insert(start, 0);
    let start_h = heuristic(start);
    open_set.push(State {
        cost: 0,
        estimated_total: ((start_h * heuristic_weight) * 1000.0) as u64,
        pos: start,
    });

    let mut ops = 0;
    let mut best_target: Option<Position> = None;

    // Screeps directional iteration order: Top, TopRight, Right, BottomRight, Bottom, BottomLeft, Left, TopLeft
    let directions = [
        (0, -1),  // Top
        (1, -1),  // TopRight
        (1, 0),   // Right
        (1, 1),   // BottomRight
        (0, 1),   // Bottom
        (-1, 1),  // BottomLeft
        (-1, 0),  // Left
        (-1, -1), // TopLeft
    ];

    while let Some(State { cost, pos, .. }) = open_set.pop() {
        ops += 1;
        if ops > max_ops {
            break;
        }

        if is_at_goal(pos) {
            best_target = Some(pos);
            break;
        }

        if let Some(&recorded_g) = g_score.get(&pos) {
            if cost > recorded_g {
                continue;
            }
        }

        for (dx, dy) in directions {
            let nx = pos.x as i32 + dx;
            let ny = pos.y as i32 + dy;
            if nx < 0 || nx >= 100 || ny < 0 || ny >= 100 {
                continue;
            }

            // Screeps pf.cc rule: diagonal moves forbid cutting around wall corners
            if dx != 0 && dy != 0 {
                let orth1_blocked = match crate::game::utils::get_terrain_at_pos(pos.x, ny as u8) {
                    crate::constants::Terrain::Wall => true,
                    _ => custom_cm.as_ref().map(|cm| cm.get(pos.x, ny as u8) == 255).unwrap_or(false),
                };
                let orth2_blocked = match crate::game::utils::get_terrain_at_pos(nx as u8, pos.y) {
                    crate::constants::Terrain::Wall => true,
                    _ => custom_cm.as_ref().map(|cm| cm.get(nx as u8, pos.y) == 255).unwrap_or(false),
                };
                if orth1_blocked || orth2_blocked {
                    continue;
                }
            }

            let neighbor = Position { x: nx as u8, y: ny as u8 };

            // Tile cost calculation
            let tile_cost = if let Some(cm) = custom_cm.as_ref() {
                let custom_c = cm.get(neighbor.x, neighbor.y);
                if custom_c == 255 {
                    continue; // Impassable
                } else if custom_c > 0 {
                    custom_c as u32
                } else {
                    match crate::game::utils::get_terrain_at_pos(neighbor.x, neighbor.y) {
                        crate::constants::Terrain::Wall => continue,
                        crate::constants::Terrain::Swamp => swamp_cost,
                        _ => plain_cost,
                    }
                }
            } else {
                match crate::game::utils::get_terrain_at_pos(neighbor.x, neighbor.y) {
                    crate::constants::Terrain::Wall => continue,
                    crate::constants::Terrain::Swamp => swamp_cost,
                    _ => plain_cost,
                }
            };

            let tentative_g = cost + tile_cost;
            if tentative_g < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, pos);
                g_score.insert(neighbor, tentative_g);
                let h_val = heuristic(neighbor);
                let f_score = ((tentative_g as f64 + h_val * heuristic_weight) * 1000.0) as u64;
                open_set.push(State {
                    cost: tentative_g,
                    estimated_total: f_score,
                    pos: neighbor,
                });
            }
        }
    }

    if let Some(target) = best_target {
        let mut path = Vec::new();
        let mut curr = target;
        while curr != start {
            path.push(curr);
            if let Some(&prev) = came_from.get(&curr) {
                curr = prev;
            } else {
                break;
            }
        }
        path.reverse();
        let path_cost = *g_score.get(&target).unwrap_or(&0);
        SearchResults {
            path,
            ops,
            cost: path_cost,
            incomplete: false,
        }
    } else {
        let cm_start_cost = custom_cm.as_ref().map(|cm| cm.get(start.x, start.y)).unwrap_or(0);
        let goals_info: Vec<String> = goals.iter().map(|g| format!("({},{}) r:{}", g.pos.x, g.pos.y, g.range)).collect();
        log::debug!(
            "[search_path_astar] INCOMPLETE PATH: start=({},{}) start_cm_cost={} flee={} max_ops={} ops_used={} plain_cost={} swamp_cost={} goals=[{}]",
            start.x, start.y, cm_start_cost, flee, max_ops, ops, plain_cost, swamp_cost, goals_info.join(", ")
        );
        SearchResults {
            path: Vec::new(),
            ops,
            cost: 0,
            incomplete: true,
        }
    }
}

pub fn search_path_jps(
    origin: &wasm_bindgen::JsValue,
    goal: &wasm_bindgen::JsValue,
    options: Option<&SearchPathOptions>,
) -> SearchResults {
    // Jump Point Search (JPS) implementation scanning straight and diagonal rays
    use std::collections::{BinaryHeap, HashMap};
    use std::cmp::Ordering;
    use crate::traits::HasPosition;

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
            incomplete: true,
        };
    }

    let plain_cost = options.and_then(|o| o.plain_cost.get()).unwrap_or(2) as u32;
    let swamp_cost = options.and_then(|o| o.swamp_cost.get()).unwrap_or(10) as u32;
    let heuristic_weight = options.and_then(|o| o.heuristic_weight.get()).unwrap_or(1.2);
    let max_ops = options.and_then(|o| o.max_ops.get()).unwrap_or(50000);
    let flee = options.and_then(|o| o.flee.get()).unwrap_or(false);
    let custom_cm: Option<CostMatrix> = options.and_then(|o| o.cost_matrix.borrow().clone());

    let is_at_goal = |pos: Position| -> bool {
        for g in &goals {
            let dx = pos.x.abs_diff(g.pos.x);
            let dy = pos.y.abs_diff(g.pos.y);
            let range = dx.max(dy);
            if flee {
                if range >= g.range {
                    return true;
                }
            } else {
                if range <= g.range {
                    return true;
                }
            }
        }
        false
    };

    if is_at_goal(start) {
        return SearchResults {
            path: Vec::new(),
            ops: 0,
            cost: 0,
            incomplete: false,
        };
    }

    let heuristic = |pos: Position| -> f64 {
        let mut min_h = f64::MAX;
        for g in &goals {
            let dx = pos.x.abs_diff(g.pos.x) as f64;
            let dy = pos.y.abs_diff(g.pos.y) as f64;
            let dist = dx.max(dy);
            if dist < min_h {
                min_h = dist;
            }
        }
        min_h
    };

    let get_cost = |x: u8, y: u8| -> Option<u32> {
        if let Some(cm) = custom_cm.as_ref() {
            let c = cm.get(x, y);
            if c == 255 {
                None
            } else if c > 0 {
                Some(c as u32)
            } else {
                match crate::game::utils::get_terrain_at_pos(x, y) {
                    crate::constants::Terrain::Wall => None,
                    crate::constants::Terrain::Swamp => Some(swamp_cost),
                    _ => Some(plain_cost),
                }
            }
        } else {
            match crate::game::utils::get_terrain_at_pos(x, y) {
                crate::constants::Terrain::Wall => None,
                crate::constants::Terrain::Swamp => Some(swamp_cost),
                _ => Some(plain_cost),
            }
        }
    };

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: u32,
        estimated_total: u64,
        pos: Position,
        dir: (i32, i32),
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.estimated_total.cmp(&self.estimated_total)
                .then_with(|| other.cost.cmp(&self.cost))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut open_set = BinaryHeap::new();
    let mut g_score: HashMap<Position, u32> = HashMap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    g_score.insert(start, 0);

    let target_goal = goals.first().map(|g| g.pos).unwrap_or(start);
    let _goal_dx = (target_goal.x as i32 - start.x as i32).signum();
    let _goal_dy = (target_goal.y as i32 - start.y as i32).signum();

    // Screeps native pf.cc 8-direction iteration order
    let initial_dirs = [
        (0, -1),  // Top
        (1, -1),  // TopRight
        (1, 0),   // Right
        (1, 1),   // BottomRight
        (0, 1),   // Bottom
        (-1, 1),  // BottomLeft
        (-1, 0),  // Left
        (-1, -1), // TopLeft
    ];

    for &dir in &initial_dirs {
        open_set.push(State {
            cost: 0,
            estimated_total: ((heuristic(start) * heuristic_weight) * 1000.0) as u64,
            pos: start,
            dir,
        });
    }

    let mut ops = 0;
    let mut best_target: Option<Position> = None;

    // Jump helper function with corner-cutting prevention (pf.cc compliance)
    let jump = |mut curr_x: i32, mut curr_y: i32, dx: i32, dy: i32, current_g: u32| -> Option<(Position, u32)> {
        let mut accumulated_g = current_g;
        loop {
            let nx = curr_x + dx;
            let ny = curr_y + dy;
            if nx < 0 || nx >= 100 || ny < 0 || ny >= 100 {
                return None;
            }

            // Screeps pf.cc rule: diagonal moves forbid cutting around wall corners
            if dx != 0 && dy != 0 {
                let orth1_blocked = get_cost(curr_x as u8, ny as u8).is_none();
                let orth2_blocked = get_cost(nx as u8, curr_y as u8).is_none();
                if orth1_blocked || orth2_blocked {
                    return None;
                }
            }

            let cost = get_cost(nx as u8, ny as u8)?;
            accumulated_g += cost;
            let next_pos = Position { x: nx as u8, y: ny as u8 };

            if is_at_goal(next_pos) {
                return Some((next_pos, accumulated_g));
            }

            // Screeps pf.cc rule: diagonal jump stops at target column alignment (nx == target_goal.x)
            if dx != 0 && dy != 0 && nx == target_goal.x as i32 {
                return Some((next_pos, accumulated_g));
            }

            // Check for forced neighbors
            if dx != 0 && dy != 0 {
                // Diagonal jump checks
                let left_blocked = get_cost((nx - dx) as u8, ny as u8).is_none();
                let left_open = get_cost((nx - dx) as u8, (ny + dy) as u8).is_some();
                let right_blocked = get_cost(nx as u8, (ny - dy) as u8).is_none();
                let right_open = get_cost((nx + dx) as u8, (ny - dy) as u8).is_some();

                if (left_blocked && left_open) || (right_blocked && right_open) {
                    return Some((next_pos, accumulated_g));
                }
            } else if dx != 0 {
                // Horizontal jump checks
                let up_blocked = get_cost(nx as u8, (ny - 1) as u8).is_none();
                let up_open = get_cost((nx + dx) as u8, (ny - 1) as u8).is_some();
                let down_blocked = get_cost(nx as u8, (ny + 1) as u8).is_none();
                let down_open = get_cost((nx + dx) as u8, (ny + 1) as u8).is_some();

                if (up_blocked && up_open) || (down_blocked && down_open) {
                    return Some((next_pos, accumulated_g));
                }
            } else {
                // Vertical jump checks
                let left_blocked = get_cost((nx - 1) as u8, ny as u8).is_none();
                let left_open = get_cost((nx - 1) as u8, (ny + dy) as u8).is_some();
                let right_blocked = get_cost((nx + 1) as u8, ny as u8).is_none();
                let right_open = get_cost((nx + 1) as u8, (ny + dy) as u8).is_some();

                if (left_blocked && left_open) || (right_blocked && right_open) {
                    return Some((next_pos, accumulated_g));
                }
            }

            curr_x = nx;
            curr_y = ny;
        }
    };

    while let Some(State { cost, pos, dir, .. }) = open_set.pop() {
        ops += 1;
        if ops > max_ops {
            break;
        }

        if is_at_goal(pos) {
            best_target = Some(pos);
            break;
        }

        if let Some(&jump_res) = jump(pos.x as i32, pos.y as i32, dir.0, dir.1, cost).as_ref() {
            let (jump_pos, jump_g) = jump_res;
            if jump_g < *g_score.get(&jump_pos).unwrap_or(&u32::MAX) {
                came_from.insert(jump_pos, pos);
                g_score.insert(jump_pos, jump_g);
                let h_val = heuristic(jump_pos);
                let f_score = ((jump_g as f64 + h_val * heuristic_weight) * 1000.0) as u64;

                // In JPS, pass only valid successor directions for next node, clamping vectors toward target goal
                let cur_goal_dx = (target_goal.x as i32 - jump_pos.x as i32).signum();
                let cur_goal_dy = (target_goal.y as i32 - jump_pos.y as i32).signum();

                let raw_dirs = if dir.0 != 0 && dir.1 != 0 {
                    vec![(dir.0, dir.1), (dir.0, 0), (0, dir.1), (cur_goal_dx, cur_goal_dy), (0, cur_goal_dy), (cur_goal_dx, 0)]
                } else if dir.0 != 0 {
                    vec![(dir.0, 0), (dir.0, cur_goal_dy), (cur_goal_dx, cur_goal_dy)]
                } else {
                    vec![(0, dir.1), (cur_goal_dx, dir.1), (cur_goal_dx, cur_goal_dy)]
                };

                let mut successor_dirs = Vec::new();
                for (mut d_x, mut d_y) in raw_dirs {
                    if jump_pos.x == target_goal.x { d_x = 0; }
                    if jump_pos.y == target_goal.y { d_y = 0; }
                    if d_x != 0 || d_y != 0 {
                        if !successor_dirs.contains(&(d_x, d_y)) {
                            successor_dirs.push((d_x, d_y));
                        }
                    }
                }

                for &next_dir in &successor_dirs {
                    open_set.push(State {
                        cost: jump_g,
                        estimated_total: f_score,
                        pos: jump_pos,
                        dir: next_dir,
                    });
                }
            }
        }
    }

    if let Some(target) = best_target {
        let mut jump_nodes = Vec::new();
        let mut curr = target;
        while curr != start {
            jump_nodes.push(curr);
            if let Some(&prev) = came_from.get(&curr) {
                curr = prev;
            } else {
                break;
            }
        }
        jump_nodes.push(start);
        jump_nodes.reverse();

        // Interpolate tile-by-tile between jump nodes
        let mut full_path = Vec::new();
        for i in 0..jump_nodes.len() - 1 {
            let p1 = jump_nodes[i];
            let p2 = jump_nodes[i + 1];

            let mut step_x = p1.x as i32;
            let mut step_y = p1.y as i32;
            let target_x = p2.x as i32;
            let target_y = p2.y as i32;

            let dx = (target_x - step_x).signum();
            let dy = (target_y - step_y).signum();

            while step_x != target_x || step_y != target_y {
                step_x += dx;
                step_y += dy;
                full_path.push(Position {
                    x: step_x as u8,
                    y: step_y as u8,
                });
            }
        }

        let path_cost = *g_score.get(&target).unwrap_or(&0);
        SearchResults {
            path: full_path,
            ops,
            cost: path_cost,
            incomplete: false,
        }
    } else {
        // Fallback to A* search if JPS didn't yield full path
        search_path_astar(origin, goal, options)
    }
}

pub fn search_path_hybrid(
    origin: &wasm_bindgen::JsValue,
    goal: &wasm_bindgen::JsValue,
    options: Option<&SearchPathOptions>,
) -> SearchResults {
    use std::collections::{BinaryHeap, HashMap, HashSet};
    use std::cmp::Ordering;
    use crate::traits::HasPosition;

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
            incomplete: true,
        };
    }

    let plain_cost = options.and_then(|o| o.plain_cost.get()).unwrap_or(2) as u32;
    let swamp_cost = options.and_then(|o| o.swamp_cost.get()).unwrap_or(10) as u32;
    let heuristic_weight = options.and_then(|o| o.heuristic_weight.get()).unwrap_or(1.2);
    let max_ops = options.and_then(|o| o.max_ops.get()).unwrap_or(50000);
    let flee = options.and_then(|o| o.flee.get()).unwrap_or(false);
    let custom_cm: Option<CostMatrix> = options.and_then(|o| o.cost_matrix.borrow().clone());

    let is_at_goal = |pos: Position| -> bool {
        for g in &goals {
            let dx = pos.x.abs_diff(g.pos.x);
            let dy = pos.y.abs_diff(g.pos.y);
            let range = dx.max(dy);
            if flee {
                if range >= g.range {
                    return true;
                }
            } else {
                if range <= g.range {
                    return true;
                }
            }
        }
        false
    };

    if is_at_goal(start) {
        return SearchResults {
            path: Vec::new(),
            ops: 0,
            cost: 0,
            incomplete: false,
        };
    }

    let heuristic = |pos: Position| -> f64 {
        let mut min_h = f64::MAX;
        for g in &goals {
            let dx = pos.x.abs_diff(g.pos.x) as f64;
            let dy = pos.y.abs_diff(g.pos.y) as f64;
            let dist = dx.max(dy);
            if dist < min_h {
                min_h = dist;
            }
        }
        min_h
    };

    let get_cost = |x: u8, y: u8| -> Option<u32> {
        if let Some(cm) = custom_cm.as_ref() {
            let c = cm.get(x, y);
            if c == 255 {
                None
            } else if c > 0 {
                Some(c as u32)
            } else {
                match crate::game::utils::get_terrain_at_pos(x, y) {
                    crate::constants::Terrain::Wall => None,
                    crate::constants::Terrain::Swamp => Some(swamp_cost),
                    _ => Some(plain_cost),
                }
            }
        } else {
            match crate::game::utils::get_terrain_at_pos(x, y) {
                crate::constants::Terrain::Wall => None,
                crate::constants::Terrain::Swamp => Some(swamp_cost),
                _ => Some(plain_cost),
            }
        }
    };

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: u32,
        estimated_total: u64,
        pos: Position,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.estimated_total.cmp(&self.estimated_total)
                .then_with(|| other.cost.cmp(&self.cost))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut open_set = BinaryHeap::new();
    let mut g_score: HashMap<Position, u32> = HashMap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut closed_set: HashSet<Position> = HashSet::new();

    g_score.insert(start, 0);
    let start_h = heuristic(start);
    open_set.push(State {
        cost: 0,
        estimated_total: ((start_h * heuristic_weight) * 1000.0) as u64,
        pos: start,
    });

    let directions = [
        (0, -1),  // Top
        (1, -1),  // TopRight
        (1, 0),   // Right
        (1, 1),   // BottomRight
        (0, 1),   // Bottom
        (-1, 1),  // BottomLeft
        (-1, 0),  // Left
        (-1, -1), // TopLeft
    ];

    let mut ops = 0;
    let mut best_target: Option<Position> = None;

    while let Some(State { cost, pos, .. }) = open_set.pop() {
        if closed_set.contains(&pos) {
            continue;
        }
        closed_set.insert(pos);
        ops += 1;
        if ops > max_ops {
            break;
        }

        if is_at_goal(pos) {
            best_target = Some(pos);
            break;
        }

        let parent_opt = came_from.get(&pos).copied();
        let search_dirs: Vec<(i32, i32)> = if let Some(parent) = parent_opt {
            let dx = (pos.x as i32 - parent.x as i32).signum();
            let dy = (pos.y as i32 - parent.y as i32).signum();
            if dx != 0 && dy != 0 {
                vec![(dx, dy), (dx, 0), (0, dy), (-dx, dy), (dx, -dy)]
            } else if dx != 0 {
                vec![(dx, 0), (dx, -1), (dx, 1)]
            } else {
                vec![(0, dy), (-1, dy), (1, dy)]
            }
        } else {
            directions.to_vec()
        };

        for &(dx, dy) in &search_dirs {
            let mut curr_x = pos.x as i32;
            let mut curr_y = pos.y as i32;
            let mut jump_g = cost;

            loop {
                let nx = curr_x + dx;
                let ny = curr_y + dy;
                if nx < 0 || nx >= 100 || ny < 0 || ny >= 100 {
                    break;
                }

                // Diagonal corner cutting restriction (pf.cc compliance)
                if dx != 0 && dy != 0 {
                    let orth1_blocked = get_cost(curr_x as u8, ny as u8).is_none();
                    let orth2_blocked = get_cost(nx as u8, curr_y as u8).is_none();
                    if orth1_blocked || orth2_blocked {
                        break;
                    }
                }

                let tile_cost = match get_cost(nx as u8, ny as u8) {
                    Some(c) => c,
                    None => break,
                };
                jump_g += tile_cost;
                let neighbor = Position { x: nx as u8, y: ny as u8 };

                if closed_set.contains(&neighbor) {
                    curr_x = nx;
                    curr_y = ny;
                    continue;
                }

                // Forced neighbor rule from pf.cc: cost transitions (get_cost != Some(tile_cost))
                let is_jump_node = is_at_goal(neighbor) || if dx != 0 && dy != 0 {
                    (get_cost((nx - dx) as u8, ny as u8) != Some(tile_cost) && get_cost((nx - dx) as u8, (ny + dy) as u8).is_some()) ||
                    (get_cost(nx as u8, (ny - dy) as u8) != Some(tile_cost) && get_cost((nx + dx) as u8, (ny - dy) as u8).is_some())
                } else if dx != 0 {
                    (get_cost(nx as u8, (ny - 1) as u8) != Some(tile_cost) && get_cost((nx + dx) as u8, (ny - 1) as u8).is_some()) ||
                    (get_cost(nx as u8, (ny + 1) as u8) != Some(tile_cost) && get_cost((nx + dx) as u8, (ny + 1) as u8).is_some())
                } else {
                    (get_cost((nx - 1) as u8, ny as u8) != Some(tile_cost) && get_cost((nx - 1) as u8, (ny + dy) as u8).is_some()) ||
                    (get_cost((nx + 1) as u8, ny as u8) != Some(tile_cost) && get_cost((nx + 1) as u8, (ny + dy) as u8).is_some())
                };

                if is_jump_node {
                    if jump_g < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                        came_from.insert(neighbor, pos);
                        g_score.insert(neighbor, jump_g);
                        let h_val = heuristic(neighbor);
                        let f_score = ((jump_g as f64 + h_val * heuristic_weight) * 1000.0) as u64;
                        open_set.push(State {
                            cost: jump_g,
                            estimated_total: f_score,
                            pos: neighbor,
                        });
                    }
                    break;
                }

                curr_x = nx;
                curr_y = ny;
            }
        }
    }

    if let Some(target) = best_target {
        let mut raw_path = Vec::new();
        let mut pos = target;
        
        while pos != start {
            raw_path.push(pos);
            if let Some(&next) = came_from.get(&pos) {
                let dx = (next.x as i32 - pos.x as i32).signum();
                let dy = (next.y as i32 - pos.y as i32).signum();
                
                let mut step_x = pos.x as i32;
                let mut step_y = pos.y as i32;
                while step_x + dx != next.x as i32 || step_y + dy != next.y as i32 {
                    step_x += dx;
                    step_y += dy;
                    raw_path.push(Position {
                        x: step_x as u8,
                        y: step_y as u8,
                    });
                }
                pos = next;
            } else {
                break;
            }
        }
        raw_path.reverse();

        let path_cost = *g_score.get(&target).unwrap_or(&0);
        SearchResults {
            path: raw_path,
            ops,
            cost: path_cost,
            incomplete: false,
        }
    } else {
        SearchResults {
            path: Vec::new(),
            ops,
            cost: 0,
            incomplete: true,
        }
    }
}

pub fn search_path(
    origin: &wasm_bindgen::JsValue,
    goal: &wasm_bindgen::JsValue,
    options: Option<&SearchPathOptions>,
) -> SearchResults {
    use crate::traits::HasPosition;

    let start = if let Ok(pos) = serde_wasm_bindgen::from_value::<Position>(origin.clone()) {
        pos
    } else {
        unsafe {
            let go = &*(origin as *const wasm_bindgen::JsValue as *const crate::objects::GameObject);
            go.pos()
        }
    };

    // Bounds check matching game.path-finder.search-path.js: origin out of bounds returns empty search result
    if start.x > 99 || start.y > 99 {
        return SearchResults {
            path: Vec::new(),
            ops: 0,
            cost: 0,
            incomplete: false,
        };
    }

    // Default & option clamping matching game.path-finder.search-path.js:
    // plainCost: min(254, max(1, plain_cost || 2))
    // swampCost: min(254, max(1, swamp_cost || 10))
    // heuristicWeight: min(9.0, max(1.0, heuristic_weight || 1.2))
    // maxOps: max(1, max_ops || 10000)
    let plain_cost = options
        .and_then(|o| o.plain_cost.get())
        .map(|c| c.clamp(1, 254))
        .unwrap_or(2) as u32;

    let swamp_cost = options
        .and_then(|o| o.swamp_cost.get())
        .map(|c| c.clamp(1, 254))
        .unwrap_or(10) as u32;

    let heuristic_weight = options
        .and_then(|o| o.heuristic_weight.get())
        .map(|w| w.clamp(1.0, 9.0))
        .unwrap_or(1.2);

    let max_ops = options
        .and_then(|o| o.max_ops.get())
        .map(|o| o.max(1))
        .unwrap_or(10000);

    let flee = options.and_then(|o| o.flee.get()).unwrap_or(false);
    let custom_cm: Option<CostMatrix> = options.and_then(|o| o.cost_matrix.borrow().clone());

    // Normalize one-or-many goals into Vec<GoalSpec>
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

    // Call underlying C++ / pf_cc search implementation
    crate::pf_cc::search(
        start,
        &goals,
        plain_cost,
        swamp_cost,
        heuristic_weight,
        max_ops,
        flee,
        custom_cm.as_ref(),
    )
}
