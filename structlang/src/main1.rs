
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

fn main1() {
    let user1 = User {
        email: String::from("test@example.com"),
        username: String::from("admin"),
        active: true,
        sing_in_count: 1,
    };

    println!("{:?}", user1);
}
