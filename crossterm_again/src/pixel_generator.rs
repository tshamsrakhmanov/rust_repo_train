pub fn frame_generator() -> Vec<(f32, f32)> {
    let mut frame: Vec<(f32, f32)> = Vec::new();

    //  fill up goes here.....
    //  drawing of screen limits with explanation
    frame.push((0.0, 0.0)); // base
    frame.push((1.0, 0.0)); // bottom - max X dimension
    frame.push((1.0, 1.0)); // bottom right - max X max Y dimension
    frame.push((0.0, 1.0)); // right - max Y dimension
    frame.push((0.0, -1.0)); // top - min Y dimension
    frame.push((-1.0, 0.0)); // left - min X dimension
    frame.push((-1.0, -1.0)); // top left - min X min Y dimension
    frame.push((1.0, -1.0)); // top right - max X min Y dimension
    frame.push((-1.0, 1.0)); // top right - max X min Y dimension

    // .... ends here

    frame
}
