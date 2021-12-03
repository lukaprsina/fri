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

fn main() {}
