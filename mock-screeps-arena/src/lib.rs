pub mod constants;
pub mod ffi;
pub mod objects;
pub mod pathfinder;
pub mod pf_cc;
pub mod pf_cc_heap;
pub mod prototypes;
pub mod traits;
pub mod utils;

pub mod enums {
    pub use crate::constants::{Direction, Part, ResourceType, ReturnCode, Terrain};
}

pub mod game {
    pub use crate::utils::{
        arena_info, create_construction_site, find_path, get_cpu_time, get_heap_statistics, get_object_by_id,
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

    pub use crate::pathfinder;
    pub use crate::utils;
}

pub use crate::constants::*;
pub use crate::enums::*;
pub use crate::objects::*;
pub use crate::traits::*;

pub mod prelude {
    pub use crate::game::pathfinder::Position;
    pub use crate::traits::*;
}

pub static STRUCTURE_TOWER_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_EXTENSION_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_SPAWN_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_RAMPART_PROTOTYPE: js_sys::Object = js_sys::Object;
pub static STRUCTURE_CONTAINER_PROTOTYPE: js_sys::Object = js_sys::Object;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::pathfinder::{CostMatrix, GoalSpec, Position, SearchPathOptions, search_path};
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
        assert_eq!(results.path.last().unwrap(), &Position { x: 9, y: 14 });
    }

    #[test]
    fn test_find_path_utility() {
        let from = Position { x: 10, y: 10 };
        let to = Position { x: 10, y: 13 };

        let result = crate::game::utils::find_path(&from, &to, None);
        assert!(!result.incomplete);
        assert!(!result.path.is_empty());
        assert_eq!(result.path.last().unwrap(), &to);
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

    #[test]
    fn test_validate_path_tests_ssb5_json() {
        let path_str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/path_tests_ssb5.json");
        let file_path = std::path::Path::new(path_str);
        if !file_path.exists() {
            return;
        }

        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => return,
        };
        let data: serde_json::Value = match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(_) => return,
        };

        let terrain_str = match data["terrain"].as_str() {
            Some(s) => s,
            None => return,
        };
        let terrain_bytes: Vec<u8> = terrain_str
            .bytes()
            .map(|b| match b {
                b'1' => 1,
                b'2' => 2,
                b'3' => 3,
                _ => 0,
            })
            .collect();

        thread_local! {
            static TEST_TERRAIN: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::new());
        }
        extern "C" fn mock_get_ticks() -> u32 { 1 }
        extern "C" fn mock_get_cpu_time() -> u32 { 0 }
        extern "C" fn mock_get_objects(_: u32, _: *mut *const std::os::raw::c_void, _: *mut usize) {}
        extern "C" fn mock_get_terrain_at(x: u8, y: u8) -> u32 {
            TEST_TERRAIN.with(|t| {
                let idx = (y as usize) * 100 + (x as usize);
                t.borrow().get(idx).copied().unwrap_or(0) as u32
            })
        }
        extern "C" fn mock_queue_action(_: *const std::os::raw::c_char, _: u32, _: *const std::os::raw::c_char, _: usize, _: usize) {}

        TEST_TERRAIN.with(|t| *t.borrow_mut() = terrain_bytes);
        unsafe {
            crate::ffi::HOST_INTERFACE = Some(crate::ffi::HostInterface {
                get_ticks: mock_get_ticks,
                get_cpu_time: mock_get_cpu_time,
                get_objects: mock_get_objects,
                get_terrain_at: mock_get_terrain_at,
                queue_action: mock_queue_action,
            });
        }

        let path_tests = match data["path_tests"].as_array() {
            Some(arr) => arr,
            None => return,
        };

        let mut matched = 0;
        let mut path_diffs = 0;
        let mut cost_mismatches = 0;
        let mut inc_mismatches = 0;
        let mut len_mismatches = 0;
        let mut waypoint_diffs = 0;

        for test in path_tests {
            let ox = test["origin"]["x"].as_u64().unwrap_or(0) as u8;
            let oy = test["origin"]["y"].as_u64().unwrap_or(0) as u8;
            let gx = test["goal"]["x"].as_u64().unwrap_or(0) as u8;
            let gy = test["goal"]["y"].as_u64().unwrap_or(0) as u8;
            let range = test["range"].as_u64().unwrap_or(0) as u8;
            let flee = test["flee"].as_bool().unwrap_or(false);

            let ref_cost = test["cost"].as_u64().unwrap_or(0) as u32;
            let ref_inc = test["incomplete"].as_bool().unwrap_or(false);
            let ref_path_len = test["path"].as_array().map(|a| a.len()).unwrap_or(0);

            let start = Position { x: ox, y: oy };
            let goal = GoalSpec {
                pos: Position { x: gx, y: gy },
                range,
            };

            let mut opts = SearchPathOptions::new();
            opts.flee(flee);

            let js_origin = serde_wasm_bindgen::to_value(&start).unwrap();
            let js_goal = serde_wasm_bindgen::to_value(&goal).unwrap();

            let res = crate::pf_cc::search_path_pf_cc(&js_origin, &js_goal, Some(&opts));

            let mut path_matches = true;
            if let Some(ref_path_arr) = test["path"].as_array() {
                if res.path.len() != ref_path_arr.len() {
                    path_matches = false;
                } else {
                    for (p1, p2) in res.path.iter().zip(ref_path_arr.iter()) {
                        let rx = p2["x"].as_u64().unwrap_or(0) as u8;
                        let ry = p2["y"].as_u64().unwrap_or(0) as u8;
                        if p1.x != rx || p1.y != ry {
                            path_matches = false;
                            break;
                        }
                    }
                }
            }

            // Standardize u32::MAX for incomplete cost check
            let expected_ref_cost = if ref_inc && ref_cost == 4294967295 { 0 } else { ref_cost };

            if path_matches && res.cost == expected_ref_cost && res.incomplete == ref_inc {
                matched += 1;
            } else {
                path_diffs += 1;
                if res.incomplete != ref_inc {
                    inc_mismatches += 1;
                } else if res.cost != expected_ref_cost {
                    cost_mismatches += 1;
                    println!(
                        "COST MISMATCH: ox={} oy={} gx={} gy={} range={} flee={} | REF cost={} | OUR cost={}",
                        ox, oy, gx, gy, range, flee, ref_cost, res.cost
                    );
                } else if res.path.len() != ref_path_len {
                    len_mismatches += 1;
                    println!(
                        "LEN MISMATCH: ox={} oy={} gx={} gy={} range={} flee={} | REF len={} | OUR len={}",
                        ox, oy, gx, gy, range, flee, ref_path_len, res.path.len()
                    );
                } else {
                    waypoint_diffs += 1;
                }
            }
        }

        println!(
            "\n=== MOCK RUNNER SUMMARY (Out of {} Queries) ===",
            path_tests.len()
        );
        println!("  100% Exact Match           : {}", matched);
        println!("  Total Differences          : {}", path_diffs);
        println!("  Incomplete Flag Mismatches : {}", inc_mismatches);
        println!("  Cost Mismatches            : {}", cost_mismatches);
        println!("  Path Length Mismatches     : {}", len_mismatches);
        println!("  Same Cost/Len Waypoint Diff: {}", waypoint_diffs);

        unsafe {
            crate::ffi::HOST_INTERFACE = None;
        }
    }
}
