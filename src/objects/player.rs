use state::object::{Idx, Color, Pixel, Icon, Object};
use state::level::{Entity, Level};
use utils::ipoint::IPoint;
use utils::point::Point;
use ordered_float::OrderedFloat;
use state::context::Context;
use state::context::Action;
use rand::Rng;
use std::cell::Cell;
use state::context::Effects;

pub struct Player {
    idx: Idx,

    range: f32,
    wait_speed: i64,
    walk_speed: i64,
    attack_speed: i64,

    health: i32,
    cooldown: i64,
    wait_time: i64,
    action: Action,
}

impl Object for Player {
    fn get_idx(&self) -> Idx {
        self.idx
    }

    fn get_pixel(&self) -> Pixel {
        return Pixel(Icon::Player, Color(255, 255, 255))
    }

    fn get_ordinal(&self) -> i32 {
        0
    }

    fn is_environment(&self) -> bool {
        false
    }

    fn is_blocking(&self) -> bool {
        true
    }

    fn is_opaque(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        "Character"
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_cooldown(&self) -> i64 {
        self.cooldown.max(self.wait_time)
    }

    fn lapse_time(&mut self, interval: i64) {
        self.cooldown = 0.max(self.cooldown - interval);
        self.wait_time = 0.max(self.wait_time - interval);
    }

    fn update(&self, context: Context, _effects: &mut Effects) {
    }

    fn plan_action(&self, context: Context, effects: &mut Effects) {
        effects.actions.push(self.action);
    }

    fn execute_action(&mut self, _effects: &mut Effects, action: &Action) {
        match action {
            Action::Attack { target, source } => {
                if *source == self.idx {
                    self.cooldown = self.cooldown + self.attack_speed
                }
                if *target == self.idx {
                    self.health = 0.max(self.health - 1)
                }
            },
            Action::Walk { idx, .. } => {
                self.cooldown = self.cooldown + self.walk_speed

            }
            Action::Wait { idx, .. } => {
                self.wait_time = self.wait_speed;
            }
        }
    }

    fn as_player(&mut self) -> Option<&mut Player> {
        return Some(self)
    }

}

impl Player {
    pub fn set_action(&mut self, action: Action) {
        self.action = action;
    }
    pub fn new(idx: Idx) -> Player {
        Player {
            idx,

            range: 10.0,
            wait_speed: 10,
            walk_speed: 10,
            attack_speed: 10,

            health: 10,
            cooldown: 0,
            wait_time: 0,
            action: Action::Wait{idx},
        }
    }
}
