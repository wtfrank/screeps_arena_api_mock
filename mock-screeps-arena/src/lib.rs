pub mod ffi;
pub mod pf_cc;
pub mod pf_cc_heap;

pub mod constants {
    use serde::{Deserialize, Serialize};

    pub const SPAWN_RANGE: u32 = 20;
    pub const DEFAULT_SPAWN_DIRECTIONS: [Direction; 8] = [
        Direction::Top,
        Direction::TopRight,
        Direction::Right,
        Direction::BottomRight,
        Direction::Bottom,
        Direction::BottomLeft,
        Direction::Left,
        Direction::TopLeft,
    ];

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Direction {
        Top = 1,
        TopRight = 2,
        Right = 3,
        BottomRight = 4,
        Bottom = 5,
        BottomLeft = 6,
        Left = 7,
        TopLeft = 8,
    }

    impl Direction {
        pub fn from_u8(val: u8) -> Option<Direction> {
            match val {
                1 => Some(Direction::Top),
                2 => Some(Direction::TopRight),
                3 => Some(Direction::Right),
                4 => Some(Direction::BottomRight),
                5 => Some(Direction::Bottom),
                6 => Some(Direction::BottomLeft),
                7 => Some(Direction::Left),
                8 => Some(Direction::TopLeft),
                _ => None,
            }
        }

        pub fn multi_rot(&self, steps: i32) -> Direction {
            let val = *self as i32;
            let new_val = (((val - 1 + steps) % 8 + 8) % 8) + 1;
            match new_val {
                1 => Direction::Top,
                2 => Direction::TopRight,
                3 => Direction::Right,
                4 => Direction::BottomRight,
                5 => Direction::Bottom,
                6 => Direction::BottomLeft,
                7 => Direction::Left,
                8 => Direction::TopLeft,
                _ => unreachable!(),
            }
        }
    }

