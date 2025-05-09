fn main() {
    let str_val: Option<String> = Some("123".to_string());
    let real_val: Option<i32> = str_val.and_then(|val| val.parse::<i32>().ok());
    println!("{:?}", real_val); // 输出: Some(123)

    let str_val: Option<String> = Some("abc".to_string());
    let real_val: Option<i32> = str_val.and_then(|val| val.parse::<i32>().ok());
    println!("{:?}", real_val); // 输出: None

    let str_val: Option<String> = None;
    let real_val: Option<i32> = str_val.and_then(|val| val.parse::<i32>().ok());
    println!("{:?}", real_val); // 输出: None
}