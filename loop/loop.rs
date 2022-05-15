fn main() {
    let s = ['H', 'E', 'L', 'L', 'O', 'R', 'U', 'S', 'T'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'T' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }
}