use std::collections::HashMap;
use std::collections::hash_map::Entry;
use state::level::Level;
use state::object::Idx;
use std::cell::Cell;
use state::level::Entity;
use state::context::Context;
use rand::XorShiftRng;
use state::context::Action;
use rand::SeedableRng;
use rand::Rng;
use state::context::Effects;
use print;

pub struct World {
    rand: XorShiftRng,
    levels: HashMap<Idx, Level>,
    next_id: Cell<Idx>,
    time: i64,
}

impl World {
    pub fn new() -> World {
        World {
            rand: XorShiftRng::from_seed([1, 2, 3, 4]),
            levels: HashMap::new(),
            next_id: Cell::new(0),
            time: 0,
        }
    }

    pub fn levels(&self) -> &HashMap<Idx, Level> {
        &self.levels
    }

    pub fn next_id(&self) -> Idx {
        let next = self.next_id.get();
        self.next_id.set(next + 1);
        return next;
    }

    pub fn add_level(&mut self, level: Level) -> &mut Level {
        match self.levels.entry(level.idx()) {
            Entry::Occupied(_) => panic!(),
            Entry::Vacant(e) => e.insert(level),
        }
    }
    pub fn remove_lvl(&mut self, idx: Idx) -> Option<Level> {
        self.levels.remove(&idx)
    }

    pub fn get_mut_entity(&mut self, idx: Idx) -> Option<&mut Entity> {
        self.levels.iter_mut()
            .map(|(_, lvl)| lvl.get_mut_entity(idx))
            .find(|e| e.is_some())
            .and_then(|e| e)
    }
    pub fn get_entity(&self, idx: Idx) -> Option<&Entity> {
        self.levels.iter()
            .map(|(_, lvl)| lvl.get_entity(idx))
            .find(|e| e.is_some())
            .and_then(|e| e)
    }

    pub fn get_mut_entity_level(&mut self, idx: Idx) -> Option<&mut Level> {
        self.levels.iter_mut()
            .find(|(_, lvl)| lvl.get_entity(idx).is_some())
            .map(|t| t.1)
    }
    pub fn get_entity_level(&self, idx: Idx) -> Option<&Level> {
        self.levels.iter()
            .find(|(_, lvl)| lvl.get_entity(idx).is_some())
            .map(|t| t.1)
    }

    pub fn lapse_time(&mut self) -> Option<&mut Entity> {
        //print("lapse time".to_string());
        let interval = self.levels.iter().flat_map(
            |(_lvl_idx, lvl)| lvl.get_entities().map(
                |entity| entity.object().get_cooldown()
            )
        ).min().unwrap();

        self.levels.iter_mut().for_each(
            |(_, lvl)| lvl.get_mut_entities().for_each(
                |entity| entity.object_mut().lapse_time(interval)
            )
        );

        self.time += interval;

        self.levels.iter_mut().flat_map(
            |(_lvl_idx, lvl)| lvl.get_mut_entities()
        ).find(|entity| entity.object().get_cooldown() == 0)
    }

    pub fn build_context(&self, entity: &Entity) -> Context {
        Context {
            world: &self,
            level: self.levels.get(&entity.level()).unwrap(),
            position: entity.position(),
        }
    }

    pub fn build_effects(&mut self) -> Effects {
        let rand = XorShiftRng::from_seed([
            self.rand.next_u32(),
            self.rand.next_u32(),
            self.rand.next_u32(),
            self.rand.next_u32()
        ]);
        let actions = Vec::new();
        let messages = Vec::new();
        Effects { rand, actions, messages }
    }

    pub fn invoke_actions(&mut self, entity_idx: Idx) -> Vec<String> {
        let mut effects = self.build_effects();

        let entity = self.get_entity(entity_idx).unwrap();
        let context = self.build_context(entity);
        entity.object().plan_action(context, &mut effects);

        while let Some(action) = effects.actions.pop() {
            //print(format!("action: {:?}", action));
            self.execute_action(&mut effects, action);

            self.levels.iter()
                .flat_map(|(_, lvl)| lvl.get_entities())
                .filter(|e| e.object().is_active())
                .for_each(|entity| entity.object().update(self.build_context(entity), &mut effects));
        }

        effects.messages
    }
    pub fn execute_action(&mut self, effects: &mut Effects, action: Action) {
        match action {
            Action::Attack { target, source } => {
                let t = self.get_mut_entity(target).unwrap();
                t.object_mut().execute_action(effects, &action);
                let s = self.get_mut_entity(source).unwrap();
                s.object_mut().execute_action(effects, &action);
            }
            Action::Walk { idx, position } => {
                let mut lvl = self.get_mut_entity_level(idx).unwrap();
                if !lvl.get_tile(position).map(Level::is_blocking).unwrap_or(true) {
                    lvl.move_entity(idx, position);
                }
            }
            Action::Wait { idx } => {
                let mut e = self.get_mut_entity(idx).unwrap();
                e.object_mut().execute_action(effects, &action);
            }
        };
    }
}
