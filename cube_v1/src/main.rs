use nalgebra::{self as na, Vector4};
fn main() {
    // declare values of up vector
    // in fact, can be removed due to usage only in debug mode
    let up_vector_x: f64 = 0.0;
    let up_vector_y: f64 = 0.0;
    let up_vector_z: f64 = -1.0;

    // setup of projection matrices
    // step 1 - model matrix
    // ... this model in identity matrix - so it's main diagonal is full of 1's
    // ... this is the representation of modification for the point
    // ... as long as we dont have any modifications - we keep it as identity matrix
    let model = na::Matrix4::identity();

    // step 2 - view matrix
    // ... this matrix gives us representation of view point and how its allocated in space of
    // global coordinates
    let view = na::Matrix4::look_at_rh(
        &na::Point3::new(5.0, 5.0, 5.0),
        &na::Point3::origin(),
        &na::Vector3::new(up_vector_x, up_vector_y, up_vector_z),
    );

    // step 3 - projection matrix
    // ... with this matrix we're making NDC area - so the projected image will be ready to apply
    // to any given screen knowing  it's dimension parameters
    // in this case i'll take 100 by 50 entries of terminal symbols
    let screen_x_dimension: f64 = 100.0;
    let screen_y_dimension: f64 = 50.0;

    // we need to setup boundaries as half of the screen, because we need it to face in the center
    // of the view vector
    let left_boundary = -1.0 * (&screen_x_dimension / 2.0);
    let right_boundary = 1.0 * (&screen_x_dimension / 2.0);
    let bottom_boundary = -1.0 * (&screen_y_dimension / 2.0);
    let top_boundary = 1.0 * (&screen_y_dimension / 2.0);

    // calculation of the matrix itself
    let projection = na::Matrix4::new_orthographic(
        left_boundary,
        right_boundary,
        bottom_boundary,
        top_boundary,
        0.1,
        100.0,
    );

    // FINAL step - making MVP matrix - model X view X projection
    let mvp = projection * view * model;

    // by this point all neccessry matrix calcalutions are done
    // now we can project points, vector - all we need - to the screen

    // println!("MVP matrix: {}", mvp);

    // make 4 points
    let point1 = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let point2 = Vector4::new(10.0, 0.0, 0.0, 1.0);
    let point3 = Vector4::new(0.0, 10.0, 0.0, 1.0);
    let point4 = Vector4::new(0.0, 0.0, 10.0, 1.0);

    // println!("desired point to draw:");
    // println!("{point1}");

    // make projection of the points
    let point1_proj = mvp * point1;
    let point2_proj = mvp * point2;
    let point3_proj = mvp * point3;
    let point4_proj = mvp * point4;

    // println!("result of drawing:");
    // println!("{point1_proj}");

    // remove W paramets - so make Vec3 from Vec4
    let point1_ndc = point1_proj.xyz() / point1.w;
    let point2_ndc = point2_proj.xyz() / point2.w;
    let point3_ndc = point3_proj.xyz() / point3.w;
    let point4_ndc = point4_proj.xyz() / point4.w;

    // println!("result of ndc:");
    // println!("{point1_ndc}");

    // calculation of RAW points parameters in coordinates of screen
    // ATTENTION!
    // it raw and based on up_vector parameters
    // be ready to apply some changes if something is wrong with left-right-top-bottom on the
    // screen
    let x1_raw = (screen_x_dimension / 2.0) * (1.0 + point1_ndc.x);
    let x2_raw = (screen_x_dimension / 2.0) * (1.0 + point2_ndc.x);
    let x3_raw = (screen_x_dimension / 2.0) * (1.0 + point3_ndc.x);
    let x4_raw = (screen_x_dimension / 2.0) * (1.0 + point4_ndc.x);
    let y1_raw = (screen_y_dimension / 2.0) * (1.0 + point1_ndc.y);
    let y2_raw = (screen_y_dimension / 2.0) * (1.0 + point2_ndc.y);
    let y3_raw = (screen_y_dimension / 2.0) * (1.0 + point3_ndc.y);
    let y4_raw = (screen_y_dimension / 2.0) * (1.0 + point4_ndc.y);

    // making horizontal flip
    let x1 = screen_x_dimension - x1_raw;
    let x2 = screen_x_dimension - x2_raw;
    let x3 = screen_x_dimension - x3_raw;
    let x4 = screen_x_dimension - x4_raw;

    // this section is commented due to non-viable vertical flip
    // let y1 = screen_y_dimension - y1_raw;
    // let y2 = screen_y_dimension - y2_raw;
    // let y3 = screen_y_dimension - y3_raw;
    // let y4 = screen_y_dimension - y4_raw;

    // this section commented because:
    // it debug info about RAW points - before horizontal flip
    // println!(
    // "raw with up vector x:{} y:{},z:{}",
    // &up_vector_x, &up_vector_y, &up_vector_z
    // );
    // println!("p1 x:{}, y:{} ", &x1_raw.round(), &y1_raw.round());
    // println!("p2 x:{}, y:{} ", &x2_raw.round(), &y2_raw.round());
    // println!("p3 x:{}, y:{} ", &x3_raw.round(), &y3_raw.round());
    // println!("p4 x:{}, y:{} ", &x4_raw.round(), &y4_raw.round());
    println!("ready to draw:");
    println!("p1 x:{}, y:{} ", &x1.round(), &y1_raw.round());
    println!("p2 x:{}, y:{} ", &x2.round(), &y2_raw.round());
    println!("p3 x:{}, y:{} ", &x3.round(), &y3_raw.round());
    println!("p4 x:{}, y:{} ", &x4.round(), &y4_raw.round());
}
