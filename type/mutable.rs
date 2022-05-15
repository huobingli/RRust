fn muti() {
    let mut a = 123;
    println!("{}", a);
    a = 456;
    println!("{}", a);
}

fn main() {
    vari();
    muti();
}

fn vari() {
    let a = 123;
    println!("{}", a);

    // !! error !! cannot assign twice to immutable variable
    // a = 456;
    // println!("{}", a);
}
