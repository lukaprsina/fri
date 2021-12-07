fn get_char(n: u8) -> char {
    match n {
        0..=26 => ('a' as u8 + n) as char,
        27..=36 => (n - 27) as char,
        a => panic!("Invalid number {}", a),
    }
}

fn main() {
    let s = generate(2);
    println!("{}", s);
}

fn generate(n: u8) -> String {
    let mut result = String::new();
    let mut temp = String::new();
    // useless

    for i in 0..n {
        temp = result.clone();
        temp.push(get_char(i as u8));
        temp.push_str(&result);

        result = temp;
    }
    result
}
