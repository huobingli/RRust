fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1

    // 不能输出元组中的元素的,并且会报错
    /*
    error[E0277]: `(i32, f64, u8)` doesn't implement `std::fmt::Display`
    --> tuple.rs:17:21
    |
    17 |     println!("{ }", tup);
    |                     ^^^ `(i32, f64, u8)` cannot be formatted with the default formatter
    |
    = help: the trait `std::fmt::Display` is not implemented for `(i32, f64, u8)`
    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

    error: aborting due to previous error
    */
    // println!("{ }", tup);
    println!("{:?}", tup);
    let (x, y, z) = tup;

    println!("integer is :{:?}", tup.0);
    println!("float is :{:?}", tup.1);
    println!("unsigned integer is :{:?}", tup.2);
    // y 等于 6.4
    println!("{0}, {1}, {2}", x, y, z);
}
