use std::any::Any;
fn main() {
    // create structs, which have one common trait
    // in order to use them in storage as Vec ->
    // -> put them into Box::new
    // this will give us opportunity to call them in runtime
    let b1 = Box::new(Metal::new());
    let b2 = Box::new(Wood::new());
    let b3 = Box::new(Plastic::new());
    let b4 = Box::new(Steel::new());

    // create storate
    // in fact it's just Vec<Box<dyn trait>> wrapper
    let mut box1 = Construction::new();

    // add boxes to Vec inside of the sturct
    box1.add(b1);
    box1.add(b2);
    box1.add(b3);
    box1.add(b4);

    // iterate through objects inside vec and call their struct type
    for obj in box1.body {
        println!("---------------");

        // downcast_ref - is our choice
        if let Some(a) = obj.as_any().downcast_ref::<Metal>() {
            println!("Metal, with density: {}", a.density);
            let v1 = a.reflect((1.0, 1.0, 1.0));
            println!("reflection is {:?}", v1);
        } else if let Some(a) = obj.as_any().downcast_ref::<Wood>() {
            println!("Wood, with density: {}", a.density);
            let v1 = a.reflect((1.0, 1.0, 1.0));
            println!("reflection is {:?}", v1);
        } else if let Some(a) = obj.as_any().downcast_ref::<Plastic>() {
            println!("Plastic, with density: {}", a.density);
            let v1 = a.reflect((1.0, 1.0, 1.0));
            println!("reflection is {:?}", v1);
        } else {
            println!("unknown object")
        }
        // all else - is just calling base methods of the trait
    }
}

#[derive(Debug)]
pub struct Steel {
    density: f32,
}
impl Steel {
    pub fn new() -> Steel {
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
pub struct Metal {
    density: f32,
}
impl Metal {
    pub fn new() -> Metal {
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
pub struct Wood {
    density: f32,
}
impl Wood {
    pub fn new() -> Wood {
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
pub struct Plastic {
    density: f32,
}

impl Plastic {
    pub fn new() -> Plastic {
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

struct Construction {
    body: Vec<Box<dyn PhysicalTrait>>,
}

impl Construction {
    pub fn new() -> Construction {
        Construction { body: Vec::new() }
    }
    pub fn add(&mut self, obj: Box<dyn PhysicalTrait>) {
        // this one is important!!!
        // only by using this structure we can put something arbitraury
        // for exmpl: a box with a sturct who, shares a trait
        self.body.push(obj.into());
    }
}
// TRAIT DECLARATIONS
// this one will be applied to each one of the objects
// and will be check'd as a condition to fill up the storage
// -> only the objects who's sharing this trait are allowed inside
pub trait PhysicalTrait: Any {
    fn reflect(&self, vector: (f32, f32, f32)) -> (f32, f32, f32);
    fn get_density(&self) -> f32;
}

// ALARM!!!
// this one is important -> because of this we can call as_any
// and be able to reach HIGHER struct's type
impl dyn PhysicalTrait {
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
