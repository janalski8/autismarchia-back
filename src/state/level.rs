use std::collections::HashMap;
use std::collections::hash_map::Entry;
use state::object::{Idx};
use utils::ipoint::IPoint;
use state::object::Object;
use state::object::Pixel;
use std::collections::HashSet;
use logic::visibility::visibility_set;
use std::ops::Deref;
use std::ops::DerefMut;
use utils::point::Point;

pub struct Level {
    idx: Idx,
    size: IPoint,
    tiles: HashMap<IPoint, Vec<Entity>>,
    positions: HashMap<Idx, IPoint>
}

pub struct Entity {
    object: Box<Object>,
    level: Idx,
    position: IPoint
}

impl Entity {
    fn new(object: Box<Object>, level: Idx, position: IPoint) -> Entity {
        Entity {object, level, position}
    }
    pub fn level(&self) -> Idx {
        self.level
    }
    pub fn position(&self) -> IPoint {
        self.position
    }
    pub fn object(&self) -> &Object {
        self.object.deref()
    }
    pub fn object_mut(&mut self) -> &mut Object {
        self.object.deref_mut()
    }
}

impl Level {
    pub fn new(idx: Idx, size: IPoint) -> Level {
        let mut tiles = HashMap::new();
        for pos in size.zrange().iter() {
            tiles.insert(pos, Vec::new());
        }
        Level {idx, size, tiles, positions: HashMap::new()}
    }

    pub fn idx(&self) -> Idx {
        self.idx
    }
    pub fn size(&self) -> IPoint {
        self.size
    }
    pub fn tiles(&self) -> &HashMap<IPoint, Vec<Entity>> {
        &self.tiles
    }
    pub fn positions(&self) -> &HashMap<Idx, IPoint> {
        &self.positions
    }

    pub fn add_entity(&mut self, object: Box<Object>, position: IPoint) -> &mut Entity {
        let idx = object.get_idx();
        match self.positions.entry(idx) {
            Entry::Occupied(_) => panic!(),
            Entry::Vacant(e) => e.insert(position),
        };

        let entity = Entity::new(object, self.idx, position);
        let tile = self.tiles.get_mut(&position).unwrap();
        tile.push(entity);
        tile.last_mut().unwrap()
    }
    pub fn remove_entity(&mut self, idx: Idx) -> Option<Entity> {
        self.positions.remove(&idx)
            .map(|pos| {
                let tile = self.tiles.get_mut(&pos).unwrap();
                let index = tile.iter().position(|e| e.object.get_idx() == idx).unwrap();
                tile.remove(index)
            })
    }
    pub fn move_entity(&mut self, idx: Idx, new_position: IPoint) -> Option<&mut Entity> {
        let removed = self.remove_entity(idx);
        match removed {
            None => None,
            Some(entity) => Some(self.add_entity(entity.object, new_position)),
        }
    }

    pub fn get_tile(&self, position: IPoint) -> Option<&Vec<Entity>> {
        self.tiles.get(&position)
    }
    pub fn get_mut_tile(&mut self, position: IPoint) -> Option<&mut [Entity]> {
        self.tiles.get_mut(&position).map(|v| v.as_mut_slice())
    }
    pub fn get_position(&self, idx: Idx) -> Option<IPoint> {
        self.positions.get(&idx).map(|p| *p)
    }

    pub fn get_entities(& self) -> impl Iterator<Item=&Entity> {
        self.tiles.iter().flat_map(|(_p, v)| v.iter())
    }
    pub fn get_mut_entities(&mut self) -> impl Iterator<Item=&mut Entity> {
        self.tiles.iter_mut().flat_map(|(_p, v)| v.iter_mut())
    }

    pub fn get_entity(&self, idx: Idx) -> Option<&Entity> {
        self.get_position(idx)
            .and_then(|pos| self.tiles.get(&pos))
            .and_then(|tile| tile.iter().find(|e| e.object.get_idx() == idx))
    }
    pub fn get_mut_entity<'a>(&'a mut self, idx: Idx) -> Option<&'a mut Entity> {
        let tile = match self.positions.get(&idx) {
            None => None,
            Some(p) => self.tiles.get_mut(p),
        };
        tile.and_then(|t| t.iter_mut().find(|e| e.object.get_idx() == idx))
    }

    pub fn visible_points(&self, pos: IPoint, range: f32) -> HashSet<IPoint> {
        let transparent: HashSet<IPoint> =
            self.tiles().iter()
                .filter(|(k, vec)| pos.dist(**k) <= range && Level::is_transparent(vec))
                .map(|(k, _vec)| *k)
                .collect();
        visibility_set(&transparent, self.size(), pos, range)
    }

    pub fn build_mem_pixel(tile: &Vec<Entity>) -> Pixel {
        tile.iter()
            .filter(|e| e.object().is_environment())
            .max_by_key(|e| e.object().get_ordinal())
            .map_or(Pixel::empty(), |o| o.object().get_pixel().gray())
    }
    pub fn build_pixel(tile: &Vec<Entity>) -> Pixel {
        tile.iter()
            .max_by_key(|e| e.object().get_ordinal())
            .map_or(Pixel::empty(), |o| o.object().get_pixel())
    }
    pub fn is_transparent(tile: &Vec<Entity>) -> bool {
        tile.iter()
            .find(|e| e.object().is_opaque())
            .is_none()
    }
    pub fn is_blocking(tile: &Vec<Entity>) -> bool {
        tile.iter()
            .find(|e| e.object().is_blocking())
            .is_some()
    }
}
