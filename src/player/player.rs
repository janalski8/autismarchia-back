use std::collections::HashMap;
use std::collections::hash_map::Entry;
use state::object::Idx;
use state::world::World;
use utils::ipoint::IPoint;
use state::level::Level;
use state::object::Pixel;
use state::context::Action;
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

    pub fn wait<'a>(&mut self, game: &'a mut World) {
        let entity_idx = game.lapse_time().unwrap().object().get_idx();
        if entity_idx != self.player {
            game.invoke_actions(entity_idx);
            self.wait(game)
        }
    }

    pub fn process_key(&mut self, game: &mut World, string: &str) {
        self.wait(game);

        let dir: IPoint = match string {
            "w" => IPoint {x: 0, y: -1},
            "s" => IPoint {x: 0, y: 1},
            "a" => IPoint {x: -1, y: 0},
            "d" => IPoint {x: 1, y: 0},
            _ => return,
        };

        let player = game.get_mut_entity(self.player).unwrap();
        let position = player.position();
        match player.object_mut().as_player() {
            None => {panic!()},
            Some(p) => {
                p.set_action(Action::Walk {
                    idx: self.player,
                    position: dir + position
                });
            },
        }

        /*
        let level = game.get_mut_entity_level(self.player).unwrap();
        let entity = level.get_mut_entity(self.player).unwrap();
        let position = entity.position();
        let new_pos = dir + position;
        if !level.size().zrange().inside(new_pos) {
            self.messages.push("you hit the wall".to_string());
            return;
        }

        let tile = level.get_tile(new_pos).unwrap();
        let obstacle = tile.iter().filter(|e| e.object().is_blocking()).next();
        match obstacle {
            Some(obs) => self.messages.push("you hit the ".to_string() + &obs.object().name()),
            None => entity.object_mut().set_action(Action::Walk{
                idx: self.player,
                position: new_pos,
            })
        }*/
        game.invoke_actions(self.player);
        self.wait(game);
        self.update_memory(game.get_mut_entity_level(self.player).unwrap());
    }

    fn update_memory(&mut self, level: &mut Level) {
        let player = level.get_entity(self.player).unwrap();
        let visible = level.visible_points(player.position(), self.range);
        let pixels: HashMap<IPoint, Pixel> =
            visible.iter()
                .map(|k| (*k, Level::build_mem_pixel(level.tiles().get(k).unwrap())))
                .collect();

        match self.views.entry(level.idx()) {
            Entry::Occupied(mut e) => { e.get_mut().extend(pixels); }
            Entry::Vacant(e) => { e.insert(pixels); }
        };
    }

    pub fn build_view(&self, game: &World) -> View {
        let level = game.get_entity_level(self.player).unwrap();
        let player = level.get_entity(self.player).unwrap();
        let position = player.position();

        let visible = level.visible_points(position, self.range);

        let mut current_pixels: HashMap<IPoint, Pixel> =
            visible.into_iter()
                .map(|p| (p, Level::build_pixel(level.get_tile(p).unwrap())))
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
            size: level.size(),
            tiles: current_pixels,
        }
    }

    pub fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }
}