    impl std::fmt::Display for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl std::ops::Neg for Direction {
        type Output = Direction;
        fn neg(self) -> Direction {
            match self {
                Direction::Top => Direction::Bottom,
                Direction::TopRight => Direction::BottomLeft,
                Direction::Right => Direction::Left,
                Direction::BottomRight => Direction::TopLeft,
                Direction::Bottom => Direction::Top,
                Direction::BottomLeft => Direction::TopRight,
                Direction::Left => Direction::Right,
                Direction::TopLeft => Direction::BottomRight,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Part {
        Move = 1,
        Work = 2,
        Carry = 3,
        Attack = 4,
        RangedAttack = 5,
        Tough = 6,
        Heal = 7,
    }

    impl Part {
        pub fn cost(&self) -> u32 {
            match self {
                Part::Move => 50,
                Part::Work => 100,
                Part::Carry => 50,
                Part::Attack => 80,
                Part::RangedAttack => 150,
                Part::Tough => 10,
                Part::Heal => 250,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum ReturnCode {
        Ok = 0,
        NotOwner = -1,
        NoPath = -2,
        NameExists = -3,
        Busy = -4,
        NotFound = -5,
        NotEnough = -6,
        InvalidTarget = -7,
        Full = -8,
        NotInRange = -9,
        InvalidArgs = -10,
        Tired = -11,
        NoBodypart = -12,
        Error = -13,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Terrain {
        Plain = 0,
        Wall = 1,
        Swamp = 2,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum ResourceType {
        Energy = 1,
        Score = 2,
        ScoreX = 3,
        ScoreY = 4,
        ScoreZ = 5,
    }

    pub const ATTACK_POWER: u32 = 30;
    pub const HEAL_POWER: u32 = 12;
    pub const RANGED_HEAL_POWER: u32 = 4;
    pub const RANGED_ATTACK_POWER: u32 = 10;
    pub const CARRY_CAPACITY: u32 = 50;
    pub const DISMANTLE_POWER: u32 = 50;

    pub const TOWER_FALLOFF: f32 = 0.7;
    pub const TOWER_FALLOFF_RANGE: u32 = 20;
    pub const TOWER_OPTIMAL_RANGE: u32 = 5;
    pub const TOWER_POWER_ATTACK: u32 = 150;
    pub const TOWER_POWER_HEAL: u32 = 100;
    pub const TOWER_RANGE: u32 = 50;

    pub use self::extra::{ROOM_HEIGHT, ROOM_WIDTH};
    pub use crate::prototypes;

    pub mod numbers {
        pub use super::{
            ATTACK_POWER, CARRY_CAPACITY, DISMANTLE_POWER, HEAL_POWER, RANGED_ATTACK_POWER,
        };
    }

    pub mod extra {
        pub const ROOM_HEIGHT: u32 = 100;
        pub const ROOM_WIDTH: u32 = 100;
        pub const CONSTRUCTION_SITE_STOMP_RATIO: f32 = 0.5;
        pub const CREEP_HITS_PER_PART: u32 = 100;
        pub const MOVE_POWER: u32 = 2;
        pub const RANGED_MASS_ATTACK_POWER_RANGE_1: u32 = 10;
        pub const RANGED_MASS_ATTACK_POWER_RANGE_2: u32 = 4;
        pub const RANGED_MASS_ATTACK_POWER_RANGE_3: u32 = 1;
    }
}

pub mod enums {
    pub use crate::constants::{Direction, Part, ResourceType, ReturnCode, Terrain};
}

pub mod traits {
    use crate::game::pathfinder::Position;
    use crate::objects::{GameObject, Store};
    use js_sys::{Array, JsString, Object};
    use wasm_bindgen::JsCast;

    pub trait HasPosition {
        fn pos(&self) -> Position;
        fn range_to(&self, has_pos: &impl HasPosition) -> u8 {
            let other = has_pos.pos();
            std::cmp::max(
                self.pos().x.abs_diff(other.x),
                self.pos().y.abs_diff(other.y),
            )
        }
    }

    pub trait HasHits {
        fn hits(&self) -> u32;
        fn hits_max(&self) -> u32;
    }

    pub trait HasCooldown {
        fn cooldown(&self) -> u32;
    }

    pub trait HasStore {
        fn store(&self) -> Store;
    }

    pub trait GameObjectProperties {
        fn exists(&self) -> bool;
        fn id(&self) -> JsString;
        fn x(&self) -> u8;
        fn y(&self) -> u8;
        fn ticks_to_decay(&self) -> Option<u32>;
        fn find_path_to(
            &self,
            _pos: &Object,
            _options: Option<&crate::game::pathfinder::FindPathOptions>,
        ) -> Array {
            Array
        }
        fn find_in_range<T>(&self, _positions: &[T], _range: u8) -> Vec<T>
        where
            T: HasPosition + JsCast,
        {
            Vec::new()
        }
        fn find_closest_by_range<T>(&self, _positions: &[T]) -> Option<T>
        where
            T: HasPosition + JsCast,
        {
            None
        }
        fn find_closest_by_path<T>(
            &self,
            _positions: &[T],
            _options: Option<&crate::game::pathfinder::FindPathOptions>,
        ) -> Option<T>
        where
            T: HasPosition + JsCast,
        {
            None
        }
        fn get_range_to(&self, _pos: &Object) -> u8 {
            0
        }
    }

    pub trait OwnedStructureProperties {
        fn my(&self) -> Option<bool>;
    }

    pub trait Transferable: AsRef<GameObject> {}
    pub trait Withdrawable: AsRef<GameObject> {}
    pub trait Attackable: HasHits + AsRef<GameObject> {}
}

pub mod game {
    pub use self::utils::{
        arena_info, create_construction_site, get_cpu_time, get_heap_statistics, get_object_by_id,
        get_objects, get_objects_by_prototype, get_terrain_at, get_ticks,
    };

    pub mod visual {
        use js_sys::Object;

        #[derive(Debug, Clone, Default)]
        pub struct TextStyle {
            pub align: Option<TextAlign>,
        }

        impl TextStyle {
            pub fn align(mut self, align: TextAlign) -> Self {
                self.align = Some(align);
                self
            }
            pub fn color(mut self, _color: &str) -> Self {
                self
            }
            pub fn opacity(mut self, _opacity: f64) -> Self {
                self
            }
            pub fn font_size(mut self, _font_size: f64) -> Self {
                self
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub enum TextAlign {
            Left,
            Center,
            Right,
        }

        #[derive(Debug, Clone)]
        pub struct Visual;

        impl Visual {
            pub fn new(_layer: Option<u8>, _persistent: bool) -> Self {
                Visual
            }
            pub fn text(
                &self,
                _text: &str,
                _pos: &VisualPosition,
                _style: Option<&TextStyle>,
            ) -> &Self {
                self
            }
            pub fn line(
                &self,
                _from: &VisualPosition,
                _to: &VisualPosition,
                _style: Option<&Object>,
            ) -> &Self {
                self
            }
            pub fn circle(&self, _pos: &VisualPosition, _style: Option<&Object>) -> &Self {
                self
            }
            pub fn rect(
                &self,
                _pos: &VisualPosition,
                _w: f64,
                _h: f64,
                _style: Option<&Object>,
            ) -> &Self {
                self
            }
            pub fn poly(&self, _points: &[VisualPosition], _style: Option<&Object>) -> &Self {
                self
            }
        }

        #[derive(Debug, Clone)]
        pub struct VisualPosition {
            pub x: f32,
            pub y: f32,
        }

        impl VisualPosition {
            pub fn offset(mut self, dx: f32, dy: f32) -> Self {
                self.x += dx;
                self.y += dy;
                self
            }
        }

        impl From<crate::game::pathfinder::Position> for VisualPosition {
            fn from(pos: crate::game::pathfinder::Position) -> Self {
                VisualPosition {
                    x: pos.x as f32,
                    y: pos.y as f32,
                }
            }
        }
    }

    pub mod utils {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ArenaInfo {
            pub cpu_time_limit: u32,
            pub cpu_time_limit_ticks: u32,
            pub cpu_time_limit_first_tick: u32,
            pub tick_limit: u32,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct HeapStatistics {
            pub total_heap_size: u32,
            pub total_heap_size_executable: u32,
            pub total_physical_size: u32,
            pub total_available_size: u32,
            pub used_heap_size: u32,
            pub heap_size_limit: u32,
            pub malloced_memory: u32,
            pub peak_malloced_memory: u32,
            pub does_zap_garbage: u32,
            pub number_of_native_contexts: u32,
            pub number_of_detached_contexts: u32,
        }

        impl HeapStatistics {
            pub fn used_heap_size(&self) -> u32 {
                self.used_heap_size
            }
        }

        pub fn get_ticks() -> u32 {
            crate::ffi::with_host_interface(|iface| (iface.get_ticks)()).unwrap_or(1)
        }
        pub fn get_cpu_time() -> u32 {
            crate::ffi::with_host_interface(|iface| (iface.get_cpu_time)()).unwrap_or(0)
        }
        pub fn get_heap_statistics() -> HeapStatistics {
            HeapStatistics {
                total_heap_size: 0,
                total_heap_size_executable: 0,
                total_physical_size: 0,
                total_available_size: 0,
                used_heap_size: 0,
                heap_size_limit: 0,
                malloced_memory: 0,
                peak_malloced_memory: 0,
                does_zap_garbage: 0,
                number_of_native_contexts: 0,
                number_of_detached_contexts: 0,
            }
        }
        pub fn arena_info() -> ArenaInfo {
            ArenaInfo {
                cpu_time_limit: 50000000,
                cpu_time_limit_ticks: 50000000,
                cpu_time_limit_first_tick: 50000000,
                tick_limit: 2000,
            }
        }
        pub fn get_objects() -> Vec<crate::objects::GameObject> {
            let mut all = Vec::new();
            all.extend(
                get_objects_by_prototype(crate::prototypes::CREEP)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::STRUCTURE_SPAWN)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::STRUCTURE_TOWER)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::STRUCTURE_EXTENSION)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::STRUCTURE_RAMPART)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::STRUCTURE_CONTAINER)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::STRUCTURE_ROAD)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::STRUCTURE_WALL)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::RESOURCE)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::SOURCE)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::FLAG)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::SCORE_COLLECTOR)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::BONUS_FLAG)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::AREA_EFFECT)
                    .into_iter()
                    .map(|o| o.base),
            );
            all.extend(
                get_objects_by_prototype(crate::prototypes::CONSTRUCTION_SITE)
                    .into_iter()
                    .map(|o| o.base),
            );
            all
        }

        pub fn get_object_by_id<T>(id: &str, prototype: T) -> Option<T::Item>
        where
            T: crate::prototypes::PrototypeConstant,
            T::Item: Clone + AsRef<crate::objects::GameObject>,
        {
            let items = get_objects_by_prototype(prototype);
            items.into_iter().find(|item| item.as_ref().id == id)
        }

        pub fn get_objects_by_prototype<T>(_prototype: T) -> Vec<T::Item>
        where
            T: crate::prototypes::PrototypeConstant,
            T::Item: Clone,
        {
            crate::ffi::with_host_interface(|iface| {
                let mut ptr: *const std::ffi::c_void = std::ptr::null();
                let mut len: usize = 0;
                (iface.get_objects)(T::ID, &mut ptr, &mut len);
                if !ptr.is_null() && len > 0 {
                    let slice = unsafe { std::slice::from_raw_parts(ptr as *const T::Item, len) };
                    slice.to_vec()
                } else {
                    Vec::new()
                }
            })
            .unwrap_or_default()
        }

        pub fn get_terrain_at_pos(x: u8, y: u8) -> crate::constants::Terrain {
            crate::ffi::with_host_interface(|iface| {
                let code = (iface.get_terrain_at)(x, y);
                match code {
                    1 => crate::constants::Terrain::Wall,
                    2 => crate::constants::Terrain::Swamp,
                    _ => crate::constants::Terrain::Plain,
                }
            })
            .unwrap_or(crate::constants::Terrain::Plain)
        }

        pub fn get_terrain_at(pos: &wasm_bindgen::JsValue) -> crate::constants::Terrain {
            let (x, y) = if let Ok(p) = serde_wasm_bindgen::from_value::<crate::game::pathfinder::Position>(pos.clone()) {
                (p.x, p.y)
            } else {
                unsafe {
                    let ptr = pos as *const wasm_bindgen::JsValue as *const crate::game::pathfinder::Position;
                    ((*ptr).x, (*ptr).y)
                }
            };
            get_terrain_at_pos(x, y)
        }

        pub fn create_construction_site(
            _x: u8,
            _y: u8,
            _structure_type: &js_sys::Object,
        ) -> Result<crate::objects::ConstructionSite, crate::constants::ReturnCode> {
            Err(crate::constants::ReturnCode::Error)
        }
    }

    pub mod pathfinder {
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
                    self.bits[(y as usize) * 100 + (x as usize)] = cost;
                }
            }
            pub fn get(&self, x: u8, y: u8) -> u8 {
                if (x as usize) < 100 && (y as usize) < 100 && !self.bits.is_empty() {
                    self.bits[(y as usize) * 100 + (x as usize)]
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
            crate::pf_cc::search_path_pf_cc(origin, goal, options)
        }
    }
}

pub mod objects {
    use crate::constants::{Direction, Part, ResourceType, ReturnCode};
    use crate::game::pathfinder::Position;
    use crate::traits::*;
    use js_sys::{JsString, Object};

    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct GameObject {
        pub id: String,
        pub x: u8,
        pub y: u8,
    }

    impl GameObject {
        pub fn exists(&self) -> bool {
            true
        }
        pub fn id(&self) -> JsString {
            JsString(self.id.clone())
        }
        pub fn x(&self) -> u8 {
            self.x
        }
        pub fn y(&self) -> u8 {
            self.y
        }
        pub fn ticks_to_decay(&self) -> Option<u32> {
            None
        }
    }

    impl AsRef<wasm_bindgen::JsValue> for GameObject {
        fn as_ref(&self) -> &wasm_bindgen::JsValue {
            // Note: mock JsValue wrap
            let _ = self;
            unsafe { &*(self as *const GameObject as *const wasm_bindgen::JsValue) }
        }
    }

    impl HasPosition for GameObject {
        fn pos(&self) -> Position {
            Position {
                x: self.x,
                y: self.y,
            }
        }
    }

    impl GameObjectProperties for GameObject {
        fn exists(&self) -> bool {
            true
        }
        fn id(&self) -> JsString {
            JsString(self.id.clone())
        }
        fn x(&self) -> u8 {
            self.x
        }
        fn y(&self) -> u8 {
            self.y
        }
        fn ticks_to_decay(&self) -> Option<u32> {
            None
        }
    }

