use std::io::{BufRead, Write};

use dropspot_core::user::{create_user, login};

use crate::auth::storage::save_login;

pub async fn handle_login() -> Result<(), ()> {
    let mut stdout = std::io::stdout().lock();
    let mut stdin = std::io::stdin().lock();

    write!(stdout, "Please enter your email: ").expect("Could not prompt for email");
    stdout.flush().expect("Could not flush terminal");

    let mut email = String::new();
    stdin
        .read_line(&mut email)
        .expect("Could not read terminal input");
    email = email.trim().to_owned();

    write!(stdout, "Please enter your password: ").expect("Could not prompt for password");
    stdout.flush().expect("Could not flush terminal");

    let config = rpassword::ConfigBuilder::new()
        .output_discard()
        .password_feedback_mask('*')
        .output_writer(stdout) // Passing in input_reader causes the password to display in the terminal
        .build();

    let password = rpassword::read_password_with_config(config)
        .expect("Could not read password")
        .trim()
        .to_owned();

    let login_result = match login(email, password).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Login error: {e}");
            return Err(());
        }
    };

    println!("Logged in as {}!", login_result.user.email);

    if let Err(e) = save_login(&login_result.tokens.refresh_token) {
        eprintln!("Failed to save login token: {e}");
        return Err(());
    }

    Ok(())
}

pub async fn handle_create_user() -> Result<(), ()> {
    let mut stdout = std::io::stdout().lock();
    let mut stdin = std::io::stdin().lock();

    write!(stdout, "Please enter your email: ").expect("Could not prompt for email");
    stdout.flush().expect("Could not flush terminal");

    let mut email = String::new();
    stdin
        .read_line(&mut email)
        .expect("Could not read terminal input");
    email = email.trim().to_owned();

    write!(stdout, "Please enter your first name: ").expect("Could not prompt for first name");
    stdout.flush().expect("Could not flush terminal");

    let mut first_name = String::new();
    stdin
        .read_line(&mut first_name)
        .expect("Could not read terminal input");
    first_name = first_name.trim().to_owned();

    write!(stdout, "Please enter your last name: ").expect("Could not prompt for last name");
    stdout.flush().expect("Could not flush terminal");

    let mut last_name = String::new();
    stdin
        .read_line(&mut last_name)
        .expect("Could not read terminal input");
    last_name = last_name.trim().to_owned();

    write!(stdout, "Please enter your password: ").expect("Could not prompt for password");
    stdout.flush().expect("Could not flush terminal");

    let config = rpassword::ConfigBuilder::new()
        .output_discard()
        .password_feedback_mask('*')
        .output_writer(stdout) // Passing in input_reader causes the password to display in the terminal
        .build();

    let password = rpassword::read_password_with_config(config)
        .expect("Could not read password")
        .trim()
        .to_owned();

    let login_result = match create_user(email, password, first_name, last_name).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("User creation error: {e}");
            return Err(());
        }
    };

    println!(
        "Created user {}! All DropSpot commands will now be undertaken as this user",
        login_result.user.email
    );

    if let Err(e) = save_login(&login_result.tokens.refresh_token) {
        eprintln!("Failed to save login token: {e}");
        return Err(());
    }

    Ok(())
}
