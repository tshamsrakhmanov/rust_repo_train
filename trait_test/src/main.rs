use std::{ffi::os_str::Display, fmt::Debug};

fn main() {
    let a = SomeObject::new(0.0, 0.0);
    let b = SomeObject::new(0.0, 0.0);
    let mut list1 = SomeObjectList::new(a);
    list1.add_object(b);
    println!("{:#?}", list1);
}

#[derive(Debug)]
struct SomeObject {
    a: f32,
    b: f32,
}

impl SomeObject {
    fn new(a: f32, b: f32) -> SomeObject {
        SomeObject { a: a, b: b }
    }
}

impl GeneralTrait for SomeObject {
    fn behaviour_basic<T>(&mut self) {}
}

#[derive(Debug)]
struct SomeObjectList {
    objects_storage: Vec<SomeObject>,
}

impl SomeObjectList {
    fn new(input_object: SomeObject) -> SomeObjectList {
        let mut temp: Vec<SomeObject> = Vec::new();
        temp.push(input_object);
        SomeObjectList {
            objects_storage: temp,
        }
    }
    fn add_object(&mut self, input_obj: SomeObject) {
        self.objects_storage.push(input_obj);
    }
}

trait GeneralTrait {
    fn behaviour_basic<T>(&mut self) {}
    fn behaviour_uncommon() {}
}