    macro_rules! define_mock_struct {
        ($name:ident, { $($field_name:ident : $field_type:ty),* $(,)? }) => {
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
            pub struct $name {
                pub base: GameObject,
                $( pub $field_name : $field_type, )*
            }
            impl HasPosition for $name {
                fn pos(&self) -> Position { self.base.pos() }
            }
            impl GameObjectProperties for $name {
                fn exists(&self) -> bool { self.base.exists() }
                fn id(&self) -> JsString { self.base.id() }
                fn x(&self) -> u8 { self.base.x() }
                fn y(&self) -> u8 { self.base.y() }
                fn ticks_to_decay(&self) -> Option<u32> { self.base.ticks_to_decay() }
            }
            impl AsRef<GameObject> for $name {
                fn as_ref(&self) -> &GameObject { &self.base }
            }
            impl AsRef<wasm_bindgen::JsValue> for $name {
                fn as_ref(&self) -> &wasm_bindgen::JsValue {
                    unsafe { &*(&self.base as *const GameObject as *const wasm_bindgen::JsValue) }
                }
            }
            impl std::ops::Deref for $name {
                type Target = GameObject;
                fn deref(&self) -> &Self::Target {
                    &self.base
                }
            }
        };
    }

    define_mock_struct!(StructureTower, { hits: u32, hits_max: u32, energy: u32, energy_max: u32, my: Option<bool> });
    define_mock_struct!(StructureExtension, { hits: u32, hits_max: u32, energy: u32, energy_max: u32, my: Option<bool> });
    define_mock_struct!(StructureRampart, { hits: u32, hits_max: u32, my: Option<bool> });
    define_mock_struct!(StructureContainer, { hits: u32, hits_max: u32, store: u32, store_max: u32 });
    define_mock_struct!(StructureRoad, { hits: u32, hits_max: u32 });
    define_mock_struct!(StructureWall, { hits: u32, hits_max: u32 });
    define_mock_struct!(Source, { energy: u32, energy_max: u32 });
    define_mock_struct!(Resource, { amount: u32, resource_type: String });
    define_mock_struct!(ConstructionSite, { my: bool, progress: u32, progress_total: u32 });
    define_mock_struct!(Flag, { my: Option<bool> });
    define_mock_struct!(ScoreCollector, { my: bool });
    define_mock_struct!(BonusFlag, { my: Option<bool> });
    define_mock_struct!(AreaEffect, { effect_type: String });

    impl HasHits for StructureSpawn {
        fn hits(&self) -> u32 {
            self.hits
        }
        fn hits_max(&self) -> u32 {
            self.hits_max
        }
    }
    impl HasHits for StructureTower {
        fn hits(&self) -> u32 {
            self.hits
        }
        fn hits_max(&self) -> u32 {
            self.hits_max
        }
    }
    impl HasHits for StructureExtension {
        fn hits(&self) -> u32 {
            self.hits
        }
        fn hits_max(&self) -> u32 {
            self.hits_max
        }
    }
    impl HasHits for StructureRampart {
        fn hits(&self) -> u32 {
            self.hits
        }
        fn hits_max(&self) -> u32 {
            self.hits_max
        }
    }
    impl HasHits for StructureContainer {
        fn hits(&self) -> u32 {
            self.hits
        }
        fn hits_max(&self) -> u32 {
            self.hits_max
        }
    }
    impl HasHits for StructureRoad {
        fn hits(&self) -> u32 {
            self.hits
        }
        fn hits_max(&self) -> u32 {
            self.hits_max
        }
    }
    impl HasHits for StructureWall {
        fn hits(&self) -> u32 {
            self.hits
        }
        fn hits_max(&self) -> u32 {
            self.hits_max
        }
    }

    impl OwnedStructureProperties for StructureTower {
        fn my(&self) -> Option<bool> {
            self.my
        }
    }
    impl OwnedStructureProperties for StructureExtension {
        fn my(&self) -> Option<bool> {
            self.my
        }
    }
    impl OwnedStructureProperties for StructureRampart {
        fn my(&self) -> Option<bool> {
            self.my
        }
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct Spawning {
        pub need_time: u32,
        pub remaining_time: u32,
    }

    impl Spawning {
        pub fn need_time(&self) -> u32 {
            self.need_time
        }
        pub fn remaining_time(&self) -> u32 {
            self.remaining_time
        }
    }

    define_mock_struct!(StructureSpawn, { hits: u32, hits_max: u32, energy: u32, energy_max: u32, my: Option<bool>, spawning: Option<Spawning>, next_id: String, directions: Vec<Direction> });

    impl OwnedStructureProperties for StructureSpawn {
        fn my(&self) -> Option<bool> {
            self.my
        }
    }

    impl StructureSpawn {
        pub fn spawning(&self) -> Option<Spawning> {
            self.spawning.clone()
        }
        pub fn next_id(&self) -> String {
            self.next_id.clone()
        }
        pub fn directions(&self) -> Vec<Direction> {
            self.directions.clone()
        }
        pub fn set_directions(&mut self, directions: &[Direction]) -> ReturnCode {
            if self.my != Some(true) {
                return ReturnCode::NotOwner;
            }
            if self.spawning.is_some() {
                return ReturnCode::Busy;
            }
            if directions.len() != 8 {
                return ReturnCode::InvalidArgs;
            }
            let mut seen = std::collections::HashSet::new();
            for &d in directions {
                if !seen.insert(d as u8) {
                    return ReturnCode::InvalidArgs;
                }
            }
            self.directions = directions.to_vec();
            ReturnCode::Ok
        }

        pub fn spawn_creep(&self, body: &[Part]) -> Result<Creep, ReturnCode> {
            use crate::prototypes::PrototypeConstant;

            if self.my != Some(true) {
                return Err(ReturnCode::NotOwner);
            }
            if self.spawning.is_some() {
                return Err(ReturnCode::Busy);
            }
            if body.is_empty() || body.len() > 50 {
                return Err(ReturnCode::InvalidArgs);
            }

            let cost: u32 = body.iter().map(|p| p.cost()).sum();

            // Calculate total energy available in friendly spawns & extensions within SPAWN_RANGE
            let (spawn_energy, extension_energy) = unsafe {
                let mut total_s = 0;
                let mut total_e = 0;
                let range = crate::constants::SPAWN_RANGE as u8;
                crate::ffi::with_host_interface(|iface| {
                    let mut ptr: *const std::ffi::c_void = std::ptr::null();
                    let mut len: usize = 0;
                    (iface.get_objects)(crate::prototypes::STRUCTURE_SPAWN::ID, &mut ptr, &mut len);
                    if !ptr.is_null() && len > 0 {
                        let slice = std::slice::from_raw_parts(ptr as *const StructureSpawn, len);
                        for s in slice {
                            if s.my == Some(true)
                                && s.base.x.abs_diff(self.base.x) <= range
                                && s.base.y.abs_diff(self.base.y) <= range
                            {
                                total_s += s.energy;
                            }
                        }
                    }
                    ptr = std::ptr::null();
                    len = 0;
                    (iface.get_objects)(
                        crate::prototypes::STRUCTURE_EXTENSION::ID,
                        &mut ptr,
                        &mut len,
                    );
                    if !ptr.is_null() && len > 0 {
                        let slice =
                            std::slice::from_raw_parts(ptr as *const StructureExtension, len);
                        for e in slice {
                            if e.my == Some(true)
                                && e.base.x.abs_diff(self.base.x) <= range
                                && e.base.y.abs_diff(self.base.y) <= range
                            {
                                total_e += e.energy;
                            }
                        }
                    }
                });
                if total_s == 0 && total_e == 0 && self.energy > 0 {
                    (self.energy, 0)
                } else {
                    (total_s, total_e)
                }
            };

            if spawn_energy + extension_energy < cost {
                return Err(ReturnCode::NotEnough);
            }

            // Encode body parts into bitfield / packed uint for FFI payload
            let mut encoded_body: usize = 0;
            for (i, part) in body.iter().enumerate().take(16) {
                encoded_body |= ((*part as usize) & 0xF) << (i * 4);
            }

            crate::ffi::with_host_interface(|iface| {
                let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                (iface.queue_action)(
                    actor_c.as_ptr(),
                    crate::ffi::ActionId::SpawnCreep as u32,
                    std::ptr::null(),
                    body.len(),
                    encoded_body,
                );
            });

            let body_parts: Vec<BodyPart> = body
                .iter()
                .map(|&part| BodyPart { part, hits: 100 })
                .collect();

            // Return mock Creep instance representing the spawning creep using pre-assigned next_id
            let new_creep_id = self.next_id.clone();
            Ok(Creep {
                base: GameObject {
                    id: new_creep_id,
                    x: self.base.x,
                    y: self.base.y,
                },
                fatigue: 0,
                hits: body.len() as u32 * 100,
                hits_max: body.len() as u32 * 100,
                my: true,
                spawning: true,
                body: body_parts,
            })
        }
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct Creep {
        pub base: GameObject,
        pub fatigue: u32,
        pub hits: u32,
        pub hits_max: u32,
        pub my: bool,
        pub spawning: bool,
        pub body: Vec<BodyPart>,
    }

