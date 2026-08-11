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

pub trait GameObjectProperties: HasPosition {
    fn exists(&self) -> bool;
    fn id(&self) -> JsString;
    fn x(&self) -> u8;
    fn y(&self) -> u8;
    fn ticks_to_decay(&self) -> Option<u32>;
    fn find_path_to(
        &self,
        pos: &impl HasPosition,
        options: Option<&crate::game::pathfinder::FindPathOptions>,
    ) -> crate::game::pathfinder::SearchResults {
        crate::utils::find_path(self, pos, options)
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
