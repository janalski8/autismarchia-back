use std::ops::{Add, Sub, Neg, Mul};
use utils::ipoint::IPoint;
use utils::point::Point;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct FPoint {
    pub x: f32,
    pub y: f32,
}

impl Debug for FPoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(x:{:.2}, y:{:.2})", self.x, self.y)
    }
}
impl Display for FPoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(x:{:.2}, y:{:.2})", self.x, self.y)
    }
}


impl FPoint {
    pub fn bottom(&self, other: &FPoint) -> FPoint {
        FPoint {
            x: self.x.min(other.x),
            y: self.y.min(other.y)
        }
    }
    pub fn top(&self, other: &FPoint) -> FPoint {
        FPoint {
            x: self.x.max(other.x),
            y: self.y.max(other.y)
        }
    }
    pub fn round(&self) -> IPoint {
        IPoint {
            x: self.x.round() as i32,
            y: self.y.round() as i32
        }
    }
    pub fn ceil(&self) -> IPoint {
        IPoint {
            x: self.x.ceil() as i32,
            y: self.y.ceil() as i32
        }
    }
    pub fn floor(&self) -> IPoint {
        IPoint {
            x: self.x.floor() as i32,
            y: self.y.floor() as i32
        }
    }
    pub fn rdist(&self, to: &FPoint) -> f32 {
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }
}

impl Point<IPoint> for FPoint {
    fn dist(&self, other: &IPoint) -> f32 {
        let dx = self.x - (other.x as f32);
        let dy = self.y - (other.y as f32);
        (dx*dx + dy*dy).sqrt()
    }
}
impl Point<FPoint> for FPoint {
    fn dist(&self, other: &FPoint) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx*dx + dy*dy).sqrt()
    }
}

impl Eq for FPoint {}
impl PartialEq for FPoint {
    fn eq(&self, other: &FPoint) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add<FPoint> for FPoint {
    type Output = FPoint;
    fn add(self, other: FPoint) -> FPoint {
        FPoint { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Add<IPoint> for FPoint {
    type Output = FPoint;
    fn add(self, other: IPoint) -> FPoint {
        FPoint {x: self.x + (other.x as f32), y: self.y + (other.y as f32)}
    }
}


impl Sub<FPoint> for FPoint {
    type Output = FPoint;
    fn sub(self, other: FPoint) -> FPoint {
        FPoint { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Sub<IPoint> for FPoint {
    type Output = FPoint;
    fn sub(self, other: IPoint) -> FPoint {
        FPoint { x: self.x - (other.x as f32), y: self.y - (other.y as f32) }
    }
}

impl Neg for FPoint {
    type Output = FPoint;
    fn neg(self) -> FPoint {
        return FPoint { x: -self.x, y: -self.y };
    }
}

impl Mul<FPoint> for FPoint {
    type Output = FPoint;
    fn mul(self, other: FPoint) -> FPoint {
        FPoint { x: self.x * other.x, y: self.y * other.y }
    }
}

impl Mul<f32> for FPoint {
    type Output = FPoint;
    fn mul(self, other: f32) -> FPoint {
        FPoint { x: self.x * other, y: self.y * other }
    }
}
