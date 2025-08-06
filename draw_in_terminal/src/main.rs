use std::{thread, time};

fn main() {
    let size = term_size::dimensions().unwrap();
    let x = size.1 - 1;
    let y = size.0 - 1;

    let refresh_time = time::Duration::from_millis(50);

    loop {
        thread::sleep(refresh_time);
        clearscreen::clear().expect("Error to clear");

        // making objects vec
        let mut graphical_objects: Vec<GraphicalObject> = Vec::new();
        let p1 = Point::new(15, 10);
        let p2 = Point::new(25, 11);
        let p3 = Point::new(35, 12);
        let p4 = Point::new(35, 13);
        let p5 = Point::new(35, 14);

        graphical_objects.push(GraphicalObject::Point(p1));
        graphical_objects.push(GraphicalObject::Point(p2));
        graphical_objects.push(GraphicalObject::Point(p3));
        graphical_objects.push(GraphicalObject::Point(p4));
        graphical_objects.push(GraphicalObject::Point(p5));

        let mut objects: Vec<Point> = Vec::new();

        for grahical_object in graphical_objects {
            match grahical_object {
                GraphicalObject::Line(_) => {
                    //..
                }
                GraphicalObject::Point(temp_point) => {
                    objects.push(temp_point);
                }
            }
        }
        let base: Vec<Vec<String>> = generate_field(x, y, objects);

        print_screen(base);
    }
}

fn generate_field(x: usize, y: usize, objects: Vec<Point>) -> Vec<Vec<String>> {
    let mut screen_matrix: Vec<Vec<String>> = Vec::new();

    for i in 0..x {
        let mut matrix_row: Vec<String> = Vec::new();
        for j in 0..y {
            let temp_point: Point = Point::new(i, j);
            if objects.contains(&temp_point) {
                matrix_row.push(String::from("X"));
            } else {
                matrix_row.push(String::from(" "));
            }
        }
        screen_matrix.push(matrix_row);
    }

    screen_matrix
}

fn print_screen(screen: Vec<Vec<String>>) {
    for pos1 in screen {
        for pos2 in pos1 {
            print!("{pos2}");
        }
        print!("\n");
    }
}

struct Point {
    x_coordinate: usize,
    y_coordinate: usize,
}

struct Line {
    point0: Point,
    point1: Point,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        let point = Point {
            x_coordinate: x,
            y_coordinate: y,
        };
        point
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x_coordinate == other.x_coordinate && self.y_coordinate == other.y_coordinate
    }
}

enum GraphicalObject {
    Point(Point),
    Line(Line),
}
