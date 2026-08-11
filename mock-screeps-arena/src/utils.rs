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
        if code & 1 != 0 {
            crate::constants::Terrain::Wall
        } else if code & 2 != 0 {
            crate::constants::Terrain::Swamp
        } else {
            crate::constants::Terrain::Plain
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

pub fn find_path(
    from_pos: &impl crate::traits::HasPosition,
    to_pos: &impl crate::traits::HasPosition,
    options: Option<&crate::game::pathfinder::FindPathOptions>,
) -> crate::game::pathfinder::SearchResults {
    use crate::game::pathfinder::{GoalSpec, SearchPathOptions, search_path};

    let from = from_pos.pos();
    let to = to_pos.pos();

    // Replicate findPath logic:
    // If no costMatrix in opts, searchPath is called with range = Math.max(1, opts.range || 0)
    // Here SearchPathOptions wraps cost_matrix
    let opts = SearchPathOptions::new();
    if let Some(find_opts) = options {
        if let Some(ref cm) = find_opts.cost_matrix {
            opts.cost_matrix(cm);
        }
    }

    let goal = GoalSpec {
        pos: to,
        range: 1, // Math.max(1, opts.range || 0) -> default range is 1 in findPath
    };

    let from_js = serde_wasm_bindgen::to_value(&from).unwrap_or(wasm_bindgen::JsValue::UNDEFINED);
    let goal_js = serde_wasm_bindgen::to_value(&goal).unwrap_or(wasm_bindgen::JsValue::UNDEFINED);

    let mut result = search_path(&from_js, &goal_js, Some(&opts));

    // Post-processing logic from findPath:
    // if (!opts.range && ((result.path.length && getDistance(result.path[last], toPos) === 1) || (!result.path.length && getDistance(fromPos, toPos) <= 1))) {
    //     result.path.push({x: toPos.x, y: toPos.y});
    // }
    let dist_from_to = from.x.abs_diff(to.x).max(from.y.abs_diff(to.y));
    if let Some(last_pos) = result.path.last() {
        let dist_last_to = last_pos.x.abs_diff(to.x).max(last_pos.y.abs_diff(to.y));
        if dist_last_to == 1 {
            result.path.push(to);
        }
    } else if dist_from_to <= 1 {
        result.path.push(to);
    }

    result
}
