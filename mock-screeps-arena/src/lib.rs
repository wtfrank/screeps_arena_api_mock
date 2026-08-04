pub mod ffi;

pub mod constants {
    use serde::{Deserialize, Serialize};

    pub const SPAWN_RANGE: u32 = 20;

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
        Move = 0,
        Work = 1,
        Carry = 2,
        Attack = 3,
        Heal = 4,
        RangedAttack = 5,
        Tough = 6,
        Claim = 7,
    }

    impl Part {
        pub fn cost(&self) -> u32 {
            match self {
                Part::Move => 50,
                Part::Work => 100,
                Part::Carry => 50,
                Part::Attack => 80,
                Part::Heal => 250,
                Part::RangedAttack => 150,
                Part::Tough => 10,
                Part::Claim => 600,
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
            unsafe {
                if let Some(ref iface) = crate::ffi::HOST_INTERFACE {
                    (iface.get_ticks)()
                } else {
                    1
                }
            }
        }
        pub fn get_cpu_time() -> u32 {
            unsafe {
                if let Some(ref iface) = crate::ffi::HOST_INTERFACE {
                    (iface.get_cpu_time)()
                } else {
                    0
                }
            }
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
            unsafe {
                if let Some(ref iface) = crate::ffi::HOST_INTERFACE {
                    let mut ptr: *const std::ffi::c_void = std::ptr::null();
                    let mut len: usize = 0;
                    (iface.get_objects)(T::ID, &mut ptr, &mut len);
                    if !ptr.is_null() && len > 0 {
                        let slice = std::slice::from_raw_parts(ptr as *const T::Item, len);
                        return slice.to_vec();
                    }
                }
            }
            Vec::new()
        }

        pub fn get_terrain_at(pos: &wasm_bindgen::JsValue) -> crate::constants::Terrain {
            // In the native simulator environment without a V8 runtime,
            // pos will be a pointer or encoded value. If pos.as_f64() contains a coordinate, extract it.
            let (x, y) = (0, 0);
            unsafe {
                if let Some(ref iface) = crate::ffi::HOST_INTERFACE {
                    let code = (iface.get_terrain_at)(x, y);
                    match code {
                        1 => crate::constants::Terrain::Wall,
                        2 => crate::constants::Terrain::Swamp,
                        _ => crate::constants::Terrain::Plain,
                    }
                } else {
                    crate::constants::Terrain::Plain
                }
            }
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
        struct GoalSpec {
            pub pos: Position,
            pub range: u8,
        }

        pub fn search_path(
            origin: &wasm_bindgen::JsValue,
            goal: &wasm_bindgen::JsValue,
            options: Option<&SearchPathOptions>,
        ) -> SearchResults {
            use std::collections::{BinaryHeap, HashMap, HashSet};
            use std::cmp::Ordering;
            use crate::traits::HasPosition;

            let start = unsafe {
                let go = &*(origin as *const wasm_bindgen::JsValue as *const crate::objects::GameObject);
                go.pos()
            };
            let mut goals: Vec<GoalSpec> = Vec::new();

            if let Ok(single) = serde_wasm_bindgen::from_value::<GoalSpec>(goal.clone()) {
                goals.push(single);
            } else if let Ok(multi) = serde_wasm_bindgen::from_value::<Vec<GoalSpec>>(goal.clone()) {
                goals = multi;
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
            let max_ops = options.and_then(|o| o.max_ops.get()).unwrap_or(2000);
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

            let heuristic = |pos: Position| -> u32 {
                let mut min_h = u32::MAX;
                for g in &goals {
                    let dx = pos.x.abs_diff(g.pos.x) as u32;
                    let dy = pos.y.abs_diff(g.pos.y) as u32;
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
                estimated_total: u32,
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
            open_set.push(State {
                cost: 0,
                estimated_total: heuristic(start),
                pos: start,
            });

            let mut ops = 0;
            let mut best_target: Option<Position> = None;

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

                // Check 8-way adjacent directions
                for dx in [-1i32, 0, 1] {
                    for dy in [-1i32, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = pos.x as i32 + dx;
                        let ny = pos.y as i32 + dy;
                        if nx < 0 || nx >= 100 || ny < 0 || ny >= 100 {
                            continue;
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
                                match crate::game::utils::get_terrain_at(&serde_wasm_bindgen::to_value(&neighbor).unwrap()) {
                                    crate::constants::Terrain::Wall => continue,
                                    crate::constants::Terrain::Swamp => swamp_cost,
                                    _ => plain_cost,
                                }
                            }
                        } else {
                            match crate::game::utils::get_terrain_at(&serde_wasm_bindgen::to_value(&neighbor).unwrap()) {
                                crate::constants::Terrain::Wall => continue,
                                crate::constants::Terrain::Swamp => swamp_cost,
                                _ => plain_cost,
                            }
                        };

                        let tentative_g = cost + tile_cost;
                        if tentative_g < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                            came_from.insert(neighbor, pos);
                            g_score.insert(neighbor, tentative_g);
                            open_set.push(State {
                                cost: tentative_g,
                                estimated_total: tentative_g + heuristic(neighbor),
                                pos: neighbor,
                            });
                        }
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
                SearchResults {
                    path: Vec::new(),
                    ops,
                    cost: 0,
                    incomplete: true,
                }
            }
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
                    unsafe { &*(self as *const $name as *const wasm_bindgen::JsValue) }
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

    define_mock_struct!(StructureSpawn, { hits: u32, hits_max: u32, energy: u32, energy_max: u32, my: Option<bool>, spawning: Option<Spawning>, next_id: String });

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
                if let Some(ref iface) = crate::ffi::HOST_INTERFACE {
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
                }
                (total_s, total_e)
            };

            if spawn_energy + extension_energy < cost {
                return Err(ReturnCode::NotEnough);
            }

            // Encode body parts into bitfield / packed uint for FFI payload
            let mut encoded_body: usize = 0;
            for (i, part) in body.iter().enumerate().take(16) {
                encoded_body |= ((*part as usize) & 0xF) << (i * 4);
            }

            unsafe {
                if let Some(ref iface) = crate::ffi::HOST_INTERFACE {
                    let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                    (iface.queue_action)(
                        actor_c.as_ptr(),
                        crate::ffi::ActionId::SpawnCreep as u32,
                        std::ptr::null(),
                        body.len(),
                        encoded_body,
                    );
                }
            }

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
            unsafe { &*(self as *const Creep as *const wasm_bindgen::JsValue) }
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
            Vec::new()
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

            unsafe {
                if let Some(ref iface) = crate::ffi::HOST_INTERFACE {
                    let actor_c = std::ffi::CString::new(self.base.id.clone()).unwrap();
                    (iface.queue_action)(
                        actor_c.as_ptr(),
                        crate::ffi::ActionId::Move as u32,
                        std::ptr::null(),
                        direction as usize,
                        0,
                    );
                }
            }
            ReturnCode::Ok
        }
        pub fn move_to(&self, _target: &impl HasPosition, _options: Option<&Object>) -> ReturnCode {
            ReturnCode::Ok
        }
        pub fn attack(&self, _target: &impl Attackable) -> ReturnCode {
            ReturnCode::Ok
        }
        pub fn ranged_attack(&self, _target: &impl Attackable) -> ReturnCode {
            ReturnCode::Ok
        }
        pub fn ranged_mass_attack(&self) -> ReturnCode {
            ReturnCode::Ok
        }
        pub fn heal(&self, _target: &Creep) -> ReturnCode {
            ReturnCode::Ok
        }
        pub fn ranged_heal(&self, _target: &Creep) -> ReturnCode {
            ReturnCode::Ok
        }
        pub fn harvest(&self, _source: &Source) -> ReturnCode {
            ReturnCode::Ok
        }
        pub fn transfer(
            &self,
            _target: &impl Transferable,
            _resource_type: ResourceType,
            _amount: Option<u32>,
        ) -> ReturnCode {
            ReturnCode::Ok
        }
        pub fn withdraw(
            &self,
            _target: &impl Withdrawable,
            _resource_type: ResourceType,
            _amount: Option<u32>,
        ) -> ReturnCode {
            ReturnCode::Ok
        }
        pub fn spawning(&self) -> bool {
            self.spawning
        }
        pub fn pull(&self, _target: &Creep) -> ReturnCode {
            ReturnCode::Ok
        }
    }

    #[derive(Clone, Debug)]
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
