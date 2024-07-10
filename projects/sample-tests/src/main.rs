fn main() {
    println!("{}", get_full_name("JB*", "^Benedicto"));
}

fn get_full_name(first: &str, last: &str) -> String {
    let pattern = &['^', '$', '*', '&'];
    if first.contains(pattern) {
        panic!("First name cannot contain special characters!");
    }
    if last.contains(pattern) {
        panic!("Last name cannot contain special characters!");
    }
    let mut result: String = "".to_string();
    result.push_str(first);
    result.push_str(" ");
    result.push_str(last);

    return result;
}

#[cfg(test)]
mod mytests {
    #[test]
    fn test_get_full_name_normal_input() {
        super::get_full_name("JB", "Benedicto");
    }
    assert_eq!(result, "JB Benedicto");

    #[test]
    #[should_panic]
    fn test_get_full_name_special_chars() {
        _ = super::get_full_name("JB", "Benedicto$");
    }
}
