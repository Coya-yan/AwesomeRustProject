use std::io;
fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("x is {}", x);
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner x is {}", x);
    }
    println!("x is {}", x);

    // 加法
    let sum = 5 + 10;
    // 减法
    let difference = 95.5 - 4.3;
    // 乘法
    let product = 4 * 30;
    // 除法
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 结果为0
    // 取余
    let remainder= 43 % 5;
    let t = true;
    let f: bool = false; // 布尔值可以用括号括起来

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("y 的数值为 {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3;5];
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    a: [1, 2, 3, 4, 5];

    x = x + 1;

    let a = [1, 2, 3, 4, 5];
    println!("请输入阵列的索引");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("读取索引失败");

    let index : usize = index
        .trim()
        .parse()
        .expect("索引必须是数字");

    let element = a[index];
    println!("索引为 {} 的元素为 {}", index, element);

    io::stdin()
        .read_line(&mut index)
        .expect("读取索引失败");
        
    index = index.trim().parse().expect("索引必须是数字");
}
