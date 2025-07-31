fn main() {
    let p1 = Point {
        x_coordinate: 150.0,
        y_coordinate: 200.0,
    };
    let circ1 = Circle {
        radius: 15.0,
        center_point: p1,
    };
    let area = circ1.area();
    println!("{area}");
    let range = circ1.min_max_x();
    println!("{range:?}");
    let range_1 = circ1.min_max_y();
    println!("{range_1:?}");
}

struct Point {
    x_coordinate: f32,
    y_coordinate: f32,
}

struct Circle {
    radius: f32,
    center_point: Point,
}

impl Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * 3.141568
    }

    fn min_max_x(&self) -> (f32, f32) {
        (
            self.center_point.x_coordinate - self.radius,
            self.center_point.x_coordinate + self.radius,
        )
    }

    fn min_max_y(&self) -> (f32, f32) {
        (
            self.center_point.y_coordinate - self.radius,
            self.center_point.y_coordinate + self.radius,
        )
    }
}
