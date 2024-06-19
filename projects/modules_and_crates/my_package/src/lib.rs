// pub mod my_module;
// pub mod outer;

pub fn greet() {
    // From Library
    // println!("Hello from the library crate!");
    // From Module
    // my_module::say_hello();
    // Bringing in a path
    // use crate::outer::inner;
    // inner::hello();
    // Separating Modules into Different Files
    // outer::inner::hello();
    my_dependency::dependency_function();
}

// pub mod outer {
//     pub mod inner {
//         pub fn hello() {
//             println!("Hello from inner module!");
//         }
//     }
// }
