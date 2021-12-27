// use std::cmp::Ordering;
// use std::io;
// use rand::{thread_rng, Rng};
use std::time::{Instant};
fn main() {
    let istop = Instant::now();
    println!("{}",fibonacci(49));
    let elapsed_time = istop.elapsed();
    println!("std::time:Instant::now() overhead = {:?}",
    elapsed_time.as_secs());
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
