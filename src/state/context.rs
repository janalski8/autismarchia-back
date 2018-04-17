use state::object::Idx;
use state::world::World;
use state::level::Level;
use rand::XorShiftRng;
use utils::ipoint::IPoint;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Attack{source: Idx, target: Idx},
    Walk{idx: Idx, position: IPoint},
    Wait{idx: Idx}
}

impl Action {
    pub fn actors(&self) -> Vec<Idx> {
        match self {
            Action::Attack { source, target } => { vec![*source, *target] },
            Action::Walk { idx, .. } => { vec![*idx] },
            Action::Wait { idx } => { vec![*idx] },
        }
    }
}

pub struct Context<'a> {
    pub world: &'a World,
    pub level: &'a Level,
    pub position: IPoint,
}

pub struct Effects {
    pub rand: XorShiftRng,
    pub messages: Vec<String>,
    pub actions: Vec<Action>
}