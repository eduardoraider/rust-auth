mod authentication;

fn main() {
    let mut user = authentication::User::new("eduardo", "super-secret");

    println!("The username is: {}", user.get_username());
    user.set_password("even-more-secret-password");
}