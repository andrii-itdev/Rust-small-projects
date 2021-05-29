
struct User{
    username: String,
    email: String,
    sign_in_account: u64,
    active: bool
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_account: 1,
    }
}

fn print_user(user : &User)
{
    println!("{} {} {} {}", 
        user.email, user.username, 
        user.sign_in_account, user.active);
}

fn main() {
    let mut user1 = User{
        email: String::from("someone@exmple.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_account: 1
    };
    user1.username = String::from("superuser");
    print_user(&user1);

    let user1 = build_user(String::from("Andrii"), String::from("Yurov"));
    print_user(&user1);

    let user2 = User{
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..user1
    };
    print_user(&user2);
}
