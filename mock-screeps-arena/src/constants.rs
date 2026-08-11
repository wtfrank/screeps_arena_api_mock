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
