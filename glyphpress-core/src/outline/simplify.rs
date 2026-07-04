//! Ramer-Douglas-Peucker outline simplification.


use crate::outline::contour::ContourSet;
use crate::outline::point::Point;

pub fn simplify_contours(input: &ContourSet, tolerance: f32) -> ContourSet {
    let mut out = ContourSet::default();
    let mut cursor = 0usize;
    for &end in &input.ends {
        let start = cursor;
        let end_idx = end as usize + 1;
        let slice = &input.points[start..end_idx];
        let simplified = rdp_slice(slice, tolerance);
        let base = out.points.len() as u16;
        out.points.extend(simplified);
        out.ends.push(base + out.points.len() as u16 - base - 1);
        cursor = end_idx;
    }
    out
}

fn rdp_slice(points: &[Point], epsilon: f32) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec();
    }
    let first = points[0];
    let last = points[points.len() - 1];
    let mut max_dist = 0f32;
    let mut index = 0usize;
    for (i, p) in points.iter().enumerate().skip(1).take(points.len().saturating_sub(2)) {
        let d = perpendicular_distance(*p, first, last);
        if d > max_dist {
            max_dist = d;
            index = i;
        }
    }
    if max_dist > epsilon {
        let mut left = rdp_slice(&points[..=index], epsilon);
        let right = rdp_slice(&points[index..], epsilon);
        left.pop();
        left.extend(right);
        left
    } else {
        vec![first, last]
    }
}

fn perpendicular_distance(p: Point, a: Point, b: Point) -> f32 {
    let dx = b.x as f32 - a.x as f32;
    let dy = b.y as f32 - a.y as f32;
    if dx == 0.0 && dy == 0.0 {
        return ((p.x as f32 - a.x as f32).powi(2) + (p.y as f32 - a.y as f32).powi(2)).sqrt();
    }
    let num = ((b.y as f32 - a.y as f32) * p.x as f32 - (b.x as f32 - a.x as f32) * p.y as f32
        + b.x as f32 * a.y as f32 - b.y as f32 * a.x as f32).abs();
    let den = (dx * dx + dy * dy).sqrt();
    num / den
}

/* volume */

pub fn simplify_with_iterations(input: &crate::outline::contour::ContourSet, tolerance: f32, passes: u8) -> crate::outline::contour::ContourSet {
    let mut cur = input.clone();
    for _ in 0..passes {
        cur = simplify_contours(&cur, tolerance);
    }
    cur
}
