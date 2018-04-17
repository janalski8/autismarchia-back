use utils::ipoint::IPoint;
use std::collections::HashSet;
use utils::fpoint::FPoint;
use utils::point::Point;

pub fn dev() -> f32 {
    2.0/5.0
}

pub fn visibility_set(transparent: &HashSet<IPoint>,
                      size: IPoint,
                      origin: IPoint,
                      sight_range: f32) -> HashSet<IPoint> {

    let origin_square = origin.square_around(sight_range.ceil() as i32);
    let room_range = size.zrange();
    let candidates = origin_square.intersect(room_range);
    let mut result: HashSet<IPoint> = candidates.into_iter()
        .filter(|p| p.dist(origin) <= sight_range)
        .filter(|p| is_visible(transparent, origin, *p))
        .collect();
    result.insert(origin);
    result
}

pub fn is_visible(transparent: &HashSet<IPoint>, from: IPoint, to: IPoint) -> bool {
    sight_paths(from, to).iter().find(|(from_alt, to_alt)|
        line_of_sight(*from_alt, *to_alt)
            .iter()
            .find(|p| !transparent.contains(p))
            .is_none() // whole line of sight transparent
    ).is_some()
}

fn sight_paths(p1: IPoint, p2: IPoint) -> Vec<(FPoint, FPoint)> {
    vec![
        (p1.float(), p2.float()),
        (p1.float(), p2 + FPoint{x: 0.0, y: dev()}),
        (p1.float(), p2 + FPoint{x: 0.0, y: -dev()}),
        (p1.float(), p2 + FPoint{x: dev(),  y: 0.0}),
        (p1.float(), p2 + FPoint{x: -dev(), y: 0.0}),
    ]
}

fn line_of_sight(p1: FPoint, p2: FPoint) -> Vec<IPoint> {
    let negx = IPoint{x: -1, y: 0};
    let negy = IPoint{x: 0, y: -1};

    let rp2 = p2.round();
    let diff = p2 - p1;
    let dist = diff.x.abs().max(diff.y.abs());
    let step = diff * (1.0/dist);

    let mut result = Vec::new();
    let mut i = 1.0;
    let mut current = p1;
    while current.round() != rp2 {
        let round = current.round();
        let cdiff = round - current;

        result.push(round);
        if cdiff.x == 0.5 {
            result.push(round + negx);
        }
        if cdiff.y == 0.5 {
            result.push(round + negy);
        }

        current = p1 + (step * i);
        i += 1.0;
    }
    result
}
