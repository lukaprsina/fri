use core::cmp::max;
use std::io;

fn get_char(n: u8) -> char {
    match n {
        0..=26 => ('a' as u8 + n) as char,
        27..=36 => (n - 27) as char,
        a => panic!("Invalid number {}", a),
    }
}

// abacabadabacaba
/*
a: 1, 3, 5, 7 (y = 2x + 1) 0
b: 2, 6, 10 (y = 4x + 2) 1
c: 4, 12 (y = 8x + 4) 2
d: 8 (y = 16x + 8) 3
*/

fn into_aba(index: i64) -> i64 {
    let mut n = 1i64;

    loop {
        let kx = index - n;
        let into_char = kx as f64 / (n as f64 * 2f64);
        if into_char.fract() == 0f64 {
            return (n as f64).log(2f64).floor() as i64;
        }

        n *= 2i64;
    }
}

fn read_n_lines(n: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        lines.push(line.trim().to_string());
    }
    lines
}

fn longest_common_substring(first: &str, second: &str) -> i64 {
    let mut result = 0i64;
    let mut suff: Vec<Vec<i64>> = vec![vec![0; second.len() + 1]; first.len() + 1];

    for i in 0..=first.len() {
        for j in 0..=second.len() {
            if i == 0 || j == 0 {
                suff[i][j] = 0;
            } else if first.chars().nth(i - 1).unwrap() == second.chars().nth(j - 1).unwrap() {
                suff[i][j] = suff[i - 1][j - 1] + 1;
                result = result.max(suff[i][j]);
            } else {
                suff[i][j] = 0;
            }
        }
    }

    /* println!(
        "{:?}",
        suff.iter().for_each(|line| {
            println!("{:?}", line);
        })
    ); */

    result
}

fn main() {
    let input: Vec<usize> = read_n_lines(1)[0]
        .split(" ")
        .map(|x| x.parse::<usize>().expect("Not an integer!"))
        .collect();

    let first = (input[0], input[1]);
    let second = (input[2], input[3]);

    let mut first_str = String::new();
    for i in first.0..=first.1 {
        first_str.push(get_char(into_aba(i as i64) as u8));
    }

    let mut second_str = String::new();
    for i in second.0..=second.1 {
        second_str.push(get_char(into_aba(i as i64) as u8));
    }

    /* println!(
        "a: {}, b: {}, num: {}",
        &first_str,
        &second_str,
        longest_common_substring(&first_str, &second_str)
    ); */

    println!("{}", longest_common_substring(&first_str, &second_str));
}
