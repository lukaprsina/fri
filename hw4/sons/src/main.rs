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
    println!("NEWWWWWW\n\n\n");
    let numbers: Vec<usize> = read_n_lines(1)[0]
        .split(" ")
        .map(|number_str| number_str.parse::<usize>().unwrap())
        .collect();
    let n = numbers[0];
    let _m = numbers[0];

    let field = read_n_lines(n)
        .iter()
        .map(|n_iter| {
            n_iter
                .split(" ")
                .map(|number_str| number_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let portions = read_n_lines(1)[0]
        .split(" ")
        .map(|portion| portion.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut result = 0;
    calculate_row(&field, &portions, &mut result);
    println!("{}", result);
}

fn calculate_row(field: &Vec<Vec<usize>>, portions: &Vec<usize>, result: &mut i32) -> bool {
    let mut found: bool = false;
    if portions.len() == 0 {
        found = true;
    }

    field.iter().enumerate().for_each(|(row_num, row)| {
        let sum = row.iter().sum();

        if let Some(portion_num) = portions.iter().position(|&x| x == sum) {
            let mut temp_portions = portions.clone();
            let mut temp_field = field.clone();

            println!("New iteration:\n{:?}\n{:?}", field, portions);

            temp_portions.remove(portion_num);
            temp_field.remove(row_num);

            found = calculate_row(&temp_field, &temp_portions, result);
        }

        println!("Ending row: {:?}\n\n", row_num);
    });

    if found {
        *result += 1;
        println!("REEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE: {}", result);
    }

    found
}
