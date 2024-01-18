use rand::Rng;
use std::cmp::Ordering;
use std::io;
// 需要导入的库 会有隐式预导入的内容 比如下面的String

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        // mut表示可变的变量 因为变量默认不可变
        // :: 叫关联函数 也叫静态函数/静态方法
        let mut guess = String::new();

        // &表示是引用 且默认不可变 mut表示可变
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust 允许用一个新值来 隐藏 （Shadowing） guess 之前的值
        // trim 方法会消除 \n 或者 \r\n，只留下 5。
        // u32 注解以及与 secret_number 的比较，意味着 Rust 会推断出 secret_number 也是 u32 类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // read_line 返回枚举类型 表示多种状态
        // 出现错误 expect 会使程序直接崩溃
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
