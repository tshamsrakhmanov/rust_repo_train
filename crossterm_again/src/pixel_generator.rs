fn basic_frame(x_dim: u16, y_dim: u16) -> Vec<(u16, u16)> {
    let mut points_cloud: Vec<(u16, u16)> = Vec::new();

    // fill points cloud with some data
    for i in 0..x_dim {
        for j in 0..y_dim {
            //..
            if i == 0 {
                points_cloud.push((0, j));
            }
            if j == 0 {
                points_cloud.push((i, 0));
            }
            if i == x_dim - 1 {
                points_cloud.push((x_dim - 1, j));
            }
            if j == y_dim - 1 {
                points_cloud.push((i, y_dim - 1));
            }
        }
    }

    points_cloud
}

pub fn frame_generator(x_dim: u16, y_dim: u16) -> Vec<(u16, u16)> {
    let mut frame: Vec<(u16, u16)> = Vec::new();

    for pos in basic_frame(x_dim, y_dim) {
        frame.push(pos);
    }

    frame
}
