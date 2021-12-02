use std::{
    fs::{write, File},
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let lines = lines_from_file("input.txt");
    let numbers: Vec<usize> = lines[0]
        .split(" ")
        .map(|number_str| number_str.parse::<usize>().unwrap())
        .collect();
    let n = numbers[0];
    let m = numbers[1];

    let field = lines[1..(1 + n)]
        .iter()
        .map(|n_iter| {
            n_iter
                .split(" ")
                .map(|number_str| number_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let portions = lines[1 + n]
        .split(" ")
        .map(|portion| portion.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut result = 0;
    calculate_row(&field, &portions, &mut result);

    let mut temp_field: Vec<Vec<usize>> = Vec::new();

    for i in 0..m {
        let mut temp_column = Vec::new();
        for j in 0..n {
            temp_column.push(field[j][i]);
        }
        temp_field.push(temp_column);
    }

    calculate_row(&temp_field, &portions, &mut result);
    write("output.txt", result.to_string()).unwrap();
}

fn calculate_row(field: &Vec<Vec<usize>>, portions: &Vec<usize>, result: &mut i32) {
    let mut sum = 0;

    if field.len() == 0 && portions.len() == 0 {
        *result += 1;
    }

    for (row_num, row) in field.iter().enumerate() {
        sum += row.iter().sum::<usize>();

        if let Some(portion_num) = portions.iter().position(|&x| x == sum) {
            let mut temp_portions = portions.clone();
            let mut temp_field = field.clone();

            temp_portions.remove(portion_num);

            for _ in 0..=row_num {
                temp_field.remove(0);
            }

            calculate_row(&temp_field, &temp_portions, result);
        }
    }
}
