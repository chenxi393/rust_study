#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // let rect2 = Rectangle {
    //     height: 50,
    //     ..rect1
    // };
    println!("rect1 is {rect1:#?}");
}