    impl HasPosition for Creep {
        fn pos(&self) -> Position {
            self.base.pos()
        }
    }

    impl GameObjectProperties for Creep {
        fn exists(&self) -> bool {
            self.base.exists()
        }
        fn id(&self) -> JsString {
            self.base.id()
        }
        fn x(&self) -> u8 {
            self.base.x()
        }
        fn y(&self) -> u8 {
            self.base.y()
        }
        fn ticks_to_decay(&self) -> Option<u32> {
            self.base.ticks_to_decay()
        }
    }

    impl HasHits for Creep {
        fn hits(&self) -> u32 {
            self.hits
        }
        fn hits_max(&self) -> u32 {
            self.hits_max
        }
    }

    impl OwnedStructureProperties for Creep {
        fn my(&self) -> Option<bool> {
            Some(self.my)
        }
    }

    impl AsRef<GameObject> for Creep {
        fn as_ref(&self) -> &GameObject {
            &self.base
        }
    }
    impl AsRef<wasm_bindgen::JsValue> for Creep {
        fn as_ref(&self) -> &wasm_bindgen::JsValue {
            unsafe { &*(&self.base as *const GameObject as *const wasm_bindgen::JsValue) }
        }
    }

    impl std::ops::Deref for Creep {
        type Target = GameObject;
        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl Attackable for Creep {}
    impl Transferable for Creep {}

