use state::object::Object;
use state::object::Idx;
use state::object::Pixel;
use state::object::Icon;
use state::object::Color;

pub struct Wall {
    idx: Idx
}

impl Wall {
    pub fn new(idx: Idx) -> Wall {
        Wall {idx}
    }
}

impl Object for Wall {
    fn get_idx(&self) -> Idx {
        self.idx
    }

    fn get_pixel(&self) -> Pixel {
        Pixel(Icon::Wall, Color(255, 255, 255))
    }

    fn get_ordinal(&self) -> i32 {
        0
    }

    fn is_environment(&self) -> bool {
        true
    }

    fn is_blocking(&self) -> bool {
        true
    }

    fn is_opaque(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        "Wall"
    }
}