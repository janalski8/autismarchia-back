use rand::XorShiftRng;
use rand::Rng;
use utils::ipoint::IPoint;
use utils::irange::IRange;

pub trait PointRng {
    fn get_point(&mut self, min: &IPoint, max: &IPoint) -> IPoint;
    fn gen_in_range(&mut self, range: &IRange) -> IPoint {
        self.get_point(&range.start, &range.end)
    }
}

impl PointRng for XorShiftRng {
    fn get_point(&mut self, start: &IPoint, end: &IPoint) -> IPoint {
        IPoint {
            x: self.gen_range(start.x, end.x),
            y: self.gen_range(start.y, end.y)
        }
    }
}
