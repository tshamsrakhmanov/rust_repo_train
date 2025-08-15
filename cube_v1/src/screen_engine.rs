use nalgebra::{self as na, Vector4};

pub fn calc(
    screen_x_dimension: f64,
    screen_y_dimension: f64,
    point_input: Vector4<f64>,
) -> (isize, isize) {
    let model1: na::Matrix4<f64> = na::Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let left_boundary = -1.0 * (&screen_x_dimension / 2.0);
    let right_boundary = 1.0 * (&screen_x_dimension / 2.0);
    let bottom_boundary = -1.0 * (&screen_y_dimension / 2.0);
    let top_boundary = 1.0 * (&screen_y_dimension / 2.0);
    let view = na::Matrix4::look_at_rh(
        &na::Point3::new(100.0, 100.0, 30.0),
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
    let mvp_projection_view_model_matrix = projection * view * model1;

    let point_proj = mvp_projection_view_model_matrix * point_input;

    let point_ndc = point_proj.xyz() / point_input.w;
    let x1_raw = (screen_x_dimension / 2.0) * (1.0 + point_ndc.x);
    let y1_raw = (screen_y_dimension / 2.0) * (1.0 + point_ndc.y);

    let x1_mod = screen_x_dimension - x1_raw;

    (x1_mod as isize, y1_raw as isize)
}
