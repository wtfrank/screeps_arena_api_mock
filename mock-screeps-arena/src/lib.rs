pub mod ffi;

pub mod constants {
    use serde::{Serialize, Deserialize};

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

    pub use crate::prototypes;
    pub use self::extra::{ROOM_HEIGHT, ROOM_WIDTH};

    pub mod numbers {
        pub use super::{ATTACK_POWER, HEAL_POWER, RANGED_ATTACK_POWER, CARRY_CAPACITY, DISMANTLE_POWER};
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
    use wasm_bindgen::JsCast;
    use js_sys::{Array, JsString, Object};

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
        fn find_path_to(&self, _pos: &Object, _options: Option<&crate::game::pathfinder::FindPathOptions>) -> Array { Array }
        fn find_in_range<T>(&self, _positions: &[T], _range: u8) -> Vec<T> where T: HasPosition + JsCast { Vec::new() }
        fn find_closest_by_range<T>(&self, _positions: &[T]) -> Option<T> where T: HasPosition + JsCast { None }
        fn find_closest_by_path<T>(&self, _positions: &[T], _options: Option<&crate::game::pathfinder::FindPathOptions>) -> Option<T> where T: HasPosition + JsCast { None }
        fn get_range_to(&self, _pos: &Object) -> u8 { 0 }
    }

    pub trait OwnedStructureProperties {
        fn my(&self) -> Option<bool>;
    }

    pub trait Transferable: AsRef<GameObject> {}
    pub trait Withdrawable: AsRef<GameObject> {}
    pub trait Attackable: HasHits + AsRef<GameObject> {}
}