    impl Creep {
        pub fn fatigue(&self) -> u32 {
            self.fatigue
        }
        pub fn hits(&self) -> u32 {
            self.hits
        }
        pub fn hits_max(&self) -> u32 {
            self.hits_max
        }
        pub fn my(&self) -> bool {
            self.my
        }
        pub fn body(&self) -> Vec<BodyPart> {
            self.body.clone()
        }
        pub fn move_direction(&self, direction: Direction) -> ReturnCode {
            if !self.my {
                return ReturnCode::NotOwner;
            }
            if self.spawning {
                return ReturnCode::Busy;
            }
            if self.fatigue > 0 {
                return ReturnCode::Tired;
            }

            crate::ffi::with_host_interface(|iface| {
                let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                (iface.queue_action)(
                    actor_c.as_ptr(),
                    crate::ffi::ActionId::Move as u32,
                    std::ptr::null(),
                    direction as usize,
                    0,
                );
            });
            ReturnCode::Ok
        }
        pub fn move_to(&self, target: &impl HasPosition, _options: Option<&Object>) -> ReturnCode {
            if !self.my {
                return ReturnCode::NotOwner;
            }
            if self.spawning {
                return ReturnCode::Busy;
            }
            if self.fatigue > 0 {
                return ReturnCode::Tired;
            }
            let has_move_part = self.body.iter().any(|p| p.hits > 0 && p.part == Part::Move);
            if !has_move_part {
                return ReturnCode::NoBodypart;
            }

            let my_pos = self.pos();
            let target_pos = target.pos();
            if my_pos == target_pos {
                return ReturnCode::Ok;
            }

            struct CachedCreepPath {
                target_pos: Position,
                path: Vec<Position>,
                tick: u32,
            }
            thread_local! {
                static PATH_CACHE: std::cell::RefCell<std::collections::HashMap<String, CachedCreepPath>> = std::cell::RefCell::new(std::collections::HashMap::new());
            }

            let current_tick = crate::game::utils::get_ticks();
            let creep_id = self.base.id.clone();

            // Helper to collect obstacles for validation
            let get_obstacles = |c_id: &str| -> (std::collections::HashSet<(u8, u8)>, std::collections::HashSet<(u8, u8)>) {
                let mut static_obs = std::collections::HashSet::new();
                let mut creep_obs = std::collections::HashSet::new();
                let has_host = crate::ffi::with_host_interface(|_| ()).is_some();
                if has_host {
                    if let Ok(spawns) = std::panic::catch_unwind(|| crate::game::utils::get_objects_by_prototype(crate::constants::prototypes::STRUCTURE_SPAWN)) {
                        for s in spawns { static_obs.insert((s.base.x, s.base.y)); }
                    }
                    if let Ok(towers) = std::panic::catch_unwind(|| crate::game::utils::get_objects_by_prototype(crate::constants::prototypes::STRUCTURE_TOWER)) {
                        for t in towers { static_obs.insert((t.base.x, t.base.y)); }
                    }
                    if let Ok(exts) = std::panic::catch_unwind(|| crate::game::utils::get_objects_by_prototype(crate::constants::prototypes::STRUCTURE_EXTENSION)) {
                        for e in exts { static_obs.insert((e.base.x, e.base.y)); }
                    }
                    if let Ok(walls) = std::panic::catch_unwind(|| crate::game::utils::get_objects_by_prototype(crate::constants::prototypes::STRUCTURE_WALL)) {
                        for w in walls { static_obs.insert((w.base.x, w.base.y)); }
                    }
                    if let Ok(creeps) = std::panic::catch_unwind(|| crate::game::utils::get_objects_by_prototype(crate::constants::prototypes::CREEP)) {
                        for c in creeps {
                            if c.base.id != c_id {
                                creep_obs.insert((c.base.x, c.base.y));
                            }
                        }
                    }
                }
                (static_obs, creep_obs)
            };
            let (static_obs, creep_obs) = get_obstacles(&creep_id);

            // Attempt to reuse cached path (default reusePath = 5 ticks in Screeps)
            let mut cached_next_step: Option<Position> = None;
            PATH_CACHE.with(|cache_cell| {
                let cache = cache_cell.borrow();
                if let Some(cached) = cache.get(&creep_id) {
                    if cached.target_pos == target_pos && current_tick < cached.tick + 5 {
                        if let Some(idx) = cached.path.iter().position(|p| *p == my_pos) {
                            if idx + 1 < cached.path.len() {
                                let step = cached.path[idx + 1];
                                let is_blocked = creep_obs.contains(&(step.x, step.y)) || static_obs.contains(&(step.x, step.y));
                                if !is_blocked {
                                    cached_next_step = Some(step);
                                }
                            }
                        }
                    }
                }
            });

            let next_step = if let Some(step) = cached_next_step {
                step
            } else {
                // Build CostMatrix populating static obstacles and other creeps as cost 255
                let mut cm = crate::game::pathfinder::CostMatrix::new();
                for &(ox, oy) in &static_obs {
                    cm.set(ox, oy, 255);
                }
                for &(ox, oy) in &creep_obs {
                    cm.set(ox, oy, 255);
                }

                // Target tile should be passable even if occupied (e.g. attacking enemy spawn/creep)
                cm.set(target_pos.x, target_pos.y, 0);

                let opts = crate::game::pathfinder::SearchPathOptions::new();
                opts.cost_matrix(&cm);

                let is_target_obstacle = static_obs.contains(&(target_pos.x, target_pos.y));
                let origin_js = serde_wasm_bindgen::to_value(&my_pos).unwrap_or(wasm_bindgen::JsValue::UNDEFINED);
                let goal_js = if is_target_obstacle {
                    let spec = crate::game::pathfinder::GoalSpec { pos: target_pos, range: 1 };
                    serde_wasm_bindgen::to_value(&spec).unwrap_or(wasm_bindgen::JsValue::UNDEFINED)
                } else {
                    serde_wasm_bindgen::to_value(&target_pos).unwrap_or(wasm_bindgen::JsValue::UNDEFINED)
                };
                let search_results = crate::game::pathfinder::search_path(&origin_js, &goal_js, Some(&opts));

                if let Some(&first_step) = search_results.path.first() {
                    // Prepend current position to full path and store in cache
                    let mut full_path = vec![my_pos];
                    full_path.extend(search_results.path.clone());

                    PATH_CACHE.with(|cache_cell| {
                        cache_cell.borrow_mut().insert(creep_id.clone(), CachedCreepPath {
                            target_pos,
                            path: full_path,
                            tick: current_tick,
                        });
                    });

                    first_step
                } else {
                    my_pos
                }
            };

            let dx = next_step.x as i32 - my_pos.x as i32;
            let dy = next_step.y as i32 - my_pos.y as i32;

            let direction = match (dx.signum(), dy.signum()) {
                (0, -1) => Direction::Top,
                (1, -1) => Direction::TopRight,
                (1, 0) => Direction::Right,
                (1, 1) => Direction::BottomRight,
                (0, 1) => Direction::Bottom,
                (-1, 1) => Direction::BottomLeft,
                (-1, 0) => Direction::Left,
                (-1, -1) => Direction::TopLeft,
                _ => return ReturnCode::Ok,
            };
            self.move_direction(direction)
        }
        pub fn attack(&self, target: &impl Attackable) -> ReturnCode {
            if !self.my {
                return ReturnCode::NotOwner;
            }
            if self.spawning {
                return ReturnCode::Busy;
            }
            crate::ffi::with_host_interface(|iface| {
                let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                let target_c = std::ffi::CString::new(target.as_ref().id.clone()).unwrap();
                (iface.queue_action)(
                    actor_c.as_ptr(),
                    crate::ffi::ActionId::Attack as u32,
                    target_c.as_ptr(),
                    0,
                    0,
                );
            });
            ReturnCode::Ok
        }
        pub fn ranged_attack(&self, target: &impl Attackable) -> ReturnCode {
            if !self.my {
                return ReturnCode::NotOwner;
            }
            if self.spawning {
                return ReturnCode::Busy;
            }
            crate::ffi::with_host_interface(|iface| {
                let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                let target_c = std::ffi::CString::new(target.as_ref().id.clone()).unwrap();
                (iface.queue_action)(
                    actor_c.as_ptr(),
                    crate::ffi::ActionId::RangedAttack as u32,
                    target_c.as_ptr(),
                    0,
                    0,
                );
            });
            ReturnCode::Ok
        }
        pub fn ranged_mass_attack(&self) -> ReturnCode {
            if !self.my {
                return ReturnCode::NotOwner;
            }
            if self.spawning {
                return ReturnCode::Busy;
            }
            crate::ffi::with_host_interface(|iface| {
                let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                (iface.queue_action)(
                    actor_c.as_ptr(),
                    crate::ffi::ActionId::RangedMassAttack as u32,
                    std::ptr::null(),
                    0,
                    0,
                );
            });
            ReturnCode::Ok
        }
        pub fn heal(&self, target: &Creep) -> ReturnCode {
            if !self.my {
                return ReturnCode::NotOwner;
            }
            if self.spawning {
                return ReturnCode::Busy;
            }
            crate::ffi::with_host_interface(|iface| {
                let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                let target_c = std::ffi::CString::new(target.base.id.clone()).unwrap();
                (iface.queue_action)(
                    actor_c.as_ptr(),
                    crate::ffi::ActionId::Heal as u32,
                    target_c.as_ptr(),
                    0,
                    0,
                );
            });
            ReturnCode::Ok
        }
        pub fn ranged_heal(&self, target: &Creep) -> ReturnCode {
            if !self.my {
                return ReturnCode::NotOwner;
            }
            if self.spawning {
                return ReturnCode::Busy;
            }
            crate::ffi::with_host_interface(|iface| {
                let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                let target_c = std::ffi::CString::new(target.base.id.clone()).unwrap();
                (iface.queue_action)(
                    actor_c.as_ptr(),
                    crate::ffi::ActionId::RangedHeal as u32,
                    target_c.as_ptr(),
                    0,
                    0,
                );
            });
            ReturnCode::Ok
        }
        pub fn harvest(&self, source: &Source) -> ReturnCode {
            if !self.my {
                return ReturnCode::NotOwner;
            }
            if self.spawning {
                return ReturnCode::Busy;
            }
            crate::ffi::with_host_interface(|iface| {
                let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                let target_c = std::ffi::CString::new(source.base.id.clone()).unwrap();
                (iface.queue_action)(
                    actor_c.as_ptr(),
                    crate::ffi::ActionId::Harvest as u32,
                    target_c.as_ptr(),
                    0,
                    0,
                );
            });
            ReturnCode::Ok
        }
        pub fn transfer(
            &self,
            target: &impl Transferable,
            resource_type: ResourceType,
            amount: Option<u32>,
        ) -> ReturnCode {
            if !self.my {
                return ReturnCode::NotOwner;
            }
            if self.spawning {
                return ReturnCode::Busy;
            }
            crate::ffi::with_host_interface(|iface| {
                let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                let target_c = std::ffi::CString::new(target.as_ref().id.clone()).unwrap();
                (iface.queue_action)(
                    actor_c.as_ptr(),
                    crate::ffi::ActionId::Transfer as u32,
                    target_c.as_ptr(),
                    resource_type as usize,
                    amount.unwrap_or(0) as usize,
                );
            });
            ReturnCode::Ok
        }
        pub fn withdraw(
            &self,
            target: &impl Withdrawable,
            resource_type: ResourceType,
            amount: Option<u32>,
        ) -> ReturnCode {
            if !self.my {
                return ReturnCode::NotOwner;
            }
            if self.spawning {
                return ReturnCode::Busy;
            }
            crate::ffi::with_host_interface(|iface| {
                let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                let target_c = std::ffi::CString::new(target.as_ref().id.clone()).unwrap();
                (iface.queue_action)(
                    actor_c.as_ptr(),
                    crate::ffi::ActionId::Withdraw as u32,
                    target_c.as_ptr(),
                    resource_type as usize,
                    amount.unwrap_or(0) as usize,
                );
            });
            ReturnCode::Ok
        }
        pub fn spawning(&self) -> bool {
            self.spawning
        }
        pub fn pull(&self, _target: &Creep) -> ReturnCode {
            ReturnCode::Ok
        }
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct BodyPart {
        pub part: Part,
        pub hits: u32,
    }

    impl BodyPart {
        pub fn part(&self) -> Part {
            self.part
        }
        pub fn hits(&self) -> u32 {
            self.hits
        }
    }

    #[derive(Clone, Debug)]
    pub struct Store;

    impl Store {
        pub fn get_capacity(&self, _resource_type: Option<ResourceType>) -> Option<u32> {
            Some(2000)
        }
        pub fn get_used_capacity(&self, _resource_type: Option<ResourceType>) -> Option<u32> {
            Some(0)
        }
        pub fn get_free_capacity(&self, _resource_type: Option<ResourceType>) -> Option<u32> {
            Some(2000)
        }
    }

    impl HasStore for Creep {
        fn store(&self) -> Store {
            Store
        }
    }

    impl HasStore for StructureSpawn {
        fn store(&self) -> Store {
            Store
        }
    }
    impl HasStore for StructureTower {
        fn store(&self) -> Store {
            Store
        }
    }
    impl HasStore for StructureExtension {
        fn store(&self) -> Store {
            Store
        }
    }
    impl HasStore for StructureContainer {
        fn store(&self) -> Store {
            Store
        }
    }

    // Dynamic trait implementations resolved via struct fields

    impl OwnedStructureProperties for ConstructionSite {
        fn my(&self) -> Option<bool> {
            Some(self.my)
        }
    }

    impl ConstructionSite {
        pub fn my(&self) -> bool {
            self.my
        }
        pub fn progress(&self) -> u32 {
            self.progress
        }
        pub fn progress_total(&self) -> u32 {
            self.progress_total
        }
    }

    impl Transferable for StructureSpawn {}
    impl Transferable for StructureTower {}
    impl Transferable for StructureExtension {}
    impl Transferable for StructureContainer {}

    impl Withdrawable for StructureSpawn {}
    impl Withdrawable for StructureTower {}
    impl Withdrawable for StructureExtension {}
    impl Withdrawable for StructureContainer {}

