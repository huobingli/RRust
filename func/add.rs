fn main() {
    add(5, 6);
}

fn add(x: i32, y: i32) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);

    let sum = x + y;
    println!("sum 的值为 : {}", sum);
}