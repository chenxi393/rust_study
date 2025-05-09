fn main() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x

    foo1();
    str_to_i32()
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

pub fn foo1() {
    let mut x: String; // 未初始化的变量
                       // println!("Value: {}", x);
                       //编译阶段报错： compile error:`x` used here but it is possibly-uninitialized

    x = "1234".to_string(); // 可正常获取所有权
    println!("Value: {}", x); // 正常打印
}

fn str_to_i32() {
    //对于Option解包，提供了match关键字和各种封装了match的API
    let mut str_val: Option<String> = None;
    let real_val: i32 = str_val.and_then(|val| val.parse::<i32>().ok()).unwrap_or(0);

    println!("{}", real_val);

    str_val = None;
    //如果你执意直接使用unwrap也是可以运行时panic的
    //但这个是不符合语义预期的，因为不论是
    //因此我们推荐禁用unwarp,这几乎可以消除所有的运行时空指针问题
    let v: i32 = str_val.unwrap().parse::<i32>().unwrap();
}
