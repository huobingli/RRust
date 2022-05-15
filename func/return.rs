fn main () {
    let sum = add(1, 2);
    println!("add sum: {}", sum)
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}