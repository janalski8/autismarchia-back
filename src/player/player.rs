use std::collections::HashMap;
use std::collections::hash_map::Entry;
use state::object::{Idx, TObject};
use state::world::World;
use utils::ipoint::IPoint;
use state::level::Level;
use state::object::Pixel;
use std::collections::HashSet;
use logic::visibility::visibility_set;
use utils::point::Point;
use state::level::Entity;


pub struct View {
    pub tiles: HashMap<IPoint, Pixel>,
    pub size: IPoint,
}

pub struct PlayerData {
    pub player: Idx,
    pub views: HashMap<Idx, HashMap<IPoint, Pixel>>,
    pub messages: Vec<String>,
    range: f32,
}

impl PlayerData {
    pub fn new(idx: Idx) -> PlayerData {
        PlayerData {
            player: idx,
            views: HashMap::new(),
            messages: Vec::new(),
            range: 8.0,
        }

    }
    pub fn process_key(&mut self, game: &mut World, string: &str) {
        let dir: IPoint = match string {
            "w" => IPoint {x: 0, y: -1},
            "s" => IPoint {x: 0, y: 1},
            "a" => IPoint {x: -1, y: 0},
            "d" => IPoint {x: 1, y: 0},
            _ => return,
        };

        let level = match game.get_mut_entity_level(&self.player) {
            Some(lvl) => lvl,
            None => {
                self.messages.push("you're not in the game!".to_string());
                return;
            }
        };

        let ref entity = match level.get_entity(&self.player) {
            Some(entity) => entity,
            _ => {
                self.messages.push("you can't move!".to_string());
                return;
            }
        };

        let position = *entity.position();
        let new_pos = level.size().zrange().clip(&(dir + position));
        if new_pos == position {
            self.messages.push("you hit the wall".to_string());
            return;
        }

        let tile = level.get_tile(&new_pos).unwrap();
        let obstacle = tile.iter().filter(|e| e.object().is_blocking()).next();
        match obstacle {
            Some(obs) => self.messages.push("you hit the ".to_string() + &obs.object().to_string()),
            None => self.process_move(level, &new_pos),
        }
    }
    fn process_move(&mut self, level: &mut Level, new_pos: &IPoint) {
        level.move_entity(self.player, *new_pos);
        self.update_memory(level);
    }
    fn update_memory(&mut self, level: &mut Level) {
        let player = level.get_entity(&self.player).unwrap();
        let visible = PlayerData::visible_points(level, &player.position(), self.range);
        let pixels: HashMap<IPoint, Pixel> =
            visible.iter()
                .map(|k| (*k, PlayerData::build_mem_pixel(level.tiles().get(k).unwrap())))
                .collect();

        match self.views.entry(*level.idx()) {
            Entry::Occupied(mut e) => { e.get_mut().extend(pixels); }
            Entry::Vacant(e) => { e.insert(pixels); }
        };
    }
    pub fn build_view(&self, game: &World) -> View {
        let level = game.get_entity_level(&self.player).unwrap();
        let player = level.get_entity(&self.player).unwrap();
        let position = player.position();

        let visible = PlayerData::visible_points(level, &position, self.range);

        let mut current_pixels: HashMap<IPoint, Pixel> =
            visible.iter()
                .map(|p| (*p, PlayerData::build_pixel(level.get_tile(p).unwrap())))
                .collect();

        match self.views.get(&level.idx()) {
            None => (),
            Some(mem_pixels) => {
                for (k, mem) in mem_pixels {
                    match current_pixels.entry(*k) {
                        Entry::Occupied(_e) => (),
                        Entry::Vacant(e) => {e.insert(*mem); ()},
                    };
                }
            }
        };

        View {
            size: *level.size(),
            tiles: current_pixels,
        }
    }
    fn visible_points(level: &Level, pos: &IPoint, range: f32) -> HashSet<IPoint> {
        let transparent: HashSet<IPoint> =
            level.tiles().iter()
                .filter(|(k, vec)| k.dist(pos) < range + 2.0 && PlayerData::is_transparent(vec))
                .map(|(k, _vec)| *k)
                .collect();
        visibility_set(&transparent, level.size(), pos, range)
    }
    fn build_mem_pixel(tile: &Vec<Entity>) -> Pixel {
        tile.iter()
            .filter(|e| e.object().is_solid())
            .max_by_key(|e| e.object().get_ordinal())
            .map_or(Pixel::empty(), |o| o.object().get_pixel().gray())
    }
    fn build_pixel(tile: &Vec<Entity>) -> Pixel {
        tile.iter()
            .max_by_key(|e| e.object().get_ordinal())
            .map_or(Pixel::empty(), |o| o.object().get_pixel())
    }
    fn is_transparent(tile: &Vec<Entity>) -> bool {
        tile.iter()
            .find(|e| e.object().is_opaque())
            .is_none()
    }
    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

}
