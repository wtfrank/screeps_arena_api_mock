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
