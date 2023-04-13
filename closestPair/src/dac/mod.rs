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

    let (dl, mut l) = cp_helper(left);
    let (dr, mut r) = cp_helper(right);
    let xs: Vec<Point> = merge(&mut l, &mut r);

    // Determine the minimum distance in both halves 
    let mut dist = std::cmp::min(dl, dr);

    // Create the band (points that lie in [half-mindist, half + mindist] range)
    let band: Vec<Point> = xs.clone().into_iter().filter(|&p| {
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
pub fn seq_closest_distance(points: &[(i32, i32)]) -> i64 {
    // Sort by x-pos
    let mut xs = points.to_vec();
    xs.sort_by_key(|x| x.0);
    let (min_dist, _) = cp_helper(&xs);
    min_dist as i64
}
