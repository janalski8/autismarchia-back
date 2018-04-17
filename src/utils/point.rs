pub trait Point<O> {
    fn dist(self, other: O) -> f32;
}