use state::object::Object;
use state::object::Idx;
use state::object::Pixel;
use state::object::Icon;
use state::object::Color;

pub struct Floor {
    idx: Idx
}

impl Floor {
    pub fn new(idx: Idx) -> Floor {
        Floor {idx}
    }
}

impl Object for Floor {
    fn get_idx(&self) -> Idx {
        self.idx
    }

    fn get_pixel(&self) -> Pixel {
        Pixel(Icon::Floor, Color(255, 255, 255))
    }

    fn get_ordinal(&self) -> i32 {
        -1024
    }

    fn is_environment(&self) -> bool {
        true
    }

    fn is_blocking(&self) -> bool {
        false
    }

    fn is_opaque(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        "Floor"
    }
}