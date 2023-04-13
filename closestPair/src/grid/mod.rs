use std::collections::HashMap;

type Point = (i32, i32);

pub fn euclidian_dist_sqrd(x: &Point, y: &Point) -> f64 {
    (y.0 - x.0).pow(2) as f64 + (y.1 - x.1).pow(2) as f64
}

pub fn closest_pair(points: &[Point]) -> (Point, Point) {
    if points.len() < 2 {
        return (points[0], points[1])
    }
    
    let mut grid: HashMap<i64, Vec<Point>> = HashMap::new();
    let mut min_distance = euclidian_dist_sqrd(&points[0], &points[1]) as f64;
    let mut closest_pair = (points[0], points[1]);
    
    for p in points {
        let x_bucket = (p.0 as f64/ (min_distance+0.001)).floor() as i64;
        let y_bucket = (p.1 as f64/ (min_distance+0.001)).floor() as i64;
        
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                let bucket_key = ((x_bucket + x_offset) << 32) + (y_bucket + y_offset);
                if let Some(other_points) = grid.get(&bucket_key) {
                    for other_point in other_points {
                        let distance = euclidian_dist_sqrd(&other_point, p);
                        if distance < min_distance {
                            min_distance = distance;
                            closest_pair = (*p, *other_point);
                        }
                    }
                }
            }
        }
        
        grid.entry((x_bucket << 32) + y_bucket).or_default().push(*p);
    }
    
    closest_pair
}
