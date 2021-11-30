use std::{collections::HashMap, io};

fn read_n_lines(n: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        lines.push(line.trim().to_string());
    }
    lines
}

fn main() {
    let input = &read_n_lines(1)[0];
    let substring = &read_n_lines(1)[0];

    let sub_length = substring.len();

    let mut num_anagrams = 0;
    let mut num_empty = 0;
    let mut input_chars: HashMap<char, i32> = HashMap::new();
    let sub_chars: Vec<char> = substring.chars().collect();

    let main_string = &input[..input.len() - sub_length + 1];

    main_string.chars().enumerate().for_each(|(position, _)| {
        let my_str = input[position..position + sub_length].to_string();

        my_str.chars().for_each(|letter| {
            if letter == '?' {
                num_empty += 1;
            } else {
                *input_chars.entry(letter).or_default() += 1;
            }
        });

        let has_letters = input_chars.iter_mut().all(|(x, length)| {
            *length = *length - 1;

            *length == 0 && sub_chars.contains(&x)
        });

        if has_letters && (num_empty + input_chars.len() == sub_length) {
            num_anagrams += 1;
        }

        num_empty = 0;
        input_chars.clear();
    });

    println!("{}", num_anagrams);
}
