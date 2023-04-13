/*
 * Justin Copeland 6380358
 */
use rayon::prelude::*;

type Point = (i32, i32);

// Merge from class but by y-ordering
fn merge(xs: &[Point], ys: &[Point]) -> Vec<Point> {
    let mut ans = vec![];
    let (mut xs, mut ys) = (xs, ys);

    use std::cmp::Ordering;
    while !xs.is_empty() && !ys.is_empty() {
        match xs[0].1.cmp(&ys[0].1) {
            Ordering::Less => { ans.push(xs[0]); xs = &xs[1..]; }
            _ => { ans.push(ys[0]); ys = &ys[1..]; }
        }
    }

    // One will be empty so we can just extend both
    ans.extend(xs.iter());
    ans.extend(ys.iter());
    ans 
}

/// Calculates the squared distance between two points
fn euclidian_dist_sqrd(x: &Point, y: &Point) -> i32 {
    (y.0 - x.0).pow(2) + (y.1 - x.1).pow(2)
}

/// Calculates minimum distance pair in list of points by checking all combinations 
fn cp_brute_force(points: &[Point]) -> (i32, Vec<Point>) {
    let mut optimal = ( euclidian_dist_sqrd(&points[0], &points[1]), vec![points[0], points[1]] );

    // Looks bad but only used for small sizes
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let (a, b) = ( points[i], points[j] );
            let new = euclidian_dist_sqrd(&a, &b);
            if new < optimal.0 { optimal = (new, vec![a,b]); }
        }
    }
    optimal
}

#[allow(dead_code)]
fn cp_helper(points: &[Point]) -> (i32, Vec<Point>) {
    // Thank you Algo for letting me suffer through this beforehand

    // If our points are few enough, compute the brute force answer
    if points.len() <= 3 { return cp_brute_force(&points); }

    // Split points in half evenly
    let half = points.len() / 2;
    let (left, right) = points.split_at(half);
    let mid = points[half].0;

    // Recurse in parallel and merge left and right by y
    let ((dl, mut l), (dr, mut r)) = rayon::join(|| cp_helper(left), || cp_helper(right));
    let xs: Vec<Point> = merge(&mut l, &mut r);

    // Determine the minimum distance in both halves 
    let mut dist = std::cmp::min(dl, dr);

    // Create the band (points that lie in [half-mindist, half + mindist] range)
    let band: Vec<Point> = xs.clone().into_par_iter().filter(|&p| {
        p.0 > (mid - dist) && p.0 < (mid + dist)
    }).collect();

    // Only need to check the next 7 points (or up to length)
    for i in 0..band.len() {
        for j in i+1..std::cmp::min(i+8, band.len()) {
            let d = euclidian_dist_sqrd(&band[i], &band[j]);
            dist = std::cmp::min(dist, d);
        }
    }

    (dist, xs)
}

#[allow(dead_code)]
pub fn par_closest_distance(points: &[(i32, i32)]) -> i64 {
    // Sort by x-pos
    let mut xs = points.to_vec();
    xs.sort_by_key(|x| x.0);
    let (min_dist, _) = cp_helper(&xs);
    min_dist as i64
}

#[cfg(test)]
mod tests {
    use crate::cp::*;

    #[test]
    fn brute_force_test() {
        let xs: Vec<(i32, i32)> = vec![(10, 10), (0, 0), (3, -1), (4, 2)];
        let optimal = cp_brute_force(&xs);
        assert_eq!(optimal.0, 10);

        let xs2: Vec<(i32, i32)> = vec![(-450, 484), (-586, 974), (-334, 192), (-565, -946), (-376, -697), (170, 77), (-607, 301), (-724, 186), (817, -114), (-173, -223), (-841, -957), (909, -228), (-6, 636), (834, -436), (-50, -193), (-141, 545), (-963, -953), (756, -541), (347, 259), (-606, 615), (-98, -349), (-797, 223), (911, 58), (-44, 208), (2, 569), (449, -422), (-865, -756), (952, -619), (845, 15), (-263, 306), (660, 13), (-868, -578), (947, 50), (655, 259), (-827, -104), (-571, 537), (375, -872), (-21, 35), (301, 792), (796, -925), (-107, 22), (-923, 848), (212, -408), (-375, -55), (712, -115), (-678, -850), (-960, -682), (-38, 551), (791, -212), (-568, 1000), (734, -456), (-970, -593), (-785, -352), (-279, -142), (-726, 404), (-364, -552), (-543, -176), (776, 886), (376, -882), (310, -966), (-834, -534), (832, 888), (115  , -639), (523, -581), (97, -224), (-598, -647), (208, 952), (367, 660), (412, 775), (-103, -20), (770, 32), (472, 193), (410, 661), (-352, 579), (-155, -853), (-257, 996), (158, -710), (-741, -319), (246, -570), (420, 943), (607, -246), (369, -696), (307, 24), (-648, 399), (-72, 475), (-340, -204), (-944, -966), (145, 532), (183, -909), (-899, -206), (893, 853), (221, -891), (874, -966), (592, -250), (585, 606), (121, -  252), (-653, 241), (452, 740), (557, -783), (-533, 38)];

        let optimal = cp_brute_force(&xs2);
        assert_eq!(optimal.0, 101);
    }

