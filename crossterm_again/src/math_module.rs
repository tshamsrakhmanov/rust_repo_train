use nalgebra::{self as na, Matrix4};

pub fn calculate_triangles() -> Matrix4<f32> {
    // calculate view matrix                            mat4_view
    // calculate projection matrix                      mat4_proj
    // take points in relation to global coordinate system
    // make a product of all and get screen-space dots

    let camera_postion = na::Vector3::new(0.0, 100.0, 0.0);
    let target = na::Vector3::new(0.0, 0.0, 0.0);
    let up_vector = na::Vector3::new(0.0, 1.0, 0.0);
    let front_vector = (target - camera_postion).normalize();
    let raw_right_vector = up_vector.cross(&front_vector);
    let right_vector = &raw_right_vector.normalize();
    let dot_1 = right_vector.dot(&target) * (-1.0);
    let dot_2 = up_vector.dot(&target) * (-1.0);
    let dot_3 = front_vector.dot(&target);

    let view_matrix = na::Matrix4::new(
        right_vector[0],
        right_vector[1],
        right_vector[2],
        dot_1,
        up_vector[0],
        up_vector[1],
        up_vector[2],
        dot_2,
        front_vector[0],
        front_vector[1],
        front_vector[2],
        dot_3,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    let projection_matrix = na::Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let some_point_matrix = na::Vector4::new(0.0, 0.0, 100.0, 1.0);

    let result_point_on_screen: Matrix4<f32> = view_matrix * projection_matrix;

    result_point_on_screen
}
