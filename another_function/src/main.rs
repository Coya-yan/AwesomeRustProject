fn main() {
    println!("Hello, world!");
    another_function(5);
    let y = {
        let x = 3;
        x + 1
    };

    let x = five();

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn another_function(x: i32) {
    println!("x  的数值为{}", x);
}

fn print_labeled_measurement(value:i32, unit_label: char) {
    println!("测量值为 {} {}", value, unit_label);
}

fn five()-> i32 {
    5
}
