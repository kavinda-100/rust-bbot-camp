use colorize::AnsiColor;
use inquire::CustomUserError;
use inquire::error::InquireResult;
use inquire::validator::Validation;

/// This is example of use of inquire crate
pub fn start() {
    // prompt user for inputs using inquire crate
    let mut prompt_results: Vec<String> = Vec::new();

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
    let player_name = inquire::Text::new("Please enter your player name (3-20 characters):")
        .with_validator(player_name_validation)
        .with_help_message("This will be your player name in the game.")
        .with_initial_value("Player")
        .prompt();
    match player_name {
        Ok(name) => {
            prompt_results.push(name);
        }
        Err(e) => {
            println!("{0}", format!("Error: {}", e).red());
            return;
        }
    }
    // ask player's age
    let player_age = inquire::Text::new("Please enter your player age (number):")
        .with_validator(player_age_validation)
        .with_help_message("This will be your player age in the game.")
        .with_initial_value("18")
        .prompt();
    match player_age {
        Ok(age) => {
            prompt_results.push(age);
        }
        Err(e) => {
            println!("{0}", format!("Error: {}", e).red());
            return;
        }
    }

    println!("Prompt results: {:#?}", prompt_results);
}
