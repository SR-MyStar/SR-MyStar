use dialoguer::Input;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("欢迎来到猜数游戏！");
    let number: u8 = rand::thread_rng().gen_range(1..=100);
    let mut count: u32 = 0;
    loop {
        let guess: u8 = match Input::new()
            .with_prompt("输入一个1-100之间的数字")
            .interact_text()
        {
            Err(_) => continue,
            Ok(input) => input,
        };
        count += 1;
        match guess.cmp(&number) {
            Ordering::Less => println!("小了！"),
            Ordering::Greater => println!("大了！"),
            Ordering::Equal => {
                println!("恭喜你获胜！共用{}次", count);
                break;
            }
        };
    }
}
