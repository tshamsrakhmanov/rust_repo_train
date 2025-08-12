use nalgebra::{self as na, Vector4};

pub fn calculate(
    screen_x_dimension: f64,
    screen_y_dimension: f64,
    point_input: Vector4<f64>,
) -> (u32, u32) {
    let model: na::Matrix4<f64> = na::Matrix4::identity();

    let left_boundary = -1.0 * (&screen_x_dimension / 2.0);
    let right_boundary = 1.0 * (&screen_x_dimension / 2.0);
    let bottom_boundary = -1.0 * (&screen_y_dimension / 2.0);
    let top_boundary = 1.0 * (&screen_y_dimension / 2.0);
    let view = na::Matrix4::look_at_rh(
        &na::Point3::new(100.0, 100.0, 25.0),
        &na::Point3::origin(),
        &na::Vector3::new(0.0, 0.0, -1.0),
    );
    let projection = na::Matrix4::new_orthographic(
        left_boundary,
        right_boundary,
        bottom_boundary,
        top_boundary,
        0.1,
        100.0,
    );
    let mvp = projection * view * model;

    let point_proj = mvp * point_input;

    let point_ndc = point_proj.xyz() / point_input.w;
    let x1_raw = (screen_x_dimension / 2.0) * (1.0 + point_ndc.x);
    let y1_raw = (screen_y_dimension / 2.0) * (1.0 + point_ndc.y);
    let x1 = screen_x_dimension - x1_raw;

    (x1 as u32, y1_raw as u32)
}
