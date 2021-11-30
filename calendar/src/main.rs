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
    let n = read_n_lines(1)[0].parse::<usize>().unwrap();
    let mut names = read_n_lines(n);
    let separator = &read_n_lines(1)[0];

    names.sort_by(|a, b| {
        let mut first = a.clone();
        first.push_str(separator);
        first.push_str(b);

        let mut second = b.clone();
        second.push_str(separator);
        second.push_str(a);

        first.cmp(&second)
    });

    let mut results: Vec<String> = Vec::new();

    names.iter_mut().enumerate().for_each(|(pos, name)| {
        if pos % 2 == 0 {
            results.push(name.to_string());
        } else {
            let last = results.last_mut().unwrap();
            last.push_str(separator);
            last.push_str(name);
        }
    });

    println!("{}", results.join("\n"));
}
