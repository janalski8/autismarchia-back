use utils::ipoint::IPoint;
use state::world::World;
use design::blueprint::Blueprint;
use player::player::PlayerData;
use state::object::Pixel;
use state::object::Object;
use serde_json;
use objects::player::Player;
use print_raw;
use print;


pub struct GameState {
    pub game: World,
    pub player: PlayerData
}

impl GameState {
    pub fn new(size: IPoint) -> GameState {
        let mut game = World::new();
        let idx = game.next_id();

        let level = Blueprint::example(size).level_from_blueprint(&mut game);

        let empty_tile = level.tiles().iter().filter(|(_p, tile)|
            !tile.iter().any(|e| e.object().is_blocking())
        ).next();
        let empty_pos = match empty_tile {
            None => panic!(),
            Some(p) => p.0
        };
        level.add_entity(Box::new(Player::new(idx)), *empty_pos);

        GameState {
            game,
            player: PlayerData::new(idx)
        }
    }
    pub fn process_key(&mut self, string: &str) {
        //print("press key".to_string());
        self.player.process_key(&mut self.game, string);
    }
    pub fn get_view(&mut self) -> String {
        let empty = &Pixel::empty();
        let view = self.player.build_view(&self.game);
        let mut result = Vec::new();
        for y in 0..view.size.y {
            let mut row = Vec::new();
            for x in 0..view.size.x {
                row.push(view.tiles.get(&IPoint{x, y}).unwrap_or(empty));
            }
            result.push(row);
        }
        serde_json::to_string(&result).unwrap()
    }
    pub fn get_messages(&mut self, count: i32) -> String {
        let messages = self.player.get_messages();
        let start_idx = (messages.len() as i32 - count).max(0) as usize;
        let part = &messages[start_idx..];
        serde_json::to_string(part).unwrap()
    }
}