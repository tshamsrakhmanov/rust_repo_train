use nalgebra::{self, Point2};
fn main() {
    let p0m0 = Point2::new(-1, -1);
    let p0m1 = Point2::new(1, -1);
    let p0m2 = Point2::new(-1, 1);
    let p0m3 = Point2::new(1, 1);

    let p0 = Point2::new(0, 0);
    let p1 = Point2::new(20, 0);
    let p2 = Point2::new(0, 20);

    let p3 = Point2::new(0, 0);
    let p4 = Point2::new(-20, 0);
    let p5 = Point2::new(0, 20);

    let p6 = Point2::new(0, 0);
    let p7 = Point2::new(20, 0);
    let p8 = Point2::new(0, -20);

    let p9 = Point2::new(0, 0);
    let p10 = Point2::new(-20, 0);
    let p11 = Point2::new(0, -20);

    let tr0 = Triangle2D {
        point0: p0,
        point1: p1,
        point2: p2,
    };
    let tr1 = Triangle2D {
        point0: p3,
        point1: p4,
        point2: p5,
    };
    let tr2 = Triangle2D {
        point0: p6,
        point1: p7,
        point2: p8,
    };
    let tr3 = Triangle2D {
        point0: p9,
        point1: p10,
        point2: p11,
    };
    let tr0_1 = Triangle2D {
        point0: p0m0,
        point1: p1,
        point2: p2,
    };
    let tr1_1 = Triangle2D {
        point0: p0m1,
        point1: p4,
        point2: p5,
    };
    let tr2_1 = Triangle2D {
        point0: p0m2,
        point1: p7,
        point2: p8,
    };
    let tr3_1 = Triangle2D {
        point0: p0m3,
        point1: p10,
        point2: p11,
    };
    println!("{tr0:?}");
    println!("{tr1:?}");
    println!("{tr2:?}");
    println!("{tr3:?}");
    println!("{tr0_1:?}");
    println!("{tr1_1:?}");
    println!("{tr2_1:?}");
    println!("{tr3_1:?}");
    let answer_0 = tr0.is_flat_top();
    let answer_1 = tr1.is_flat_top();
    let answer_2 = tr2.is_flat_top();
    let answer_3 = tr3.is_flat_top();
    let answer_4 = tr0.is_flat_bot();
    let answer_5 = tr1.is_flat_bot();
    let answer_6 = tr2.is_flat_bot();
    let answer_7 = tr3.is_flat_bot();
    let answer_8 = tr0.is_free_form();
    let answer_9 = tr1.is_free_form();
    let answer_10 = tr2.is_free_form();
    let answer_11 = tr3.is_free_form();
    let answer_12 = tr0_1.is_free_form();
    let answer_13 = tr0_1.is_flat_bot();
    let answer_14 = tr0_1.is_flat_top();
    let answer_15 = tr1_1.is_free_form();
    let answer_16 = tr1_1.is_flat_bot();
    let answer_17 = tr1_1.is_flat_top();
    let answer_18 = tr2_1.is_free_form();
    let answer_19 = tr2_1.is_flat_bot();
    let answer_20 = tr2_1.is_flat_top();
    let answer_21 = tr3_1.is_free_form();
    let answer_22 = tr3_1.is_flat_bot();
    let answer_23 = tr3_1.is_flat_top();
    println!();
    println!("tr0 is flat top: {answer_0}");
    println!("tr1 is flat top: {answer_1}");
    println!("tr2 is flat top: {answer_2}");
    println!("tr3 is flat top: {answer_3}");
    println!();
    println!("tr0 is flat BOT: {answer_4}");
    println!("tr1 is flat BOT: {answer_5}");
    println!("tr2 is flat BOT: {answer_6}");
    println!("tr3 is flat BOT: {answer_7}");
    println!();
    println!("tr0 is free form: {answer_8}");
    println!("tr1 is free form: {answer_9}");
    println!("tr2 is free form: {answer_10}");
    println!("tr3 is free form: {answer_11}");
    println!();
    println!("tr0_0 is free form: {answer_12}");
    println!("tr0_0 is is_flat_bot: {answer_13}");
    println!("tr0_0 is is_flat_top: {answer_14}");
    println!();
    println!("tr1_0 is free form: {answer_15}");
    println!("tr1_0 is is_flat_bot: {answer_16}");
    println!("tr1_0 is is_flat_top: {answer_17}");
    println!();
    println!("tr2_0 is free form: {answer_18}");
    println!("tr2_0 is is_flat_bot: {answer_19}");
    println!("tr2_0 is is_flat_top: {answer_20}");
    println!();
    println!("tr3_0 is free form: {answer_21}");
    println!("tr3_0 is is_flat_bot: {answer_22}");
    println!("tr3_0 is is_flat_top: {answer_23}");
}

#[derive(Debug)]
struct Triangle2D {
    point0: Point2<i32>,
    point1: Point2<i32>,
    point2: Point2<i32>,
}

impl Triangle2D {
    fn is_flat_top(&self) -> bool {
        let mut answer = false;
        if self.point0.y == self.point1.y && self.point2.y > self.point0.y {
            answer = true;
        }
        if self.point1.y == self.point2.y && self.point0.y > self.point1.y {
            answer = true;
        }
        if self.point2.y == self.point0.y && self.point1.y > self.point2.y {
            answer = true;
        }
        answer
    }
    fn is_flat_bot(&self) -> bool {
        let mut answer = false;
        if self.point0.y == self.point1.y && self.point2.y < self.point0.y {
            answer = true;
        }
        if self.point1.y == self.point2.y && self.point0.y < self.point1.y {
            answer = true;
        }
        if self.point2.y == self.point0.y && self.point1.y < self.point2.y {
            answer = true;
        }
        answer
    }
    fn is_free_form(&self) -> bool {
        let mut answer = false;
        if !self.is_flat_top() && !self.is_flat_bot() {
            answer = true;
        }
        answer
    }
    fn split_to_flat_triangles(&self) -> Vec<Triangle2D> {
        let mut answer: Vec<Triangle2D> = Vec::new();
        answer
    }
}
