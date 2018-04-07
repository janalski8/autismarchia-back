use std::fmt::{Display, Formatter, Error};

pub type Idx = u32;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Pixel (Icon, Color);
impl Pixel {
    pub fn empty() -> Pixel {
        return Pixel(Icon::Empty, Color(255, 255, 255))
    }
    pub fn gray(&self) -> Pixel {
        Pixel(self.0, Color(128, 128, 128))
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Color (u8, u8, u8);

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Icon {
    Wall,
    Player,
    Floor,
    Empty
}
pub trait TObject {
    fn get_idx(&self) -> Idx;
    fn get_pixel(&self) -> Pixel;
    fn get_ordinal(&self) -> i32;
    fn is_solid(&self) -> bool;
    fn is_blocking(&self) -> bool;
    fn is_opaque(&self) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub enum Object {
    Player {idx: Idx},
    Wall {idx: Idx},
    Floor {idx: Idx}
}

impl TObject for Object {
    fn get_idx(&self) -> Idx {
        match self {
            &Object::Player{idx} => idx,
            &Object::Wall{idx} => idx,
            &Object::Floor{idx} => idx
        }
    }
    fn get_pixel(&self) -> Pixel {
        match self {
            &Object::Player{..} => Pixel(Icon::Player, Color(255, 255, 255)),
            &Object::Wall{..} => Pixel(Icon::Wall, Color(255, 255, 255)),
            &Object::Floor{..} => Pixel(Icon::Floor, Color(255, 255, 255))
        }
    }
    fn get_ordinal(&self) -> i32 {
        match self {
            &Object::Player{..} => 1,
            &Object::Wall{..} => 0,
            &Object::Floor{..} => -1,
        }
    }
    fn is_solid(&self) -> bool { // can you see it after you've seen int
        match self {
            &Object::Player{..} => false,
            &Object::Wall{..} => true,
            &Object::Floor{..} => true,
        }
    }
    fn is_blocking(&self) -> bool { //can you move through it
        match self {
            &Object::Player{..} => true,
            &Object::Wall{..} => true,
            &Object::Floor{..} => false,
        }
    }
    fn is_opaque(&self) -> bool {
        match self {
            &Object::Player{..} => false,
            &Object::Wall{..} => true,
            &Object::Floor{..} => false,
        }
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}



