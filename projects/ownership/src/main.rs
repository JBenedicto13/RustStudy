// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s; // BIG PROBLEM
//     let r2 = &mut s; // BIG PROBLEM
//     // let r1 = &s; // no problem
//     // let r2 = &s; // no problem
    

//     println!("{}, {}, and {}", r1, r2);
// }


fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}