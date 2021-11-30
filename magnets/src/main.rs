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
    let num_of_lines = read_n_lines(1)[0].parse::<usize>().unwrap();

    let all = read_n_lines(num_of_lines)
        .iter()
        .map(|line| {
            let magnet = line.trim();

            match magnet {
                "01" => true,
                "10" => false,
                _ => panic!("Invalid magnet"),
            }
        })
        .collect::<Vec<bool>>();

    let mut num_groups = 1;
    let mut previous = all[0];

    all.iter().for_each(|&magnet| {
        if previous ^ magnet {
            num_groups += 1;
        }
        previous = magnet;
    });

    print!("{}", num_groups);
}
