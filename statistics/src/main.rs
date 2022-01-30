// use std::cmp::Ordering;
//use std::io;
// use rand::{thread_rng, Rng};
use std::time::{Instant};
mod active_function;
#[derive(Debug)]
struct IpAddress{
    kind:String,
    address:String,
    num:u32,
}
impl IpAddress {
    fn area(&self) {
        println!("{}", self.address);
    }
    fn double(&self){
        println!("{}", self.num);
    }
}



fn main() {
    let istop = Instant::now();
    println!("{}",fibonacci(10));
    let elapsed_time = istop.elapsed();
    println!("std::time:Instant::now() overhead = {:?}",
    elapsed_time.as_secs());
    let hoge = IpAddress {
        kind: String::from("v4"),
        address: String::from("127.0.0.1"),
        num:10,
    };

    println!("{:?}",hoge);
    hoge.area();
    hoge.double();
    println!("{:?}", active_function::active_fn::relu(-2));
}

fn fibonacci(n: u64) -> u64{
    return match n{
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1)
    }
}

// fn practice(){
//     println!("Guess the number!");
//     let objective: Option<i32> =Some(0);
//     match objective {
//         Some(x) if x % 2 == 0 => println!("0"),
//         Some(_) => println!("1"),
//         None => println!("None"),
//     }
//     let mut rng = thread_rng();
//     let secret_number: u32 = rng.gen_range(1..101);
//     println!("{}", secret_number);
//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }
