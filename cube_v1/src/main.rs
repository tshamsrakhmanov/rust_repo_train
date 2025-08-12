use nalgebra::{self as na, Vector4};
fn main() {
    let model = na::Matrix4::identity();
    let view = na::Matrix4::look_at_rh(
        &na::Point3::new(5.0, 5.0, 5.0),
        &na::Point3::origin(),
        &na::Vector3::new(0.0, 1.0, 0.0),
    );
    let screen_x_dimension: f64 = 100.0;
    let screen_y_dimension: f64 = 50.0;

    let left_boundary = -1.0 * (&screen_x_dimension / 2.0);
    let right_boundary = 1.0 * (&screen_x_dimension / 2.0);
    let bottom_boundary = -1.0 * (&screen_y_dimension / 2.0);
    let top_boundary = 1.0 * (&screen_y_dimension / 2.0);

    let projection = na::Matrix4::new_orthographic(
        left_boundary,
        right_boundary,
        bottom_boundary,
        top_boundary,
        0.1,
        100.0,
    );

    // 2. Multiply (no references)
    let mvp = projection * view * model;

    // 3. Use `mvp` for shaders or transformations
    println!("MVP matrix: {}", mvp);

    let point1 = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let point2 = Vector4::new(10.0, 0.0, 0.0, 1.0);
    let point3 = Vector4::new(0.0, 10.0, 0.0, 1.0);
    let point4 = Vector4::new(0.0, 0.0, 10.0, 1.0);

    println!("desired point to draw:");
    println!("{point1}");

    let point1_proj = mvp * point1;
    let point2_proj = mvp * point2;
    let point3_proj = mvp * point3;
    let point4_proj = mvp * point4;

    println!("result of drawing:");
    println!("{point1_proj}");

    let point1_ndc = point1_proj.xyz() / point1.w;
    let point2_ndc = point2_proj.xyz() / point2.w;
    let point3_ndc = point3_proj.xyz() / point3.w;
    let point4_ndc = point4_proj.xyz() / point4.w;

    println!("result of ndc:");
    println!("{point1_ndc}");

    let x1 = (screen_x_dimension / 2.0) * (1.0 + point1_ndc.x);
    let x2 = (screen_x_dimension / 2.0) * (1.0 + point2_ndc.x);
    let x3 = (screen_x_dimension / 2.0) * (1.0 + point3_ndc.x);
    let x4 = (screen_x_dimension / 2.0) * (1.0 + point4_ndc.x);
    let y1 = (screen_y_dimension / 2.0) * (1.0 + point1_ndc.y);
    let y2 = (screen_y_dimension / 2.0) * (1.0 + point2_ndc.y);
    let y3 = (screen_y_dimension / 2.0) * (1.0 + point3_ndc.y);
    let y4 = (screen_y_dimension / 2.0) * (1.0 + point4_ndc.y);
    println!("x:{}, y:{} ", &x1, &y1);
    println!("x:{}, y:{} ", &x2, &y2);
    println!("x:{}, y:{} ", &x3, &y3);
    println!("x:{}, y:{} ", &x4, &y4);
}
