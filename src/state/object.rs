use state::context::Action;
use state::context::Context;
use state::context::Effects;
use objects::player::Player;

pub type Idx = u32;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Pixel (pub Icon, pub Color);
impl Pixel {
    pub fn empty() -> Pixel {
        return Pixel(Icon::Empty, Color(255, 255, 255))
    }
    pub fn gray(&self) -> Pixel {
        Pixel(self.0, Color(128, 128, 128))
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Color (pub u8, pub u8, pub u8);

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Icon {
    Wall,
    Player,
    Enemy,
    Floor,
    Empty
}

pub trait Object {
    fn get_idx(&self) -> Idx;
    fn get_pixel(&self) -> Pixel;
    fn get_ordinal(&self) -> i32;
    fn is_environment(&self) -> bool;
    fn is_blocking(&self) -> bool;
    fn is_opaque(&self) -> bool;
    fn name(&self) -> &str;

    fn is_active(&self) -> bool { false }
    fn get_cooldown(&self) -> i64 { i64::max_value() }
    fn lapse_time(&mut self, _interval: i64) { }
    fn update(&self, _context: Context, _effects: &mut Effects) { }
    fn plan_action(&self, _context: Context, _effects: &mut Effects) { }
    fn execute_action(&mut self, _effects: &mut Effects, _action: &Action) { }

    fn as_player(&mut self) -> Option<&mut Player> { None }
}
