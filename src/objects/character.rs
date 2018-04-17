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

struct Character {
    enemy: Idx,
    idx: Idx,

    range: f32,
    wait_speed: i64,
    walk_speed: i64,
    attack_speed: i64,

    health: Cell<i32>,
    cooldown: Cell<i64>,
    wait_time: Cell<i64>,
    last_enemy: Cell<Option<(Idx, IPoint)>>,
}

impl Object for Character {
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
        if self.health.get() > 0 {
            self.cooldown.get().max(self.wait_time.get())
        } else {
            i64::max_value()
        }
    }

    fn lapse_time(&mut self, interval: i64) {
        let new_cooldown = 0.max(self.cooldown.get() - interval);
        let new_wait_time = 0.max(self.wait_time.get() - interval);
        self.cooldown.set(new_cooldown);
        self.wait_time.set(new_wait_time);
    }

    fn update(&self, context: Context, _effects: &mut Effects) {
        let enemy = context.level.visible_points(context.position, self.range).into_iter()
            .flat_map(|p| context.level.get_tile(p).unwrap().iter())
            .filter(|e| self.is_enemy(e))
            .min_by_key(|e| OrderedFloat(e.position().dist(context.position)));
        if let Some(e) = enemy {
            self.last_enemy.set(Some((e.object().get_idx(), e.position())));
            self.wait_time.set(0);
        };
    }

    fn plan_action(&self, context: Context, effects: &mut Effects) {
        let mut ctx = context;
        let maybe_action: Option<Action>;
        match self.last_enemy.get() {
            None => maybe_action = self.plan_walk_random(&mut ctx, effects),
            Some((enemy_idx, enemy_pos)) => {
                if enemy_pos.neumann_dist(ctx.position) > 1 {
                    maybe_action = self.plan_walk_towards(enemy_pos, &mut ctx, effects);
                } else {
                    maybe_action = Some(Action::Attack{ target: enemy_idx, source: self.get_idx() })
                }
            }
        }
        match maybe_action {
            None => {effects.actions.push(Action::Wait {idx: self.idx})},
            Some(a) => {effects.actions.push(a)},
        }
    }

    fn execute_action(&mut self, _effects: &mut Effects, action: &Action) {
        match action {
            Action::Attack { target, source } => {
                if *source == self.idx {
                    self.cooldown.set(self.cooldown.get() + self.attack_speed)
                }
                if *target == self.idx {
                    self.health.set(0.max(self.health.get() - 1))
                }
            },
            Action::Walk { idx, .. } => {
                self.cooldown.set(self.cooldown.get() + self.walk_speed)

            }
            Action::Wait { idx, .. } => {
                self.wait_time.set(self.wait_time.get() + self.wait_speed);
            }
        }
    }
}

impl Character {
    fn is_enemy(&self, entity: &Entity) -> bool {
        entity.object().get_idx() == self.enemy
    }

    fn walk_options(&self, context: &mut Context) -> Vec<IPoint> {
        context.position.neumann_surrounding()
            .into_iter()
            .filter(|p| !context.level
                .get_tile(context.position + *p)
                .map(Level::is_blocking)
                .unwrap_or(true)
            ).collect()
    }

    fn plan_walk_random(&self, context: &mut Context, effects: &mut Effects) -> Option<Action> {
        let mut options = self.walk_options(context);
        effects.rand.shuffle(&mut options);
        options.into_iter().last().map(|position| Action::Walk { idx: self.get_idx(), position })
    }

    fn plan_walk_towards(&self, target: IPoint, context: &mut Context, _effects: &mut Effects) -> Option<Action> {
        let direction = target - context.position;
        if direction == IPoint::zero() {
            self.last_enemy.set(None);
            return None;
        }

        let abs_dir = direction.abs();
        let dir_result = if abs_dir.x > abs_dir.y {
            IPoint{x: direction.x.signum(), y: 0}
        } else {
            IPoint{x: 0, y: direction.y.signum()}
        };
        let position = dir_result + context.position;
        let options = self.walk_options(context);
        if options.contains(&position) {
            Some(Action::Walk { idx: self.get_idx(), position })
        } else {
            self.last_enemy.set(None);
            return None;
        }
    }
}
