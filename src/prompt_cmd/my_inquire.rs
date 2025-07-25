use colorize::AnsiColor;
use inquire::CustomUserError;
use inquire::validator::Validation;
use std::string::String;

/// This is example of use of inquire crate
pub fn start() {
    // prompt user for inputs using inquire crate
    let mut player_name: String = String::new();
    let mut player_age: String = String::new();
    let mut player_region: String = String::new();
    let mut player_class: Vec<String> = Vec::new();
    let mut player_dob: String = String::new();

    let regions = vec![
        "Europe".to_string(),
        "Asia".to_string(),
        "North America".to_string(),
        "South America".to_string(),
        "Africa".to_string(),
        "Australia".to_string(),
    ];
    let player_class_list = vec![
        "Warrior".to_string(),
        "Mage".to_string(),
        "Rogue".to_string(),
        "Paladin".to_string(),
        "Hunter".to_string(),
        "Druid".to_string(),
        "Archer".to_string(),
    ];

    //? ========================= Validate inputs with inquire =========================
    // validate player name
    let player_name_validation = |input: &str| -> Result<Validation, CustomUserError> {
        return if input.is_empty() {
            Ok(Validation::Invalid("Player name cannot be empty".into()))
        } else if input.len() > 20 {
            Ok(Validation::Invalid(
                "Player name must be less than 20 characters long".into(),
            ))
        } else if input.len() < 3 {
            Ok(Validation::Invalid(
                "Player name must be at least 3 characters long".into(),
            ))
        } else {
            Ok(Validation::Valid)
        };
    };
    // validate player age
    let player_age_validation = |input: &str| -> Result<Validation, CustomUserError> {
        return if input.is_empty() {
            Ok(Validation::Invalid("Player age cannot be empty".into()))
        } else if input.parse::<u32>().is_err() {
            Ok(Validation::Invalid("Player age must be a number".into()))
        } else {
            Ok(Validation::Valid)
        };
    };

    // ========================= Prompt with inquire =========================
    println!("{0}", "============================".green());

    let proceed = inquire::prompt_confirmation("Are you ready to proceed? (y/n)");
    match proceed {
        Ok(true) => {
            println!(" ");
        }
        Ok(false) => {
            println!("{0}", "No problem, take your time!".yellow());
            return;
        }
        Err(e) => {
            println!("{0}", format!("Error: {}", e).red());
            return;
        }
    }
    // ask player's name
    let player_name_res = inquire::Text::new("Please enter your player name (3-20 characters):")
        .with_validator(player_name_validation)
        .with_help_message("This will be your player name in the game.")
        .with_initial_value("Player")
        .prompt();
    match player_name_res {
        Ok(name) => {
            player_name = name;
        }
        Err(e) => {
            println!("{0}", format!("Error: {}", e).red());
            return;
        }
    }
    // ask player's age
    let player_age_res = inquire::Text::new("Please enter your player age (number):")
        .with_validator(player_age_validation)
        .with_help_message("This will be your player age in the game.")
        .with_initial_value("18")
        .prompt();
    match player_age_res {
        Ok(age) => {
            player_age = age;
        }
        Err(e) => {
            println!("{0}", format!("Error: {}", e).red());
            return;
        }
    }
    // ask player's region
    let player_region_res = inquire::Select::new("Please select your region:", regions)
        .with_help_message("This will be your player region in the game.")
        .prompt();
    match player_region_res {
        Ok(region) => {
            player_region = region;
        }
        Err(e) => {
            println!("{0}", format!("Error: {}", e).red());
            return;
        }
    }
    // ask player's class (multiple choice)
    let player_class_res =
        inquire::MultiSelect::new("Please select your class:", player_class_list)
            .with_help_message("You can select multiple classes.")
            .prompt();
    match player_class_res {
        Ok(classes) => {
            // Convert Vec<String> to Vec<String> for consistency
            // let classes: Vec<String> = classes.into_iter().map(|s| s.to_string()).collect();
            player_class = classes
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
        }
        Err(e) => {
            println!("{0}", format!("Error: {}", e).red());
            return;
        }
    }
    // ask player's date of birth
    let player_dob_res = inquire::DateSelect::new("Please select your date of birth:")
        .with_help_message("This will be your player date of birth in the game.")
        .prompt();
    match player_dob_res {
        Ok(dob) => {
            // Format the date as a string
            player_dob = dob.format("%Y-%m-%d").to_string();
        }
        Err(e) => {
            println!("{0}", format!("Error: {}", e).red());
            return;
        }
    }

    // ========================= Display the results =========================
    #[derive(Debug)]
    struct Player {
        name: String,
        age: String,
        region: String,
        class: Vec<String>,
        dob: String,
    }
    impl Player {
        fn new(name: String, age: String, region: String, class: Vec<String>, dob: String) -> Self {
            Player {
                name,
                age,
                region,
                class,
                dob,
            }
        }

        fn display(&self) {
            println!("{0}", "============================".green());
            println!(
                "{0}",
                "============= Player Details ===============".yellow()
            );
            println!("Player Name: {}", self.name);
            println!("Player Age: {}", self.age);
            println!("Player Region: {}", self.region);
            println!("Player Class: {:?}", self.class);
            println!("Player Date of Birth: {}", self.dob);
            println!("{0}", "============================".green());
        }
    }

    // Create a new player instance
    let player = Player::new(
        player_name,
        player_age,
        player_region,
        player_class,
        player_dob,
    );
    // Display the player details
    player.display();
    println!("{0}", "============================".green());
    println!("{0}", "You are ready for playing!".green());
    println!("{0}", "Good Luck!".red());
    println!("{0}", "============================".green());
    println!();
}
