use std::ops::{Add, Sub, Neg, Mul};
use std::hash::{Hash, Hasher};
use utils::fpoint::FPoint;
use utils::irange::IRange;
use utils::point::Point;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct IPoint {
    pub x: i32,
    pub y: i32,
}

impl Debug for IPoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}
impl Display for IPoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}

impl IPoint {
    pub fn zero() -> IPoint {
        IPoint {
            x: 0,
            y: 0
        }
    }
    pub fn bottom(&self, other: &IPoint) -> IPoint {
        IPoint {
            x: self.x.min(other.x),
            y: self.y.min(other.y)
        }
    }
    pub fn top(&self, other: &IPoint) -> IPoint {
        IPoint {
            x: self.x.max(other.x),
            y: self.y.max(other.y)
        }
    }
    pub fn float(&self) -> FPoint {
        FPoint {
            x: self.x as f32,
            y: self.y as f32
        }
    }
    pub fn rdist(&self, to: &IPoint) -> i32 {
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }
    pub fn range(self, to: IPoint) -> IRange {
        IRange {start: self, end: to}
    }
    pub fn zrange(self) -> IRange {
        IRange {start: IPoint::zero(), end: self}
    }
    pub fn square_around(self, radius: i32) -> IRange {
        IRange {
            start: self - IPoint{x: radius, y: radius},
            end: self + IPoint{x: radius + 1, y: radius + 1}
        }
    }
}

impl Point<IPoint> for IPoint {
    fn dist(&self, other: &IPoint) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        ((dx*dx + dy*dy) as f32).sqrt()
    }
}
impl Point<FPoint> for IPoint {
    fn dist(&self, other: &FPoint) -> f32 {
        let dx = (self.x as f32) - other.x;
        let dy = (self.y as f32) - other.y;
        (dx*dx + dy*dy).sqrt()
    }
}

impl Eq for IPoint {}
impl PartialEq for IPoint {
    fn eq(&self, other: &IPoint) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add<IPoint> for IPoint {
    type Output = IPoint;
    fn add(self, other: IPoint) -> IPoint {
        IPoint {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Add<FPoint> for IPoint {
    type Output = FPoint;
    fn add(self, other: FPoint) -> FPoint {
        FPoint {x: self.x as f32 + other.x, y: self.y as f32 + other.y}
    }
}

impl Sub<IPoint> for IPoint {
    type Output = IPoint;
    fn sub(self, other: IPoint) -> IPoint {
        IPoint {x: self.x - other.x, y: self.y - other.y}
    }
}
impl Sub<FPoint> for IPoint {
    type Output = FPoint;
    fn sub(self, other: FPoint) -> FPoint {
        FPoint {x: self.x as f32 - other.x, y: self.y as f32 - other.y}
    }
}

impl Neg for IPoint {
    type Output = IPoint;
    fn neg(self) -> IPoint {
        return IPoint {x: -self.x, y: -self.y};
    }
}

impl Mul<IPoint> for IPoint {
    type Output = IPoint;
    fn mul(self, other: IPoint) -> IPoint {
        IPoint {x: self.x * other.x, y: self.y * other.y}
    }
}

impl Mul<i32> for IPoint {
    type Output = IPoint;
    fn mul(self, other: i32) -> IPoint {
        IPoint {x: self.x * other, y: self.y * other}
    }
}

impl Hash for IPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
