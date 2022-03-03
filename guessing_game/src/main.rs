use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("这是一个小小的猜数字游戏...");
    println!("\n请猜测一个数字:");
    let secrect_number = rand::thread_rng().gen_range(1..101);
    println!("秘密数字是: {}", secrect_number);

    loop {
        println!("\n请输入你的猜测数字");
        let mut guess = String::new();
        io:: stdin()
            .read_line(&mut guess)
            .expect("读取失败");
        println!("你输入的数字是: {}", guess);
        let guess: u32 = guess.trim().parse().expect("请输入一个数字");
        println!("你输入的数字是: {}", guess);
        match guess.cmp(&secrect_number) {
            Ordering::Less => println!("你猜的数字太小了"),
            Ordering::Greater => println!("你猜的数字太大了"),
            Ordering::Equal => {
                println!("你猜对了");
                break;
            }
        }
    }



}
