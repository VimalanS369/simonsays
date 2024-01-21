use std::env;
use rand::Rng;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let num = rand::thread_rng().gen_range(1..=2);
    if args.len() !=0 {
        if num%2 == 1 {
            println!("Simon Says {}",args.join(" "));
        }
        else{
        println!("{}",args.join(" "));
        }
    }
    else{
        println!("");
    }
}
