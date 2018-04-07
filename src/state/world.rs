use std::collections::HashMap;
use std::collections::hash_map::Entry;
use design::blueprint::Blueprint;
use state::level::Level;
use state::object::{Idx, Object};
use utils::ipoint::IPoint;

type IdxSouce = u32;
pub struct World {
    pub levels: HashMap<Idx, Level>,
    pub next_id: IdxSouce,
}
trait Source { fn next(&mut self) -> Idx; }
trait Levels {
    fn add_level(&mut self, idx: Idx, size: IPoint) -> &mut Level;
}

impl Source for IdxSouce {
    fn next(&mut self) -> Idx {
        *self = *self + 1;
        *self - 1
    }
}

impl Levels for HashMap<Idx, Level> {
    fn add_level(&mut self, idx: Idx, size: IPoint) -> &mut Level {
        let l = Level {
            idx,
            size,
            entities: HashMap::new(),
        };
        match self.entry(l.idx) {
            Entry::Occupied(_) => panic!(),
            Entry::Vacant(e) => {
                e.insert(l)
            }
        }
    }
}

impl World {
    pub fn new() -> World {
        World {
            levels: HashMap::new(),
            next_id: 0,
        }
    }
    pub fn next_id(&mut self) -> Idx {
        self.next_id.next()
    }
    pub fn new_level(&mut self, size: IPoint) -> &mut Level {
        let result = Level::new(self.next_id.next(), size);
        match self.levels.entry(result.idx) {
            Entry::Occupied(_) => panic!(),
            Entry::Vacant(e) => e.insert(result),
        }
    }
    pub fn remove_lvl(&mut self, level: &Level) -> Option<Level> {
        self.levels.remove(&level.idx)
    }
    pub fn get_mut_entity_level<'a>(&'a mut self, idx: &Idx) -> Option<&'a mut Level> {
        for (_k, value) in self.levels.iter_mut() {
            if value.entities.contains_key(idx) {
                return Some(value);
            }
        }
        None
    }
    pub fn get_entity_level<'a>(&'a self, idx: &Idx) -> Option<&'a Level> {
        for (_k, value) in self.levels.iter() {
            if value.entities.contains_key(idx) {
                return Some(value);
            }
        }
        None
    }
    pub fn get_mut_level(&mut self, idx: &Idx) -> Option<&mut Level> {
        self.levels.get_mut(idx)
    }
    pub fn create_level(&mut self, blueprint: &Blueprint) -> &mut Level {
        let levels = &mut self.levels;
        let source = &mut self.next_id;
        let level = levels.add_level(source.next(), blueprint.size);
        for point in level.size.zrange().iterator() {
            if blueprint.tiles.get(&point).is_none() {
                let object = Object::Wall { idx: source.next() };
                level.add_entity(object, point);
            } else {
                let object = Object::Floor { idx: source.next() };
                level.add_entity(object, point);
            }
        }
        level
    }
}
