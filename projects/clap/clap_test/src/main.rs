use clap::{command, Arg};

fn main() {
    let match_result = command!()
    .arg(
        Arg::new("firstname")
            .short('f')
            .long("first-name")
    )
    .arg(
        Arg::new("lastname")
            .short('l')
            .long("last-name")
    )
    .arg(
        Arg::new("fluffy")
            .long("fluffy")
    )
    .get_matches();
}