    impl Attackable for StructureSpawn {}
    impl Attackable for StructureTower {}
    impl Attackable for StructureExtension {}
    impl Attackable for StructureContainer {}
    impl Attackable for StructureRoad {}
    impl Attackable for StructureRampart {}
    impl Attackable for StructureWall {}

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct Structure {
        pub base: GameObject,
    }
    impl HasPosition for Structure {
        fn pos(&self) -> Position {
            self.base.pos()
        }
    }
    impl GameObjectProperties for Structure {
        fn exists(&self) -> bool {
            self.base.exists()
        }
        fn id(&self) -> JsString {
            self.base.id()
        }
        fn x(&self) -> u8 {
            self.base.x()
        }
        fn y(&self) -> u8 {
            self.base.y()
        }
        fn ticks_to_decay(&self) -> Option<u32> {
            self.base.ticks_to_decay()
        }
    }
    impl HasHits for Structure {
        fn hits(&self) -> u32 {
            100
        }
        fn hits_max(&self) -> u32 {
            100
        }
    }
    impl AsRef<GameObject> for Structure {
        fn as_ref(&self) -> &GameObject {
            &self.base
        }
    }
    impl std::ops::Deref for Structure {
        type Target = GameObject;
        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct OwnedStructure {
        pub base: Structure,
        pub my: Option<bool>,
    }
    impl HasPosition for OwnedStructure {
        fn pos(&self) -> Position {
            self.base.pos()
        }
    }
    impl GameObjectProperties for OwnedStructure {
        fn exists(&self) -> bool {
            self.base.base.exists()
        }
        fn id(&self) -> JsString {
            self.base.base.id()
        }
        fn x(&self) -> u8 {
            self.base.base.x()
        }
        fn y(&self) -> u8 {
            self.base.base.y()
        }
        fn ticks_to_decay(&self) -> Option<u32> {
            self.base.base.ticks_to_decay()
        }
    }
    impl HasHits for OwnedStructure {
        fn hits(&self) -> u32 {
            100
        }
        fn hits_max(&self) -> u32 {
            100
        }
    }
    impl AsRef<GameObject> for OwnedStructure {
        fn as_ref(&self) -> &GameObject {
            &self.base.as_ref()
        }
    }
    impl std::ops::Deref for OwnedStructure {
        type Target = Structure;
        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }
    impl OwnedStructureProperties for OwnedStructure {
        fn my(&self) -> Option<bool> {
            self.my
        }
    }
    impl Attackable for OwnedStructure {}
}

pub use crate::constants::*;
pub use crate::enums::*;
pub use crate::objects::*;
pub use crate::traits::*;

pub mod prelude {
    pub use crate::game::pathfinder::Position;
    pub use crate::traits::*;
}

// Prototypes definitions to satisfy `screeps_arena::prototypes` checks
pub mod prototypes {
    pub trait PrototypeConstant {
        type Item;
        const ID: u32;
    }

    pub struct CREEP;
    impl PrototypeConstant for CREEP {
        type Item = crate::objects::Creep;
        const ID: u32 = 1;
    }

    pub struct STRUCTURE_SPAWN;
    impl PrototypeConstant for STRUCTURE_SPAWN {
        type Item = crate::objects::StructureSpawn;
        const ID: u32 = 2;
    }

    pub struct STRUCTURE_TOWER;
    impl PrototypeConstant for STRUCTURE_TOWER {
        type Item = crate::objects::StructureTower;
        const ID: u32 = 3;
    }

    pub struct STRUCTURE_CONTAINER;
    impl PrototypeConstant for STRUCTURE_CONTAINER {
        type Item = crate::objects::StructureContainer;
        const ID: u32 = 6;
    }

    pub struct RESOURCE;
    impl PrototypeConstant for RESOURCE {
        type Item = crate::objects::Resource;
        const ID: u32 = 9;
    }

    pub struct SOURCE;
    impl PrototypeConstant for SOURCE {
        type Item = crate::objects::Source;
        const ID: u32 = 10;
    }

    pub struct FLAG;
    impl PrototypeConstant for FLAG {
        type Item = crate::objects::Flag;
        const ID: u32 = 11;
    }

    pub struct OWNED_STRUCTURE;
    impl PrototypeConstant for OWNED_STRUCTURE {
        type Item = crate::objects::OwnedStructure;
        const ID: u32 = 16;
    }

    pub struct STRUCTURE_WALL;
    impl PrototypeConstant for STRUCTURE_WALL {
        type Item = crate::objects::StructureWall;
        const ID: u32 = 8;
    }

    pub struct STRUCTURE_RAMPART;
    impl PrototypeConstant for STRUCTURE_RAMPART {
        type Item = crate::objects::StructureRampart;
        const ID: u32 = 5;
    }

    pub struct STRUCTURE_EXTENSION;
    impl PrototypeConstant for STRUCTURE_EXTENSION {
        type Item = crate::objects::StructureExtension;
        const ID: u32 = 4;
    }

    pub struct CONSTRUCTION_SITE;
    impl PrototypeConstant for CONSTRUCTION_SITE {
        type Item = crate::objects::ConstructionSite;
        const ID: u32 = 15;
    }

    pub struct STRUCTURE_ROAD;
    impl PrototypeConstant for STRUCTURE_ROAD {
        type Item = crate::objects::StructureRoad;
        const ID: u32 = 7;
    }

    pub struct SCORE_COLLECTOR;
    impl PrototypeConstant for SCORE_COLLECTOR {
        type Item = crate::objects::ScoreCollector;
        const ID: u32 = 12;
    }

    pub struct BONUS_FLAG;
    impl PrototypeConstant for BONUS_FLAG {
        type Item = crate::objects::BonusFlag;
        const ID: u32 = 13;
    }

    pub struct AREA_EFFECT;
    impl PrototypeConstant for AREA_EFFECT {
        type Item = crate::objects::AreaEffect;
        const ID: u32 = 14;
    }

    pub use self::CONSTRUCTION_SITE as CONSTRUCTION_SITE_PROTOTYPE;
    pub use self::CREEP as CREEP_PROTOTYPE;
    pub use self::FLAG as FLAG_PROTOTYPE;
    pub use self::RESOURCE as RESOURCE_PROTOTYPE;
    pub use self::SOURCE as SOURCE_PROTOTYPE;
    pub use self::STRUCTURE_CONTAINER as STRUCTURE_CONTAINER_PROTOTYPE;
    pub use self::STRUCTURE_EXTENSION as STRUCTURE_EXTENSION_PROTOTYPE;
    pub use self::STRUCTURE_SPAWN as STRUCTURE_SPAWN_PROTOTYPE;
    pub use self::STRUCTURE_TOWER as STRUCTURE_TOWER_PROTOTYPE;
}

pub static STRUCTURE_TOWER_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_EXTENSION_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_SPAWN_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_RAMPART_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_CONTAINER_PROTOTYPE: js_sys::Object = js_sys::Object;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::pathfinder::{CostMatrix, Position, SearchPathOptions, search_path};
    use crate::objects::{Creep, GameObject};

    #[test]
    fn test_search_path_creep_as_ref_origin() {
        let creep = Creep {
            base: GameObject {
                id: "creep_1".to_string(),
                x: 10,
                y: 10,
            },
            fatigue: 0,
            hits: 100,
            hits_max: 100,
            my: true,
            spawning: false,
            body: Vec::new(),
        };

        #[derive(serde::Serialize)]
        struct GoalSpec {
            pub pos: Position,
            pub range: u8,
        }

        let goal = GoalSpec {
            pos: Position { x: 10, y: 15 },
            range: 1,
        };
        let js_goal = serde_wasm_bindgen::to_value(&goal).unwrap();

        let options = SearchPathOptions::new();
        options.plain_cost(1);

        let results = search_path(creep.as_ref(), &js_goal, Some(&options));
        assert!(!results.incomplete);
        assert!(!results.path.is_empty());
        assert_eq!(results.path.last().unwrap(), &Position { x: 10, y: 14 });
    }

    #[test]
    fn test_search_path_options_interior_mutability() {
        let options = SearchPathOptions::new();
        options.flee(true);
        options.plain_cost(5);
        options.swamp_cost(20);

        assert_eq!(options.flee.get(), Some(true));
        assert_eq!(options.plain_cost.get(), Some(5));
        assert_eq!(options.swamp_cost.get(), Some(20));
    }

