fn main() {
    let p1 = Point {
        x_coordinate: 150.0,
        y_coordinate: 200.0,
    };
    let circ1 = Circle {
        radius: 15.0,
        center_point: &p1,
    };
    let circ2 = Circle {
        radius: 12.0,
        center_point: &p1,
    };
    let area = circ1.area();
    println!("{area}");
    let range_x = circ1.min_max_x();
    println!("{range_x:?}");
    let range_y = circ1.min_max_y();
    println!("{range_y:?}");
    let answer = circ2.is_inside(&circ1);
    println!("{answer}");
}

struct Point {
    x_coordinate: f32,
    y_coordinate: f32,
}

struct Circle<'a> {
    radius: f32,
    center_point: &'a Point,
}

impl<'a> Circle<'a> {
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

    fn is_inside(&self, other: &Circle) -> bool {
        self.center_point.x_coordinate == other.center_point.x_coordinate
            && self.center_point.y_coordinate == other.center_point.y_coordinate
            && self.radius < other.radius
    }
}
