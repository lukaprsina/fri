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
    l: i32,
    r: i32,
    s: i32,
}

#[derive(Debug)]
struct TestCase {
    n: i32,
    s: i32,
    e: Vec<Employee>,
}

fn main() {
    let t: i32 = read_n_lines(1)[0].parse().unwrap();

    let mut test_cases: Vec<TestCase> = Vec::new();

    for i in 0..t {
        let n_and_s: Vec<i32> = read_n_lines(1)[0]
            .split(" ")
            .map(|num| num.parse().unwrap())
            .collect();
        test_cases.push(TestCase {
            n: n_and_s[0],
            s: n_and_s[1],
            e: Vec::new(),
        });
        for _ in 0..n_and_s[0] {
            let l_and_r: Vec<i32> = read_n_lines(1)[0]
                .split(" ")
                .map(|num| num.parse().unwrap())
                .collect();
            test_cases[i as usize].e.push(Employee {
                l: l_and_r[0],
                r: l_and_r[1],
                s: 0,
            });
        }
    }

    test_cases.iter_mut().for_each(|test_case| {
        test_case.e.sort_by(|a, b| a.l.cmp(&b.l));
        println!("{:?}", &test_case);

        let mut median = (test_case.e.len() as f32 / 2.0f32).ceil() as i32 - 1;
        let mut temp_sal = test_case.e[median as usize].l;

        // test_case.e.iter().enumerate().for_each(|(pos, e)| {});

        let found = false;
        while !found {
            if temp_sal < test_case.e[(median as usize) + 1].s {
                test_case.e[median as usize].s += 1;
            }
        }
    });

    println!("{:?}", &test_cases);
}
