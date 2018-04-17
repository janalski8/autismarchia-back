use core::cmp;
use utils::ipoint::IPoint;
use utils::fpoint::FPoint;

#[derive(Clone, Copy, Debug)]
pub struct IRange {
    pub start: IPoint,
    pub end: IPoint,
}

impl IRange {
    pub fn center(self) -> FPoint {
        (self.end + self.start).float() * (0.5)
    }
    pub fn size(self) -> IPoint {
        self.end - self.start
    }
    pub fn clip(self, point: IPoint) -> IPoint {
        point.top(self.start).bottom(self.end)
    }
    pub fn rdist(self, other: IRange) -> i32 {
        let max = self.end.bottom(other.end);
        let min = self.start.top(other.start);
        let diff = max - min;
        -cmp::min(diff.x, diff.y)
    }
    pub fn inside(self, point: IPoint) -> bool {
        return point.x < self.end.x && point.y < self.end.y &&
            point.x >= self.start.x && point.y >= self.start.y;
    }
    pub fn iter(self) -> IITer {
        IITer::new(self)
    }
    pub fn intersect(self, other: IRange) -> IRange {
        IRange {
            start: self.start.top(other.start),
            end: self.end.bottom(other.end)
        }
    }
}

impl IntoIterator for IRange {
    type Item = IPoint;
    type IntoIter = IITer;

    fn into_iter(self) -> IITer {
        IITer::new(self)
    }
}

pub struct IITer {
    range: IRange,
    x: i32,
    y: i32
}

impl IITer {
    fn new(range: IRange) -> IITer {
        IITer {
            range,
            x: range.start.x,
            y: range.start.y,
        }
    }
}

impl Iterator for IITer {
    type Item = IPoint;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.y >= self.range.end.y {
            return None
        }
        let result = IPoint { x: self.x, y: self.y };
        self.x += 1;
        if self.x >= self.range.end.x {
            self.x = self.range.start.x;
            self.y += 1;
        }
        Some(result)
    }
}
