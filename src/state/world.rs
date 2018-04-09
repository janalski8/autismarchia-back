use std::collections::HashMap;
use std::collections::hash_map::Entry;
use design::blueprint::Blueprint;
use state::level::Level;
use state::object::{Idx, Object};
use utils::ipoint::IPoint;

type IdxSouce = u32;
trait Source { fn next(&mut self) -> Idx; }
impl Source for IdxSouce {
    fn next(&mut self) -> Idx {
        *self = *self + 1;
        *self - 1
    }
}

trait Levels {
    fn add_level(&mut self, idx: Idx, size: IPoint) -> &mut Level;
}
impl Levels for HashMap<Idx, Level> {
    fn add_level(&mut self, idx: Idx, size: IPoint) -> &mut Level {
        let level = Level::new(idx, size);
        match self.entry(*level.idx()) {
            Entry::Occupied(_) => panic!(),
            Entry::Vacant(e) => e.insert(level)
        }
    }
}

pub struct World {
    levels: HashMap<Idx, Level>,
    next_id: IdxSouce,
}
impl World {
    pub fn new() -> World {
        World {
            levels: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn levels(&self) -> &HashMap<Idx, Level> {
        &self.levels
    }

    pub fn next_id(&mut self) -> Idx {
        self.next_id.next()
    }

    pub fn add_level(&mut self, size: IPoint) -> &mut Level {
        let result = Level::new(self.next_id.next(), size);
        match self.levels.entry(*result.idx()) {
            Entry::Occupied(_) => panic!(),
            Entry::Vacant(e) => e.insert(result),
        }
    }
    pub fn remove_lvl(&mut self, level: &Level) -> Option<Level> {
        self.levels.remove(&level.idx())
    }

    pub fn get_mut_entity_level(& mut self, idx: &Idx) -> Option<&mut Level> {
        self.levels.iter_mut().find(|(_, lvl)| lvl.get_entity(idx).is_some()).map(|t| t.1)
    }
    pub fn get_entity_level(& self, idx: &Idx) -> Option<&Level> {
        self.levels.iter().find(|(_, lvl)| lvl.get_entity(idx).is_some()).map(|t| t.1)
    }

    pub fn level_from_blueprint(&mut self, blueprint: &Blueprint) -> &mut Level {
        let levels = &mut self.levels;
        let source = &mut self.next_id;
        let level = levels.add_level(source.next(), blueprint.size);
        for point in level.size().zrange().iter() {
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
