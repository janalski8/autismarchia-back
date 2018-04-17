extern crate rand;

use core::cmp;
use std::collections::HashMap;
use rand::Rng;
use rand::XorShiftRng;
use rand::SeedableRng;
use utils::ipoint::IPoint;
use utils::irange::IRange;
use utils::pointrng::PointRng;
use state::world::World;
use state::level::Level;
use objects::wall::Wall;
use objects::floor::Floor;

#[derive(Debug)]
pub enum Tile {
    Room
}
pub type TileMap = HashMap<IPoint, Tile>;
pub trait Tiles {
    fn build_room(&mut self, room: IRange);
    fn build_xline(&mut self, x: i32, y1: i32, y2: i32);
    fn build_yline(&mut self, x1: i32, x2: i32, y: i32);
}
impl Tiles for TileMap {
    fn build_room(&mut self, room: IRange) {
        for point in room.iter() {
            self.insert(point, Tile::Room);
        }
    }
    fn build_xline(&mut self, x: i32, y1: i32, y2: i32) {
        let (oy1, oy2) = if y1 < y2 {(y1, y2)} else {(y2, y1)};
        for y in oy1..oy2+1 {
            self.insert(IPoint {x, y}, Tile::Room);
        }
    }
    fn build_yline(&mut self, x1: i32, x2: i32, y: i32) {
        let (ox1, ox2) = if x1 < x2 {(x1, x2)} else {(x2, x1)};
        for x in ox1..ox2+1 {
            self.insert(IPoint {x, y}, Tile::Room);
        }
    }
}


pub struct Blueprint {
    pub size: IPoint,
    pub rooms: Vec<IRange>,
    pub tiles: TileMap,
    pub random: XorShiftRng
}
impl Blueprint {
    pub fn example(size: IPoint) -> Blueprint {
        let mut bp = Blueprint::new(size);
        for _ in 0..7 {
            bp.try_add_room(IPoint { x: 3, y: 3 }.range(IPoint { x: 12, y: 12 }), 5);
        }
        for _ in 0..4 {
            bp.try_add_room(IPoint { x: 1, y: 1 }.range(IPoint { x: 2, y: 2 }), 5);
        }
        bp.build_rooms();
        bp.connect_tree();
        bp
    }
    pub fn new(size: IPoint) -> Blueprint {
        let seed: [u32; 4] = [2, 3, 6, 5];
        Blueprint {
            size,
            rooms: Vec::new(),
            tiles: HashMap::new(),
            random: XorShiftRng::from_seed(seed),
        }
    }
    pub fn build_rooms(&mut self) {
        let rooms = &mut self.rooms;
        let tiles = &mut self.tiles;
        for room in rooms {
            tiles.build_room(*room);
        }
    }
    pub fn try_add_room(&mut self, size_range: IRange, mindist: i32) {
        let mut room = None;
        for _ in 1..100 {
            let size = self.random.gen_in_range(size_range);
            let end = self.random.gen_in_range(size.range(self.size));
            let start = end - size;
            let newroom = IRange {start, end};
            let dist = self.rooms.iter().fold(
                i32::max_value(),
                |acc, x| cmp::min(acc, newroom.rdist(*x))
            );
            if dist >= mindist {
                room = Some(newroom);
                break;
            }
        }
        if let Some(r) = room {
            self.rooms.push(r);
        }
    }
    pub fn connect_points(&mut self, p1: IPoint, p2: IPoint) {
        let xeq = p1.x == p2.x;
        let yeq = p1.y == p2.y;
        if xeq && yeq {
            self.tiles.insert(p1, Tile::Room);
        } else if xeq {
            self.tiles.build_xline(p1.x, p1.y, p2.y);
        } else if yeq {
            self.tiles.build_yline(p1.x, p2.x, p2.y);
        } else {
            let pi;
            if self.random.gen_range(0, 2) == 1 {
                pi = IPoint {x: p1.x, y: p2.y};
            } else {
                pi = IPoint {x: p2.x, y: p1.y};
            }
            self.connect_points(p1, pi);
            self.connect_points(pi, p2);
        }
    }
    pub fn connect_tree(&mut self) {

        if self.rooms.len() == 0 {
            return
        }

        let len = self.rooms.len();
        let mut tree: Vec<usize> = self.rooms.iter().map(|_x| len).collect();
        tree[0] = 0;

        for _ in 1..len {
            let mut best_dist: i32 = i32::max_value();
            let mut best_child = 0;
            let mut best_parent = 0;
            for (parent, _) in tree.iter().enumerate().filter(|&(_i, &x)| x < len) {
                for (child, _) in tree.iter().enumerate().filter(|&(_i, &x)| x == len) {
                    let dist = (&self.rooms[child]).rdist(self.rooms[parent]);
                    if dist < best_dist {
                        best_dist = dist;
                        best_parent = parent;
                        best_child = child;
                    }
                }
            }
            tree[best_child] = best_parent;
        }
        for (child, &parent) in tree.iter().enumerate() {
            let p1 = self.rooms[child].center().floor();
            let p2 = self.rooms[parent].center().floor();
            self.connect_points(p1, p2);
        }
    }

    pub fn level_from_blueprint<'a>(&self, world: &'a mut World) -> &'a mut Level {
        let mut level = Level::new(world.next_id(), self.size);
        for point in self.size.zrange().iter() {
            if self.tiles.get(&point).is_none() {
                let object = Wall::new(world.next_id());
                level.add_entity(Box::new(object), point);
            } else {
                let object = Floor::new(world.next_id());
                level.add_entity(Box::new(object), point);
            }
        }
        world.add_level(level)
    }
}
