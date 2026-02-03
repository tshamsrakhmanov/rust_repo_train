use std::any::Any;
fn main() {
    let b1 = Metal::new();
    let b2 = Wood::new();
    let b3 = Plastic::new();
    let b4 = Steel::new();
    let mut box1 = Construction::new();
    box1.add(Box::new(b1));
    box1.add(Box::new(b2));
    box1.add(Box::new(b3));
    box1.add(Box::new(b4));
    for obj in box1.body {
        println!("---------------");
        let d = obj.get_density();
        println!("obj density is: {}", d);
        let ref1 = obj.reflect((1.0, 1.0, 1.0));
        println!("obj ref is {:?}", ref1);
        println!("obj type_id is {:?}", obj.type_id());
        if let Some(a) = obj.as_any().downcast_ref::<Metal>() {
            println!("Metal");
        } else if let Some(a) = obj.as_any().downcast_ref::<Wood>() {
            println!("Wood")
        } else if let Some(a) = obj.as_any().downcast_ref::<Plastic>() {
            println!("Plastic")
        } else {
            println!("unknown object")
        }
    }
}

#[derive(Debug)]
struct Steel {
    density: f32,
}
impl Steel {
    fn new() -> Steel {
        Steel { density: 4.0 }
    }
}

impl PhysicalTrait for Steel {
    fn reflect(&self, vector: (f32, f32, f32)) -> (f32, f32, f32) {
        (vector.0 * 2.0, vector.1 * 2.0, vector.2 * 2.0)
    }
    fn get_density(&self) -> f32 {
        self.density
    }
}

#[derive(Debug)]
struct Metal {
    density: f32,
}
impl Metal {
    fn new() -> Metal {
        Metal { density: 3.0 }
    }
}

impl PhysicalTrait for Metal {
    fn reflect(&self, vector: (f32, f32, f32)) -> (f32, f32, f32) {
        (vector.0 * 2.0, vector.1 * 2.0, vector.2 * 2.0)
    }
    fn get_density(&self) -> f32 {
        self.density
    }
}

#[derive(Debug)]
struct Wood {
    density: f32,
}
impl Wood {
    fn new() -> Wood {
        Wood { density: 2.0 }
    }
}

impl PhysicalTrait for Wood {
    fn reflect(&self, vector: (f32, f32, f32)) -> (f32, f32, f32) {
        (vector.0 * 3.0, vector.1 * 3.0, vector.2 * 3.0)
    }
    fn get_density(&self) -> f32 {
        self.density
    }
}

#[derive(Debug)]
struct Plastic {
    density: f32,
}

impl Plastic {
    fn new() -> Plastic {
        Plastic { density: 1.0 }
    }
}

impl PhysicalTrait for Plastic {
    fn reflect(&self, vector: (f32, f32, f32)) -> (f32, f32, f32) {
        (vector.0 * 4.0, vector.1 * 4.0, vector.2 * 4.0)
    }
    fn get_density(&self) -> f32 {
        self.density
    }
}

trait PhysicalTrait: Any {
    fn reflect(&self, vector: (f32, f32, f32)) -> (f32, f32, f32);
    fn get_density(&self) -> f32;
}

impl dyn PhysicalTrait {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
struct Construction {
    body: Vec<Box<dyn PhysicalTrait>>,
}

impl Construction {
    fn new() -> Construction {
        Construction { body: Vec::new() }
    }
    fn add(&mut self, obj: Box<dyn PhysicalTrait>) {
        self.body.push(obj.into());
    }
}
