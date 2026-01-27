use nalgebra::Vector3;

pub fn is_point_on_line(
    vec_c: Vector3<f32>,
    vec_a: Vector3<f32>,
    vec_b: Vector3<f32>,
    tolerance: f32,
) -> (bool, f32) {
    let vec_d = vec_b - vec_a;
    let vec_e = vec_c - vec_a;
    // 1st check - if point is on the line
    // len of cross prod give a understanding if point is on the line
    let cross_product_length = vec_e.cross(&vec_d).norm();

    if cross_product_length < tolerance {
        // 2nd check
        // dot product give an understanding how much from start point we are with requested point
        let dot_product = vec_d.dot(&vec_e);

        if dot_product > 0.0 {
            let distance_of_hit = vec_e.norm();
            return (true, distance_of_hit);
        } else {
            return (false, -1.0);
        }
    } else {
        return (false, -1.0);
    }
}
