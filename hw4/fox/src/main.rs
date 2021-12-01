use std::io;

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
    let input: Vec<usize> = read_n_lines(1)[0]
        .split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    let row = input[0];
    let col = input[1];

    for i in 0..row {
        if i % 2 == 0 {
            print!("{}", "#".repeat(col));
        } else if i % 4 == 1 {
            print!("{}", ".".repeat(col - 1));
            print!("#");
        } else if i % 4 == 3 {
            print!("#");
            print!("{}", ".".repeat(col - 1));
        }

        if i != (row - 1) {
            println!();
        }
    }
}