    #[test]
    fn test_search_path_default_empty_costmatrix() {
        let creep = Creep {
            base: GameObject {
                id: "creep_default_cm".to_string(),
                x: 5,
                y: 5,
            },
            fatigue: 0,
            hits: 100,
            hits_max: 100,
            my: true,
            spawning: false,
            body: Vec::new(),
        };

        #[derive(serde::Serialize)]
        struct GoalSpec {
            pub pos: Position,
            pub range: u8,
        }

        let goal = GoalSpec {
            pos: Position { x: 5, y: 10 },
            range: 0,
        };
        let js_goal = serde_wasm_bindgen::to_value(&goal).unwrap();

        // 1. Pass None for options -> default empty cost matrix logic
        let results_none = search_path(creep.as_ref(), &js_goal, None);
        assert!(!results_none.incomplete);
        assert_eq!(results_none.path.len(), 5);
        assert_eq!(results_none.path.last().unwrap(), &Position { x: 5, y: 10 });

        // 2. Pass SearchPathOptions with an empty CostMatrix::new()
        let options = SearchPathOptions::new();
        let empty_cm = CostMatrix::new();
        options.cost_matrix(&empty_cm);

        let results_empty_cm = search_path(creep.as_ref(), &js_goal, Some(&options));
        assert!(!results_empty_cm.incomplete);
        assert_eq!(results_empty_cm.path.len(), 5);
        assert_eq!(results_empty_cm.path.last().unwrap(), &Position { x: 5, y: 10 });
    }

    #[test]
    fn test_search_path_with_costmatrix_obstacles() {
        let creep = Creep {
            base: GameObject {
                id: "creep_cm_obstacles".to_string(),
                x: 10,
                y: 10,
            },
            fatigue: 0,
            hits: 100,
            hits_max: 100,
            my: true,
            spawning: false,
            body: Vec::new(),
        };

        #[derive(serde::Serialize)]
        struct GoalSpec {
            pub pos: Position,
            pub range: u8,
        }

        let goal = GoalSpec {
            pos: Position { x: 10, y: 14 },
            range: 0,
        };
        let js_goal = serde_wasm_bindgen::to_value(&goal).unwrap();

        let options = SearchPathOptions::new();
        let mut cm = CostMatrix::new();

        // Block direct path (10, 11), (10, 12), (10, 13) with impassable cost 255
        cm.set(10, 11, 255);
        cm.set(10, 12, 255);
        cm.set(10, 13, 255);

        options.cost_matrix(&cm);

        let results = search_path(creep.as_ref(), &js_goal, Some(&options));
        assert!(!results.incomplete);
        assert!(!results.path.is_empty());
        assert_eq!(results.path.last().unwrap(), &Position { x: 10, y: 14 });

        // Path must detoured around the blocked column (x != 10 for intermediate positions)
        let contains_blocked_tile = results.path.iter().any(|p| p.x == 10 && (p.y >= 11 && p.y <= 13));
        assert!(!contains_blocked_tile);
    }

    use serial_test::serial;

    #[test]
    #[serial]
    fn test_search_path_around_terrain_wall() {
        extern "C" fn mock_get_ticks() -> u32 { 0 }
        extern "C" fn mock_get_cpu_time() -> u32 { 0 }
        extern "C" fn mock_get_objects(_: u32, _: *mut *const std::ffi::c_void, _: *mut usize) {}
        extern "C" fn mock_queue_action(_: *const std::ffi::c_char, _: u32, _: *const std::ffi::c_char, _: usize, _: usize) {}
        extern "C" fn mock_get_terrain_at(x: u8, y: u8) -> u32 {
            // Place a wall obstacle at (20, 21), (20, 22), (20, 23)
            if x == 20 && (y == 21 || y == 22 || y == 23) {
                1 // Wall
            } else {
                0 // Plain
            }
        }

        unsafe {
            crate::ffi::set_host_interface(crate::ffi::HostInterface {
                get_ticks: mock_get_ticks,
                get_cpu_time: mock_get_cpu_time,
                get_objects: mock_get_objects,
                get_terrain_at: mock_get_terrain_at,
                queue_action: mock_queue_action,
            });
        }

        let creep = Creep {
            base: GameObject {
                id: "creep_terrain_wall".to_string(),
                x: 20,
                y: 20,
            },
            fatigue: 0,
            hits: 100,
            hits_max: 100,
            my: true,
            spawning: false,
            body: Vec::new(),
        };

        #[derive(serde::Serialize)]
        struct GoalSpec {
            pub pos: Position,
            pub range: u8,
        }

        let goal = GoalSpec {
            pos: Position { x: 20, y: 24 },
            range: 0,
        };
        let js_goal = serde_wasm_bindgen::to_value(&goal).unwrap();

        let results = search_path(creep.as_ref(), &js_goal, None);
        assert!(!results.incomplete);
        assert!(!results.path.is_empty());
        assert_eq!(results.path.last().unwrap(), &Position { x: 20, y: 24 });

        // Path must detour around the terrain wall at (20, 21-23)
        let contains_terrain_wall = results.path.iter().any(|p| p.x == 20 && (p.y >= 21 && p.y <= 23));
        assert!(!contains_terrain_wall);
        unsafe {
            crate::ffi::HOST_INTERFACE = None;
        }
    }

    #[test]
    fn test_search_path_raw_position_goal() {
        let creep = Creep {
            base: GameObject {
                id: "creep_raw_pos_goal".to_string(),
                x: 10,
                y: 10,
            },
            fatigue: 0,
            hits: 100,
            hits_max: 100,
            my: true,
            spawning: false,
            body: Vec::new(),
        };

        // Pass a raw Position struct as goal instead of GoalSpec { pos, range }
        let raw_pos_goal = Position { x: 10, y: 13 };
        let js_goal = serde_wasm_bindgen::to_value(&raw_pos_goal).unwrap();

        let results = search_path(creep.as_ref(), &js_goal, None);
        assert!(!results.incomplete);
        assert_eq!(results.path.len(), 3);
        assert_eq!(results.path.last().unwrap(), &Position { x: 10, y: 13 });
    }

    #[test]
    fn test_search_path_multi_goals() {
        let creep = Creep {
            base: GameObject {
                id: "creep_multi_goals".to_string(),
                x: 10,
                y: 10,
            },
            fatigue: 0,
            hits: 100,
            hits_max: 100,
            my: true,
            spawning: false,
            body: Vec::new(),
        };

        // 1. Vec<GoalSpec>
        #[derive(serde::Serialize)]
        struct GoalSpec {
            pub pos: Position,
            pub range: u8,
        }

        let multi_goal_specs = vec![
            GoalSpec { pos: Position { x: 10, y: 13 }, range: 0 },
            GoalSpec { pos: Position { x: 10, y: 18 }, range: 0 },
        ];
        let js_multi_spec = serde_wasm_bindgen::to_value(&multi_goal_specs).unwrap();

        let results_spec = search_path(creep.as_ref(), &js_multi_spec, None);
        assert!(!results_spec.incomplete);
        assert_eq!(results_spec.path.last().unwrap(), &Position { x: 10, y: 13 }); // Closest goal reached

        // 2. Vec<Position>
        let multi_pos = vec![
            Position { x: 10, y: 13 },
            Position { x: 10, y: 18 },
        ];
        let js_multi_pos = serde_wasm_bindgen::to_value(&multi_pos).unwrap();

        let results_pos = search_path(creep.as_ref(), &js_multi_pos, None);
        assert!(!results_pos.incomplete);
        assert_eq!(results_pos.path.last().unwrap(), &Position { x: 10, y: 13 });
    }

    #[test]
    fn test_search_path_raw_position_origin() {
        // Pass a serialized Position directly as origin
        let raw_pos_origin = Position { x: 5, y: 5 };
        let js_origin = serde_wasm_bindgen::to_value(&raw_pos_origin).unwrap();

        let raw_pos_goal = Position { x: 5, y: 8 };
        let js_goal = serde_wasm_bindgen::to_value(&raw_pos_goal).unwrap();

        let results = search_path(&js_origin, &js_goal, None);
        assert!(!results.incomplete);
        assert_eq!(results.path.len(), 3);
        assert_eq!(results.path.last().unwrap(), &Position { x: 5, y: 8 });
    }

