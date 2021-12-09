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

#[derive(Debug)]
struct Employee {
<<<<<<< HEAD
    low: i32,
    high: i32,
    salary: i32,
=======
    low: i64,
    high: i64,
>>>>>>> 5ab71a413c537f8d8658e456e1dda33871bb5c60
}

#[derive(Debug)]
struct TestCase {
    n: i64,
    s: i64,
    e: Vec<Employee>,
}

fn main() {
    let t: i32 = read_n_lines(1)[0].parse().unwrap();

    let mut test_cases: Vec<TestCase> = Vec::new();

    for i in 0..t {
        let n_and_s: Vec<i64> = read_n_lines(1)[0]
            .split(" ")
            .map(|num| num.parse().unwrap())
            .collect();
        test_cases.push(TestCase {
            n: n_and_s[0],
            s: n_and_s[1],
            e: Vec::new(),
        });
        for _ in 0..n_and_s[0] {
            let l_and_r: Vec<i64> = read_n_lines(1)[0]
                .split(" ")
                .map(|num| num.parse().unwrap())
                .collect();
            test_cases[i as usize].e.push(Employee {
                low: l_and_r[0],
                high: l_and_r[1],
<<<<<<< HEAD
                salary: l_and_r[0],
=======
>>>>>>> 5ab71a413c537f8d8658e456e1dda33871bb5c60
            });
        }
    }

<<<<<<< HEAD
    test_cases.iter_mut().for_each(|test_case| {
        test_case.e.sort_by(|a, b| a.salary.cmp(&b.salary));
        println!("{:?}", &test_case);

        let mut median = (test_case.e.len() as f32 / 2.0f32).ceil() as usize - 1 as usize;

        // can fail
        loop {
            if test_case.e[median].salary > test_case.e[median + 1].salary {
                test_case.e.sort_by(|a, b| a.salary.cmp(&b.salary));
            } else {
                if test_case.e[median].salary < test_case.e[median].high {
                    test_case.e[median].salary += 1;
                } else {
                    panic!("{:?}", test_case);
=======
    test_cases
        .iter_mut()
        .enumerate()
        .for_each(|(_test_pos, test_case)| {
            test_case.e.sort_by(|a, b| b.low.cmp(&a.low));
            // println!("{:?}", &test_case);

            let mut low = 1;
            let mut high = 10e9 as i64 + 1;

            while high - low > 1 {
                let mid = (low + high) / 2;
                let mut remaining_money = test_case.s;
                let mut median_pos = (test_case.n as f64 / 2f64).ceil() as i32;

                for employee in &test_case.e {
                    let mut temp_salary = employee.low;

                    if median_pos > 0 && employee.high >= mid {
                        median_pos -= 1;
                        temp_salary = mid.max(employee.low);
                    }

                    remaining_money -= temp_salary;
                }

                if remaining_money >= 0 && median_pos == 0 {
                    low = mid;
                } else {
                    high = mid;
>>>>>>> 5ab71a413c537f8d8658e456e1dda33871bb5c60
                }
            }

            // println!("Case #{}: low: {}, high: {}", test_pos + 1, low, high);
            println!("{}", low);
        });

    // println!("{:?}", &test_cases);
}
