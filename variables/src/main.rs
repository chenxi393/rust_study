const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // 变量默认是不可变的
    println!("The value of x is: {x}");

    let s = "测试";
    println!("{}", s.len());

    // 编译器无法推断类型 添加注解
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    println!("{THREE_HOURS_IN_SECONDS}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c}");
    println!("{z}");
    println!("{heart_eyed_cat}");

    print_labeled_measurement(5, 'h');

    let x = plus_one(5);

    println!("The value of x is: {x}");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}