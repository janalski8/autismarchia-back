use std::fmt::{Formatter, Debug, Display, Error};
use std::collections::HashMap;
use state::object::{Idx, Object, TObject};
use utils::ipoint::IPoint;

pub struct Level {
    pub idx: Idx,
    pub size: IPoint,
    pub entities: HashMap<Idx, Entity>,
}
pub struct Layout<'a> {
    pub tiles: HashMap<IPoint, Vec<&'a Entity>>,
    pub size: IPoint,
}
pub struct Entity {
    pub object: Object,
    pub level: Idx,
    pub position: IPoint
}

impl<'a> Display for Layout<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}
impl<'a> Debug for Layout<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut result: Result<(), Error> = Result::Ok(());
        let size = self.size;
        for y in 0..(size.y) {
            for x in 0..(size.x) {
                let vecopt: Option<&Vec<&Entity>> = self.tiles.get(&IPoint {x, y});
                let chr = match vecopt.unwrap().iter().max_by_key(|e| e.object.get_ordinal()) {
                    None => ' ',
                    Some(entity) => match entity.object {
                        Object::Player {..} => '@',
                        Object::Wall {..} => '#',
                        Object::Floor {..} => '.',
                    }
                };
                result = result.and_then(|()| write!(f, "{}", chr));
            }
            result = result.and_then(|()| write!(f, "\n"));
        }
        result
    }
}

impl Level {
    pub fn new(idx: Idx, size: IPoint) -> Level {
        Level {
            idx,
            size,
            entities: HashMap::new()
        }
    }
    pub fn add_entity(&mut self, object: Object, position: IPoint) {
        let idx = object.get_idx();
        let e = Entity {
            object,
            position,
            level: self.idx,
        };
        self.entities.insert(idx, e);
    }
    pub fn remove_entity(&mut self, idx: &Idx) -> Option<Entity> {
        self.entities.remove(idx)
    }
    pub fn move_entity(&mut self, idx: &Idx, position: IPoint) {
        self.remove_entity(idx).map(|entity| self.add_entity(entity.object, position));
    }
    pub fn get_entity(&self, idx: &Idx) -> Option<&Entity> {
        self.entities.get(idx)
    }
    pub fn get_mut_entity(&mut self, idx: &Idx) -> Option<&mut Entity> {
        self.entities.get_mut(idx)
    }
    pub fn get_tile(&self, position: &IPoint) -> Vec<&Entity> {
        let mut result = Vec::new();
        for (_key, entity) in self.entities.iter() {
            if &entity.position == position {
                result.push(entity);
            }
        }
        result
    }
    pub fn build_layout(&self) -> Layout {
        let mut result = HashMap::new();
        for p in self.size.zrange().iterator() {
            result.insert(p, Vec::new());
        }
        for (_key, entity) in self.entities.iter() {
            result.get_mut(&entity.position).unwrap().push(entity);
        }
        return Layout {
            tiles: result,
            size: self.size
        };
    }
}
