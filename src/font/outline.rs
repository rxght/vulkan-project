use super::Contour;
use cgmath::Vector2;

const SCALING_FACTOR: f32 = 0.1;

/// Calculates the vertices and indices for a contour
pub fn calc_contour_vertices(contour: &Contour) -> Vec<Vector2<f32>> {
    let mut vertices = Vec::new();

    if contour.points.is_empty() {
        return vertices;
    }
    vertices.push(contour.points[0] * SCALING_FACTOR);

    for i in 1..contour.points.len() {
        let current_point = contour.points[i] * SCALING_FACTOR;
        let is_previous_on_curve = contour.on_curve[i - 1];
        let on_curve = contour.on_curve[i];

        if is_previous_on_curve {
            if on_curve {
                vertices.push(current_point);
            }
        } else {
            let p_0 = *vertices.last().unwrap();
            let p_1 = contour.points[i - 1] * SCALING_FACTOR;
            let p_2 = match on_curve {
                true => current_point,
                false => 0.5 * (p_1 + current_point),
            };

            const SEGMENT_COUNT: usize = 3;
            for j in 1..=SEGMENT_COUNT {
                let t = j as f32 / SEGMENT_COUNT as f32;
                let new_vertex = t * t * (p_0 - 2.0 * p_1 + p_2) + 2.0 * t * (-p_0 + p_1) + p_0;
                vertices.push(new_vertex);
            }
        }
    }

    return vertices;
}
