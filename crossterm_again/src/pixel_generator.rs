pub fn frame_generator(x_dim: u16, y_dim: u16) -> Vec<(u16, u16)> {
    let mut frame: Vec<(u16, u16)> = Vec::new();

    //  fill up goes here.....
    frame.push((15, 15));

    for i in draw_line(15, 15, 80, 20) {
        frame.push(i);
    }
    for i in draw_triangle(5, 5, 10, 16, 30, 30) {
        frame.push(i);
    }

    // .... ends here
    frame
}

fn draw_line(x0: u16, y0: u16, x1: u16, y1: u16) -> Vec<(u16, u16)> {
    let mut result: Vec<(u16, u16)> = Vec::new();

    // add start and end of a line
    result.push((x0, y0));
    result.push((x1, y1));

    let temp = bresenham::Bresenham::new((x0 as isize, y0 as isize), (x1 as isize, y1 as isize));
    let mut transversed_temp: Vec<(u16, u16)> = Vec::new();

    for pos in temp {
        transversed_temp.push((pos.0 as u16, pos.1 as u16));
    }

    for pos in transversed_temp {
        result.push(pos);
    }

    result
}

fn draw_triangle(x0: u16, y0: u16, x1: u16, y1: u16, x2: u16, y2: u16) -> Vec<(u16, u16)> {
    let mut result: Vec<(u16, u16)> = Vec::new();
    result.push((x0, y0));
    result.push((x1, y1));
    result.push((x2, y2));

    let temp0 = bresenham::Bresenham::new((x0 as isize, y0 as isize), (x1 as isize, y1 as isize));
    let temp1 = bresenham::Bresenham::new((x1 as isize, y1 as isize), (x2 as isize, y2 as isize));
    let temp2 = bresenham::Bresenham::new((x2 as isize, y2 as isize), (x0 as isize, y0 as isize));

    for pos in temp0 {
        result.push((pos.0 as u16, pos.1 as u16));
    }
    for pos in temp1 {
        result.push((pos.0 as u16, pos.1 as u16));
    }
    for pos in temp2 {
        result.push((pos.0 as u16, pos.1 as u16));
    }

    result
}
