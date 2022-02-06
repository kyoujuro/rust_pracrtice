use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
    user_info(String::from("hoge"));
}
struct User{
    name: String,
    id: i32,
    email: String,
    active: bool
}
fn user_info(user_name: String){
    let first_name = User{
        name: user_name,
        id: i32::from(10),
        email: String::from("test"),
        active: bool::from(true)
    };
    println!("{}, {}, {}, {}", first_name.name
    , first_name.id, first_name.email, first_name.active);
}