    #[test]
    fn merge_test() {
        let mut xs: Vec<(i32, i32)> = vec![(10, 10), (0, 0), (3, -1), (4, 2)];
        xs.sort_by_key(|x| x.1);
        let mut ys: Vec<(i32, i32)> = vec![(8, 3), (7, 6), (-5, -2), (9, 9)];
        ys.sort_by_key(|x| x.1);
        let expected: Vec<(i32, i32)> = vec![(-5, -2), (3, -1), (0, 0), (4, 2), (8, 3), (7, 6), (9, 9), (10, 10)];
        let merged = merge(&xs, &ys);
        assert_eq!(expected, merged);
    }

    #[test]
    fn basic_par_closest_distance() {
        let points: Vec<(i32, i32)> = vec![(0, 0), (5, 3), (-2, 1), (1, 1)];
        let out = par_closest_distance(&points);
        assert_eq!(out, 2);

        let points2: Vec<(i32, i32)> = vec![(1, 3), (-10, -10), (42, 168), (22, 8), (9, 5)];
        let out2 = par_closest_distance(&points2);
        assert_eq!(out2, 68);

        let points3: Vec<(i32, i32)> = vec![(-926, 952), (770, -319), (-652, -367), (325, -998), (882, 696), (-932, -950), (-194, -777), (727, -281), (-707, 128), (613, -755), (531, 621), (417, -285), (-494, 180), (-328, 393), (-123, 46), (549, 325), (408, -74), (900, -220), (-104, -722), (-331, -688), (52, -770), (833, 569), (577, 558), (-776, 201), (410, -445), (-738, 680), (35, -946), (-454, 550), (-656, 60), (-102, 992), (756, -819), (-187, 203), (-618, -300), (-105, 362), (-892, 270), (-963, 800), (-63, -697), (-947, 720), (-498, -718), (51, 216), (-62, 22), (-288, 810), (902, 339), (-914, -120), (80, 655), (917, -334), (601, -717), (95, 952), (-393, -429), (-621, 941)];
        let out3 = par_closest_distance(&points3);
        assert_eq!(out3, 1588);

        let points4: Vec<(i32, i32)> = vec![(-450, 484), (-586, 974), (-334, 192), (-565, -946), (-376, -697), (170, 77), (-607, 301), (-724, 186), (817, -114), (-173, -223), (-841, -957), (909, -228), (-6, 636), (834, -436), (-50, -193), (-141, 545), (-963, -953), (756, -541), (347, 259), (-606, 615), (-98, -349), (-797, 223), (911, 58), (-44, 208), (2, 569), (449, -422), (-865, -756), (952, -619), (845, 15), (-263, 306), (660, 13), (-868, -578), (947, 50), (655, 259), (-827, -104), (-571, 537), (375, -872), (-21, 35), (301, 792), (796, -925), (-107, 22), (-923, 848), (212, -408), (-375, -55), (712, -115), (-678, -850), (-960, -682), (-38, 551), (791, -212), (-568, 1000), (734, -456), (-970, -593), (-785, -352), (-279, -142), (-726, 404), (-364, -552), (-543, -176), (776, 886), (376, -882), (310, -966), (-834, -534), (832, 888), (115, -639), (523, -581), (97, -224), (-598, -647), (208, 952), (367, 660), (412, 775), (-103, -20), (770, 32), (472, 193), (410, 661), (-352, 579), (-155, -853), (-257, 996), (158, -710), (-741, -319), (246, -570), (420, 943), (607, -246), (369, -696), (307, 24), (-648, 399), (-72, 475), (-340, -204), (-944, -966), (145, 532), (183, -909), (-899, -206), (893, 853), (221, -891), (874, -966), (592, -250), (585, 606), (121, -252), (-653, 241), (452, 740), (557, -783), (-533, 38)];
        let out4 = par_closest_distance(&points4);
        assert_eq!(out4, 101);
    }
}
