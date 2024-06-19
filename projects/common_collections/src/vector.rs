pub fn vector_mod {
    // Vector
    let v: Vec<i32> = Vec::new(); //Initiating a vector with no values

    let v = vec![1, 2, 3, 4, 5]; //Initiating a vector with values

    let mut v = Vec::new();

    v.push(5); //Adding values to a vector
    v.push(6);
    v.push(7);
    // v.push(8);


    println!("Vector collection:\n{:?}", v);
    let third:&i32 = &v[2];
    println!("The third element is {third}");

    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("There is no fourth element."),
    }

    let mut items = vec![100, 32, 57];
    for i in &mut items {
        *i += 50;
    }

    println!("Items:\n{:?}", items);
}