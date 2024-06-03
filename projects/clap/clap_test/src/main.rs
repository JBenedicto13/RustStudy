use clap::{command, Arg, Command, ArgMatches};

fn main() {
    let match_result = command!()
    .subcommand(
        Command::new("register-person")
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .aliases(["fname"])
                .required(true)
                .help("The person's first name")
                // .conflicts_with("lastname")
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .aliases(["lname"])
                .required(true)
                .help("The person's last name")
        )
    )
    .subcommand(
        Command::new("register-pet")
        .arg(
            Arg::new("pet-name")
                .long("pet-name")
                .short('n')
                .required(true)
        )
    )
    .about("This application registers people with their doctor's office")
    .arg(
        Arg::new("fluffy")
            .long("fluffy")
            .help("Person wearing fluffy coat or maybe not")
    )
    .get_matches();

    // println!("Petname: {}", match_result.get_one::<String>("pet-name").unwrap_or("NO PET NAME".to_string()));
    // let pet_args: Option<&ArgMatches> = match_result.subcommand_matches("register-pet");
    // println!("Does pet name exists? {}", pet_args.unwrap().get_one::<String>("pet-name"));

    let person_args: &ArgMatches = match_result.subcommand_matches("register-person").unwrap();

    let fname: &String = person_args.get_one::<String>("firstname").unwrap();
    let lname: &String = person_args.get_one::<String>("lastname").unwrap();

    println!("Firstname: {} Lastname: {}", fname, lname);    
}