    #[test]
    fn test_spawn_creep_body_packing() {
        use crate::constants::Part;

        let spawn = StructureSpawn {
            base: GameObject {
                id: "spawn1".to_string(),
                x: 10,
                y: 10,
            },
            hits: 5000,
            hits_max: 5000,
            energy: 1000,
            energy_max: 1000,
            my: Some(true),
            spawning: None,
            next_id: "creep1".to_string(),
            directions: crate::constants::DEFAULT_SPAWN_DIRECTIONS.to_vec(),
        };

        let body = vec![Part::Move, Part::Work, Part::Carry, Part::Attack, Part::RangedAttack, Part::Tough, Part::Heal];
        let mut encoded_body: usize = 0;
        for (i, part) in body.iter().enumerate().take(16) {
            encoded_body |= ((*part as usize) & 0xF) << (i * 4);
        }

        // 0x7654321 = (7<<24) | (6<<20) | (5<<16) | (4<<12) | (3<<8) | (2<<4) | (1<<0)
        assert_eq!(encoded_body, 0x7654321);

        let creep = spawn.spawn_creep(&body).unwrap();
        assert_eq!(creep.body().len(), 7);
        assert_eq!(creep.body()[0].part(), Part::Move);
        assert_eq!(creep.body()[6].part(), Part::Heal);
    }

    use std::cell::RefCell;

    thread_local! {
        static LAST_ACTION: RefCell<Option<(u32, String, String, usize, usize)>> = RefCell::new(None);
    }

    #[test]
    #[serial]
    fn test_creep_actions_ffi_queueing() {
        extern "C" fn mock_get_ticks() -> u32 { 0 }
        extern "C" fn mock_get_cpu_time() -> u32 { 0 }
        extern "C" fn mock_get_objects(_: u32, _: *mut *const std::ffi::c_void, _: *mut usize) {}
        extern "C" fn mock_get_terrain_at(_: u8, _: u8) -> u32 { 0 }
        extern "C" fn mock_queue_action(
            actor: *const std::ffi::c_char,
            action_id: u32,
            target: *const std::ffi::c_char,
            arg1: usize,
            arg2: usize,
        ) {
            let actor_str = unsafe { std::ffi::CStr::from_ptr(actor).to_str().unwrap().to_string() };
            let target_str = if target.is_null() {
                String::new()
            } else {
                unsafe { std::ffi::CStr::from_ptr(target).to_str().unwrap().to_string() }
            };
            LAST_ACTION.with(|cell| {
                *cell.borrow_mut() = Some((action_id, actor_str, target_str, arg1, arg2));
            });
        }

        unsafe {
            crate::ffi::set_host_interface(crate::ffi::HostInterface {
                get_ticks: mock_get_ticks,
                get_cpu_time: mock_get_cpu_time,
                get_objects: mock_get_objects,
                get_terrain_at: mock_get_terrain_at,
                queue_action: mock_queue_action,
            });
        }

        let creep = Creep {
            base: GameObject {
                id: "c1".to_string(),
                x: 10,
                y: 10,
            },
            fatigue: 0,
            hits: 100,
            hits_max: 100,
            my: true,
            spawning: false,
            body: vec![
                BodyPart { part: Part::Move, hits: 100 },
                BodyPart { part: Part::Work, hits: 100 },
                BodyPart { part: Part::Carry, hits: 100 },
                BodyPart { part: Part::Attack, hits: 100 },
                BodyPart { part: Part::RangedAttack, hits: 100 },
                BodyPart { part: Part::Heal, hits: 100 },
            ],
        };

        // 1. Move Direction
        assert_eq!(creep.move_direction(Direction::Top), ReturnCode::Ok);
        LAST_ACTION.with(|cell| {
            let (action, actor, target, arg1, _) = cell.borrow().clone().unwrap();
            assert_eq!(action, crate::ffi::ActionId::Move as u32);
            assert_eq!(actor, "c1");
            assert_eq!(target, "");
            assert_eq!(arg1, Direction::Top as usize);
        });

        // 2. Attack
        let target_creep = Creep {
            base: GameObject {
                id: "c2".to_string(),
                x: 11,
                y: 10,
            },
            fatigue: 0,
            hits: 100,
            hits_max: 100,
            my: false,
            spawning: false,
            body: vec![],
        };
        assert_eq!(creep.attack(&target_creep), ReturnCode::Ok);
        LAST_ACTION.with(|cell| {
            let (action, actor, target, _, _) = cell.borrow().clone().unwrap();
            assert_eq!(action, crate::ffi::ActionId::Attack as u32);
            assert_eq!(actor, "c1");
            assert_eq!(target, "c2");
        });

        // 3. Ranged Attack
        assert_eq!(creep.ranged_attack(&target_creep), ReturnCode::Ok);
        LAST_ACTION.with(|cell| {
            let (action, actor, target, _, _) = cell.borrow().clone().unwrap();
            assert_eq!(action, crate::ffi::ActionId::RangedAttack as u32);
            assert_eq!(actor, "c1");
            assert_eq!(target, "c2");
        });

        // 4. Heal
        assert_eq!(creep.heal(&target_creep), ReturnCode::Ok);
        LAST_ACTION.with(|cell| {
            let (action, actor, target, _, _) = cell.borrow().clone().unwrap();
            assert_eq!(action, crate::ffi::ActionId::Heal as u32);
            assert_eq!(actor, "c1");
            assert_eq!(target, "c2");
        });

        // 5. Ranged Heal
        assert_eq!(creep.ranged_heal(&target_creep), ReturnCode::Ok);
        LAST_ACTION.with(|cell| {
            let (action, actor, target, _, _) = cell.borrow().clone().unwrap();
            assert_eq!(action, crate::ffi::ActionId::RangedHeal as u32);
            assert_eq!(actor, "c1");
            assert_eq!(target, "c2");
        });

        // 6. Transfer
        assert_eq!(creep.transfer(&target_creep, ResourceType::Energy, Some(50)), ReturnCode::Ok);
        LAST_ACTION.with(|cell| {
            let (action, actor, target, arg1, arg2) = cell.borrow().clone().unwrap();
            assert_eq!(action, crate::ffi::ActionId::Transfer as u32);
            assert_eq!(actor, "c1");
            assert_eq!(target, "c2");
            assert_eq!(arg1, ResourceType::Energy as usize);
            assert_eq!(arg2, 50);
        });

        // 7. Withdraw
        let target_container = StructureContainer {
            base: GameObject {
                id: "cont1".to_string(),
                x: 10,
                y: 11,
            },
            hits: 2500,
            hits_max: 2500,
            store: 1000,
            store_max: 2000,
        };
        assert_eq!(creep.withdraw(&target_container, ResourceType::Energy, Some(100)), ReturnCode::Ok);
        LAST_ACTION.with(|cell| {
            let (action, actor, target, arg1, arg2) = cell.borrow().clone().unwrap();
            assert_eq!(action, crate::ffi::ActionId::Withdraw as u32);
            assert_eq!(actor, "c1");
            assert_eq!(target, "cont1");
            assert_eq!(arg1, ResourceType::Energy as usize);
            assert_eq!(arg2, 100);
        });

        // 8. Move To
        let creep_no_move = Creep {
            base: GameObject {
                id: "c_no_move".to_string(),
                x: 10,
                y: 10,
            },
            fatigue: 0,
            hits: 100,
            hits_max: 100,
            my: true,
            spawning: false,
            body: Vec::new(),
        };

        let creep_with_move = Creep {
            base: GameObject {
                id: "c_move".to_string(),
                x: 10,
                y: 10,
            },
            fatigue: 0,
            hits: 100,
            hits_max: 100,
            my: true,
            spawning: false,
            body: vec![BodyPart {
                part: Part::Move,
                hits: 100,
            }],
        };

        let move_target = Position { x: 15, y: 10 };

        assert_eq!(creep_no_move.move_to(&move_target, None), ReturnCode::NoBodypart);

        LAST_ACTION.with(|cell| *cell.borrow_mut() = None);
        assert_eq!(creep_with_move.move_to(&move_target, None), ReturnCode::Ok);

        LAST_ACTION.with(|cell| {
            if let Some((action, actor, _target, arg1, _arg2)) = cell.borrow().clone() {
                assert_eq!(action, crate::ffi::ActionId::Move as u32);
                assert_eq!(actor, "c_move");
                assert!(arg1 == Direction::Right as usize || arg1 == Direction::TopRight as usize);
            }
        });

        unsafe {
            crate::ffi::HOST_INTERFACE = None;
        }
    }
}