pub mod game {
    pub use self::utils::{arena_info, get_ticks, get_cpu_time, get_heap_statistics, get_object_by_id, get_objects, get_objects_by_prototype, get_terrain_at, create_construction_site};

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
            pub fn color(mut self, _color: &str) -> Self { self }
            pub fn opacity(mut self, _opacity: f64) -> Self { self }
            pub fn font_size(mut self, _font_size: f64) -> Self { self }
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
            pub fn new(_layer: Option<u8>, _persistent: bool) -> Self { Visual }
            pub fn text(&self, _text: &str, _pos: &VisualPosition, _style: Option<&TextStyle>) -> &Self { self }
            pub fn line(&self, _from: &VisualPosition, _to: &VisualPosition, _style: Option<&Object>) -> &Self { self }
            pub fn circle(&self, _pos: &VisualPosition, _style: Option<&Object>) -> &Self { self }
            pub fn rect(&self, _pos: &VisualPosition, _w: f64, _h: f64, _style: Option<&Object>) -> &Self { self }
            pub fn poly(&self, _points: &[VisualPosition], _style: Option<&Object>) -> &Self { self }
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
                VisualPosition { x: pos.x as f32, y: pos.y as f32 }
            }
        }
    }

    pub mod utils {
        use serde::{Serialize, Deserialize};

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
            pub fn used_heap_size(&self) -> u32 { self.used_heap_size }
        }

        pub fn get_ticks() -> u32 { 1 }
        pub fn get_cpu_time() -> u32 { 0 }
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
        pub fn get_object_by_id<T>(_id: &str) -> Option<T> { None }
        pub fn get_objects() -> Vec<crate::objects::GameObject> { Vec::new() }
        pub fn get_objects_by_prototype<T>(_prototype: T) -> Vec<T::Item> where T: crate::prototypes::PrototypeConstant { Vec::new() }
        pub fn get_terrain_at(_pos: &wasm_bindgen::JsValue) -> crate::constants::Terrain { crate::constants::Terrain::Plain }
        pub fn create_construction_site(_x: u8, _y: u8, _structure_type: &js_sys::Object) -> Result<crate::objects::ConstructionSite, crate::constants::ReturnCode> { Err(crate::constants::ReturnCode::Error) }
    }

    pub mod pathfinder {
        use serde::{Serialize, Deserialize};
        use js_sys::{Array, Object};
        use crate::traits::HasPosition;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct Position {
            pub x: u8,
            pub y: u8,
        }

        impl HasPosition for Position {
            fn pos(&self) -> Position { *self }
        }

        impl std::fmt::Display for Position {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        impl From<Position> for Object {
            fn from(_pos: Position) -> Self { Object }
        }

        impl std::ops::Add<crate::constants::Direction> for Position {
            type Output = Position;
            fn add(self, dir: crate::constants::Direction) -> Position {
                let mut p = self;
                match dir {
                    crate::constants::Direction::Top => { p.y = p.y.saturating_sub(1); }
                    crate::constants::Direction::TopRight => { p.x = p.x.saturating_add(1); p.y = p.y.saturating_sub(1); }
                    crate::constants::Direction::Right => { p.x = p.x.saturating_add(1); }
                    crate::constants::Direction::BottomRight => { p.x = p.x.saturating_add(1); p.y = p.y.saturating_add(1); }
                    crate::constants::Direction::Bottom => { p.y = p.y.saturating_add(1); }
                    crate::constants::Direction::BottomLeft => { p.x = p.x.saturating_sub(1); p.y = p.y.saturating_add(1); }
                    crate::constants::Direction::Left => { p.x = p.x.saturating_sub(1); }
                    crate::constants::Direction::TopLeft => { p.x = p.x.saturating_sub(1); p.y = p.y.saturating_sub(1); }
                }
                p
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SearchPathOptions {
            pub cost_matrix: Option<CostMatrix>,
            pub max_ops: Option<u32>,
            pub heuristic_weight: Option<f64>,
            pub max_rooms: Option<u32>,
            pub plain_cost: Option<u8>,
            pub swamp_cost: Option<u8>,
            pub flee: Option<bool>,
        }

        impl SearchPathOptions {
            pub fn new() -> Self {
                SearchPathOptions {
                    cost_matrix: None,
                    max_ops: None,
                    heuristic_weight: None,
                    max_rooms: None,
                    plain_cost: None,
                    swamp_cost: None,
                    flee: None,
                }
            }
            pub fn cost_matrix(&self, _cm: &CostMatrix) -> &Self { self }
            pub fn max_ops(&self, _val: u32) -> &Self { self }
            pub fn heuristic_weight(&self, _val: f64) -> &Self { self }
            pub fn max_rooms(&self, _val: u32) -> &Self { self }
            pub fn plain_cost(&self, _val: u8) -> &Self { self }
            pub fn swamp_cost(&self, _val: u8) -> &Self { self }
            pub fn flee(&self, _val: bool) -> &Self { self }
            pub fn get_cost_matrix(&self) -> CostMatrix { CostMatrix }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FindPathOptions {
            pub cost_matrix: Option<CostMatrix>,
        }

        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub struct CostMatrix;

        impl CostMatrix {
            pub fn new() -> Self { CostMatrix }
            pub fn set(&self, _x: u8, _y: u8, _cost: u8) {}
            pub fn get(&self, _x: u8, _y: u8) -> u8 { 0 }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SearchResults {
            pub path: Vec<Position>,
            pub ops: u32,
            pub cost: u32,
            pub incomplete: bool,
        }

        impl SearchResults {
            pub fn path(&self) -> Vec<Position> { self.path.clone() }
            pub fn incomplete(&self) -> bool { self.incomplete }
        }

        pub fn search_path(_origin: &crate::objects::GameObject, _goal: &wasm_bindgen::JsValue, _options: Option<&SearchPathOptions>) -> SearchResults {
            SearchResults {
                path: Vec::new(),
                ops: 0,
                cost: 0,
                incomplete: true,
            }
        }
    }
}

pub mod objects {
    use crate::game::pathfinder::Position;
    use crate::traits::*;
    use crate::constants::{Part, ResourceType, ReturnCode, Direction};
    use js_sys::{JsString, Object};

    #[derive(Clone, Debug, Default)]
    pub struct GameObject {
        pub id: String,
        pub x: u8,
        pub y: u8,
    }

    impl GameObject {
        pub fn exists(&self) -> bool { true }
        pub fn id(&self) -> JsString { JsString(self.id.clone()) }
        pub fn x(&self) -> u8 { self.x }
        pub fn y(&self) -> u8 { self.y }
        pub fn ticks_to_decay(&self) -> Option<u32> { None }
    }

    impl HasPosition for GameObject {
        fn pos(&self) -> Position { Position { x: self.x, y: self.y } }
    }

    impl GameObjectProperties for GameObject {
        fn exists(&self) -> bool { true }
        fn id(&self) -> JsString { JsString(self.id.clone()) }
        fn x(&self) -> u8 { self.x }
        fn y(&self) -> u8 { self.y }
        fn ticks_to_decay(&self) -> Option<u32> { None }
    }

    macro_rules! define_mock_struct {
        ($name:ident, { $($field_name:ident : $field_type:ty),* $(,)? }) => {
            #[derive(Clone, Debug)]
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
            impl std::ops::Deref for $name {
                type Target = GameObject;
                fn deref(&self) -> &Self::Target {
                    &self.base
                }
            }
        };
    }

    define_mock_struct!(StructureSpawn, { hits: u32, hits_max: u32, energy: u32, energy_max: u32, my: Option<bool> });
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

    impl HasHits for StructureSpawn { fn hits(&self) -> u32 { self.hits } fn hits_max(&self) -> u32 { self.hits_max } }
    impl HasHits for StructureTower { fn hits(&self) -> u32 { self.hits } fn hits_max(&self) -> u32 { self.hits_max } }
    impl HasHits for StructureExtension { fn hits(&self) -> u32 { self.hits } fn hits_max(&self) -> u32 { self.hits_max } }
    impl HasHits for StructureRampart { fn hits(&self) -> u32 { self.hits } fn hits_max(&self) -> u32 { self.hits_max } }
    impl HasHits for StructureContainer { fn hits(&self) -> u32 { self.hits } fn hits_max(&self) -> u32 { self.hits_max } }
    impl HasHits for StructureRoad { fn hits(&self) -> u32 { self.hits } fn hits_max(&self) -> u32 { self.hits_max } }
    impl HasHits for StructureWall { fn hits(&self) -> u32 { self.hits } fn hits_max(&self) -> u32 { self.hits_max } }

    impl OwnedStructureProperties for StructureSpawn { fn my(&self) -> Option<bool> { self.my } }
    impl OwnedStructureProperties for StructureTower { fn my(&self) -> Option<bool> { self.my } }
    impl OwnedStructureProperties for StructureExtension { fn my(&self) -> Option<bool> { self.my } }
    impl OwnedStructureProperties for StructureRampart { fn my(&self) -> Option<bool> { self.my } }

    impl StructureSpawn {
        pub fn spawn_creep(&self, _body: &[Part]) -> Result<Creep, ReturnCode> {
            Err(ReturnCode::Error)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Creep {
        pub base: GameObject,
        pub fatigue: u32,
        pub hits: u32,
        pub hits_max: u32,
        pub my: bool,
    }

    impl HasPosition for Creep {
        fn pos(&self) -> Position { self.base.pos() }
    }

    impl GameObjectProperties for Creep {
        fn exists(&self) -> bool { self.base.exists() }
        fn id(&self) -> JsString { self.base.id() }
        fn x(&self) -> u8 { self.base.x() }
        fn y(&self) -> u8 { self.base.y() }
        fn ticks_to_decay(&self) -> Option<u32> { self.base.ticks_to_decay() }
    }

    impl HasHits for Creep {
        fn hits(&self) -> u32 { self.hits }
        fn hits_max(&self) -> u32 { self.hits_max }
    }

    impl OwnedStructureProperties for Creep {
        fn my(&self) -> Option<bool> { Some(self.my) }
    }

    impl AsRef<GameObject> for Creep {
        fn as_ref(&self) -> &GameObject { &self.base }
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
        pub fn fatigue(&self) -> u32 { self.fatigue }
        pub fn hits(&self) -> u32 { self.hits }
        pub fn hits_max(&self) -> u32 { self.hits_max }
        pub fn my(&self) -> bool { self.my }
        pub fn body(&self) -> Vec<BodyPart> { Vec::new() }
        pub fn move_direction(&self, _direction: Direction) -> ReturnCode { ReturnCode::Ok }
        pub fn move_to(&self, _target: &impl HasPosition, _options: Option<&Object>) -> ReturnCode { ReturnCode::Ok }
        pub fn attack(&self, _target: &impl Attackable) -> ReturnCode { ReturnCode::Ok }
        pub fn ranged_attack(&self, _target: &impl Attackable) -> ReturnCode { ReturnCode::Ok }
        pub fn ranged_mass_attack(&self) -> ReturnCode { ReturnCode::Ok }
        pub fn heal(&self, _target: &Creep) -> ReturnCode { ReturnCode::Ok }
        pub fn ranged_heal(&self, _target: &Creep) -> ReturnCode { ReturnCode::Ok }
        pub fn harvest(&self, _source: &Source) -> ReturnCode { ReturnCode::Ok }
        pub fn transfer(&self, _target: &impl Transferable, _resource_type: ResourceType, _amount: Option<u32>) -> ReturnCode { ReturnCode::Ok }
        pub fn withdraw(&self, _target: &impl Withdrawable, _resource_type: ResourceType, _amount: Option<u32>) -> ReturnCode { ReturnCode::Ok }
        pub fn spawning(&self) -> bool { false }
        pub fn pull(&self, _target: &Creep) -> ReturnCode { ReturnCode::Ok }
    }

    #[derive(Clone, Debug)]
    pub struct BodyPart {
        pub part: Part,
        pub hits: u32,
    }

    impl BodyPart {
        pub fn part(&self) -> Part { self.part }
        pub fn hits(&self) -> u32 { self.hits }
    }

    #[derive(Clone, Debug)]
    pub struct Store;

    impl Store {
        pub fn get_capacity(&self, _resource_type: Option<ResourceType>) -> Option<u32> { Some(2000) }
        pub fn get_used_capacity(&self, _resource_type: Option<ResourceType>) -> Option<u32> { Some(0) }
        pub fn get_free_capacity(&self, _resource_type: Option<ResourceType>) -> Option<u32> { Some(2000) }
    }

    impl HasStore for Creep {
        fn store(&self) -> Store { Store }
    }

    impl HasStore for StructureSpawn {
        fn store(&self) -> Store { Store }
    }
    impl HasStore for StructureTower {
        fn store(&self) -> Store { Store }
    }
    impl HasStore for StructureExtension {
        fn store(&self) -> Store { Store }
    }
    impl HasStore for StructureContainer {
        fn store(&self) -> Store { Store }
    }

    // Dynamic trait implementations resolved via struct fields

    impl OwnedStructureProperties for ConstructionSite {
        fn my(&self) -> Option<bool> { Some(self.my) }
    }

    impl ConstructionSite {
        pub fn my(&self) -> bool { self.my }
        pub fn progress(&self) -> u32 { self.progress }
        pub fn progress_total(&self) -> u32 { self.progress_total }
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

    #[derive(Clone, Debug)]
    pub struct Structure {
        pub base: GameObject,
    }
    impl HasPosition for Structure {
        fn pos(&self) -> Position { self.base.pos() }
    }
    impl GameObjectProperties for Structure {
        fn exists(&self) -> bool { self.base.exists() }
        fn id(&self) -> JsString { self.base.id() }
        fn x(&self) -> u8 { self.base.x() }
        fn y(&self) -> u8 { self.base.y() }
        fn ticks_to_decay(&self) -> Option<u32> { self.base.ticks_to_decay() }
    }
    impl HasHits for Structure {
        fn hits(&self) -> u32 { 100 }
        fn hits_max(&self) -> u32 { 100 }
    }
    impl AsRef<GameObject> for Structure {
        fn as_ref(&self) -> &GameObject { &self.base }
    }
    impl std::ops::Deref for Structure {
        type Target = GameObject;
        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    #[derive(Clone, Debug)]
    pub struct OwnedStructure {
        pub base: Structure,
    }
    impl HasPosition for OwnedStructure {
        fn pos(&self) -> Position { self.base.pos() }
    }
    impl GameObjectProperties for OwnedStructure {
        fn exists(&self) -> bool { self.base.base.exists() }
        fn id(&self) -> JsString { self.base.base.id() }
        fn x(&self) -> u8 { self.base.base.x() }
        fn y(&self) -> u8 { self.base.base.y() }
        fn ticks_to_decay(&self) -> Option<u32> { self.base.base.ticks_to_decay() }
    }
    impl HasHits for OwnedStructure {
        fn hits(&self) -> u32 { 100 }
        fn hits_max(&self) -> u32 { 100 }
    }
    impl AsRef<GameObject> for OwnedStructure {
        fn as_ref(&self) -> &GameObject { &self.base.as_ref() }
    }
    impl std::ops::Deref for OwnedStructure {
        type Target = Structure;
        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }
    impl OwnedStructureProperties for OwnedStructure {
        fn my(&self) -> Option<bool> { Some(true) }
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

    pub use self::CREEP as CREEP_PROTOTYPE;
    pub use self::STRUCTURE_SPAWN as STRUCTURE_SPAWN_PROTOTYPE;
    pub use self::STRUCTURE_TOWER as STRUCTURE_TOWER_PROTOTYPE;
    pub use self::STRUCTURE_CONTAINER as STRUCTURE_CONTAINER_PROTOTYPE;
    pub use self::RESOURCE as RESOURCE_PROTOTYPE;
    pub use self::SOURCE as SOURCE_PROTOTYPE;
    pub use self::FLAG as FLAG_PROTOTYPE;
    pub use self::STRUCTURE_EXTENSION as STRUCTURE_EXTENSION_PROTOTYPE;
    pub use self::CONSTRUCTION_SITE as CONSTRUCTION_SITE_PROTOTYPE;
}

pub static STRUCTURE_TOWER_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_EXTENSION_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_SPAWN_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_RAMPART_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_CONTAINER_PROTOTYPE: js_sys::Object = js_sys::Object;

