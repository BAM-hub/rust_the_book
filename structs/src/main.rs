struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = build_user(String::from("bshar@email.com"), String::from("BAM"));
    let mut user2 = User {
        email: String::from("newEmail.mail.com"),
        ..user1
    };
    user1.email = String::from("myEmail@gmail.com");

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
