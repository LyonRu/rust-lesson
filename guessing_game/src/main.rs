use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, Here is a guessing game");
    println!("first guess");


    let secret_num = rand::thread_rng().gen_range(1,100);
    println!("the secret number is {}",secret_num);
    //todo here is a problem
    //let mut guess = String::new();
    loop{
       let mut guess = String::new();
        println!("guess {}" ,guess);
        std::io::stdin().read_line(&mut guess).expect("fail in read line");
        println!("you guess {} ",guess);
        let guess :u32 = match guess.trim().parse(){
            Ok(num)=>{
              println!("number is {}",num);
                num
            },
            Err(msg)=>{
                println!("msg {}",msg);
                continue
            }
        };
        match  guess.cmp(&secret_num){
            Ordering::Less=>println!("too small"),
            Ordering::Greater=>println!("too big"),
            Ordering::Equal=>{
                println!("You Win!");
            }
        }

    }

}
