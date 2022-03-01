use std::io;

fn main() {
    println!("Hello, world! this is rust project");

    println!("请猜测一个数字:");
    println!("请输入你的猜测数字");

    let mut guess = String::new();
    io:: stdin()
        .read_line(&mut guess)
        .expect("读取失败");
    println!("你输入的数字是: {}", guess);
}
