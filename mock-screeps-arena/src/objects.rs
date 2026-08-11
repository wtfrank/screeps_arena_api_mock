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

        let get_obstacles = || -> (std::collections::HashSet<(u8, u8)>, std::collections::HashSet<(u8, u8)>) {
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
                        creep_obs.insert((c.base.x, c.base.y));
                    }
                }
            }
            (static_obs, creep_obs)
        };
        let (static_obs, creep_obs) = get_obstacles();

        // Build CostMatrix populating static obstacles and all creeps as cost 255
        let mut cm = crate::game::pathfinder::CostMatrix::new();
        for &(ox, oy) in &static_obs {
            cm.set(ox, oy, 255);
        }
        for &(ox, oy) in &creep_obs {
            cm.set(ox, oy, 255);
        }

        let opts = crate::game::pathfinder::SearchPathOptions::new();
        opts.cost_matrix(&cm);

        let origin_js = serde_wasm_bindgen::to_value(&my_pos).unwrap_or(wasm_bindgen::JsValue::UNDEFINED);
        let goal_js = serde_wasm_bindgen::to_value(&target_pos).unwrap_or(wasm_bindgen::JsValue::UNDEFINED);
        let search_results = crate::game::pathfinder::search_path(&origin_js, &goal_js, Some(&opts));

        let next_step = match search_results.path.first() {
            Some(&step) => step,
            None => return ReturnCode::Ok,
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
