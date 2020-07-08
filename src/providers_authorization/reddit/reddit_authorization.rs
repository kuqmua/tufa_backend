use roux::Reddit;

pub fn reddit_authorization(
    user_agent: &str,
    client_id: &str,
    client_secret: &str,
    username: &str,
    password: &str,
) {
    Reddit::new(user_agent, client_id, client_secret)
        .username(username)
        .password(password)
        .login()
        .unwrap(); // вынести в функцию
    println!("success reddit authorization");
}
