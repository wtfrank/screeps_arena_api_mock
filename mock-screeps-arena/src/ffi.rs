use std::os::raw::{c_char, c_void};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrototypeId {
    Creep = 1,
    StructureSpawn = 2,
    StructureTower = 3,
    StructureExtension = 4,
    StructureRampart = 5,
    StructureContainer = 6,
    StructureRoad = 7,
    StructureWall = 8,
    Resource = 9,
    Source = 10,
    Flag = 11,
    ScoreCollector = 12,
    BonusFlag = 13,
    AreaEffect = 14,
    ConstructionSite = 15,
    OwnedStructure = 16,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionId {
    Move = 1,
    Attack = 2,
    RangedAttack = 3,
    RangedMassAttack = 4,
    Heal = 5,
    RangedHeal = 6,
    Harvest = 7,
    Transfer = 8,
    Withdraw = 9,
    Build = 10,
    SpawnCreep = 11,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HostInterface {
    pub get_ticks: extern "C" fn() -> u32,
    pub get_cpu_time: extern "C" fn() -> u32,
    pub get_objects: extern "C" fn(proto: u32, out_ptr: *mut *const c_void, out_len: *mut usize),
    pub get_terrain_at: extern "C" fn(x: u8, y: u8) -> u32,
    pub queue_action: extern "C" fn(actor_id: *const c_char, action: u32, target_id: *const c_char, arg1: usize, arg2: usize),
}

pub static mut HOST_INTERFACE: Option<HostInterface> = None;

#[no_mangle]
pub extern "C" fn set_host_interface(interface: HostInterface) {
    unsafe {
        HOST_INTERFACE = Some(interface);
    }
}
