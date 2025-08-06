fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    // add some values
    vec1.push(14);
    vec1.push(11);
    vec1.push(12);
    vec1.push(13);
    vec1.push(17);

    // sort all of them
    vec1.sort();

    println!("{vec1:?}");

    // try to get a value with internal function. will give back a None if there is no value by
    // index. Useful to avoid error of non-existing index
    let var1 = vec1.get(100);

    println!("{var1:?}");

    // change all values to 15
    //  ... here is some black magic with dereferencing which is not quite understandable right now
    for i in &mut vec1 {
        *i = 15;
    }

    println!("{vec1:?}");
}
