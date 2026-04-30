use regex::Regex;
use std::fs::read_to_string;
use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Machine {
    lights: u64,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<i64>,
}

fn parse_machine(line: &str) -> Machine {
    // 1) lights -> integer
    let lights_str = line.split(']').next().unwrap().trim_start_matches('[');

    let lights = lights_str.chars().fold(0u64, |acc, c| {
        (acc << 1)
            | match c {
                '#' => 1,
                '.' => 0,
                _ => 0,
            }
    });
    // 2) vec of vecs from (…)
    let re_paren = Regex::new(r"\(([^)]*)\)").unwrap();
    let buttons = re_paren
        .captures_iter(line)
        .map(|cap| {
            cap[1]
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    // 3) vec from {…}
    let re_brace = Regex::new(r"\{([^}]*)\}").unwrap();
    let joltages = re_brace
        .captures(line)
        .map(|cap| {
            cap[1]
                .split(',')
                .map(|n| n.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .unwrap_or_default();

    Machine {
        lights,
        buttons,
        joltages,
    }
}

fn parse_input(lines: Vec<&str>) -> Vec<Machine> {
    lines.into_iter().map(|s| parse_machine(s)).collect()
}

fn lcm(a: i64, b: i64) -> i64 {
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a.abs()
    }
    (a / gcd(a, b)) * b
}

fn scale(v: &mut [i64], k: i64) {
    for x in v {
        *x *= k;
    }
}

fn print_matrix(matrix: Vec<Vec<i64>>) {
    for row in 0..matrix.len() {
        println!("{}:{:?}", row, matrix[row]);
    }
}
fn reduce_row(target_row: &mut [i64], source_row: &mut [i64], column: usize) {
    if target_row[column] != 0 {
        assert!(source_row[column] > 0);
        if target_row[column] < 0 {
            scale(target_row, -1)
        };
        let lcm: i64 = lcm(target_row[column], source_row[column]);
        scale(target_row, lcm / target_row[column]);
        scale(source_row, lcm / source_row[column]);
        for col in 0..target_row.len() {
            target_row[col] -= source_row[col]
        }
    }
}

fn swap(v: &mut [i64], w: &mut [i64]) {
    for i in 0..v.len() {
        let tmp: i64 = v[i];
        v[i] = w[i];
        w[i] = tmp;
    }
}

fn swap_rows(matrix: &mut Vec<Vec<i64>>, source: usize, target: usize) {
    let (lower, upper) = matrix.split_at_mut(target);
    swap(&mut lower[source], &mut upper[0])
}

fn reduce(matrix: &mut Vec<Vec<i64>>) {
    let diag_end = min(matrix.len(), matrix[0].len() - 1);
    for diag_index in 0..diag_end {
        for row in diag_index..diag_end {
            if matrix[row][diag_index] != 0 {
                if row != diag_index {
                    swap_rows(matrix, diag_index, row);
                }
                break;
            };
        }
        if matrix[diag_index][diag_index] < 0 {
            scale(&mut matrix[diag_index], -1)
        };
        for row in diag_index + 1..matrix.len() {
            let (lower, upper) = matrix.split_at_mut(row);
            reduce_row(&mut upper[0], &mut lower[diag_index], diag_index);
        }
    }
}

fn set_constraints(buttons: &Vec<Vec<usize>>, joltages: &Vec<i64>) -> Vec<i64> {
    let mut constraints: Vec<i64> = vec![10000000; buttons.len()];
    for group in 0..buttons.len() {
        for button in buttons[group].clone() {
            constraints[group] = min(constraints[group], joltages[button]);
        }
    }
    constraints
}

fn solve_matrix(
    matrix: &Vec<Vec<i64>>,
    row: usize,
    next: usize,
    constraints: Vec<i64>,
    solution: &mut Vec<i64>,
    minimum: &mut i64,
) {
    println!("solutions:{:?}", solution);
    if next > row {
        for guess in 0..=constraints[next] {
            solution[next] = guess;
            solve_matrix(
                matrix,
                row,
                next - 1,
                constraints.clone(),
                solution,
                minimum,
            );
        }
        return;
    }
    assert!(row == next);
    if matrix[row][next] == 0 {
        println!("matrix[{}][{}] = 0, must try values between 0 and {}", row, next, constraints[row]);
        for guess in 0..=constraints[next] {
            solution[next] = guess;
            if next > 0 {
                solve_matrix(
                    matrix,
                    row - 1,
                    next - 1,
                    constraints.clone(),
                    solution,
                    minimum,
                )
            }
        }
        return;
    }
    assert!(matrix[row][row] > 0);
    let mut row_target_sum: i64 = *matrix[row].last().expect("vector is empty");
    for k in (row + 1)..solution.len() {
        row_target_sum = row_target_sum - matrix[row][k] * solution[k];
    }
    if row_target_sum % matrix[row][next] != 0 {
        return
    }
    assert!(row_target_sum % matrix[row][row] == 0);
    let tentative_solution = row_target_sum / matrix[row][row];
    if tentative_solution < 0 {
        return 
    }
    solution[next] = tentative_solution;
    if next > 0 {
        solve_matrix(matrix, row - 1, next - 1, constraints, solution, minimum);
    } else {
        *minimum = min(*minimum, solution.iter().sum());
        println!("******************* minimum:{minimum}");
    }
}

fn machine_matrix(buttons: &Vec<Vec<usize>>, joltages: &Vec<i64>) -> Vec<Vec<i64>> {
    let max_rows = joltages.len();
    let max_cols = buttons.len() + 1;
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; max_cols]; max_rows];
    for row in 0..max_rows {
        matrix[row][max_cols - 1] = joltages[row];
    }
    for col in 0..max_cols - 1 {
        for button in buttons[col].clone() {
            matrix[button][col] = 1;
        }
    }
    matrix
}

fn minimum_presses(buttons: &Vec<Vec<usize>>, joltages: &Vec<i64>) -> i64 {
    let mut matrix: Vec<Vec<i64>> = machine_matrix(buttons, joltages);
    reduce(&mut matrix);
    let mut minimum: i64 = 10000000;
    let diag_end = min(matrix.len(), buttons.len());
    let initial_row = diag_end - 1;
    let mut solution: Vec<i64> = vec![0; buttons.len()];
    let constraints = set_constraints(&buttons, &joltages);
    let next = constraints.len() - 1;
    println!("next:{next}");
    print_matrix(matrix.clone());
    solve_matrix(
        &matrix,
        initial_row,
        next,
        constraints,
        &mut solution,
        &mut minimum,
    );
    minimum
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn lowest_common_multiple() {
        assert_eq!(36, lcm(12, 18));
    }
    #[test]
    fn scaling_a_vector() {
        let mut v: Vec<i64> = vec![23, 17, 4807];
        scale(&mut v, 2);
        assert_eq!(vec![46, 34, 9614], v);
    }
    #[test]
    fn reducing_a_matrix() {
        let mut m: Vec<Vec<i64>> = vec![
            vec![0, 0, 0, 1, 3],
            vec![0, 1, 1, 0, 23],
            vec![1, 0, 1, 0, 16],
            vec![1, 1, 1, 1, 30],
        ];
        reduce(&mut m);
        assert_eq!(
            vec![
                vec![1, 0, 1, 0, 16],
                vec![0, 1, 1, 0, 23],
                vec![0, 0, 1, -1, 9],
                vec![0, 0, 0, 1, 3],
            ],
            m
        );
    }
    #[test]
    fn solving_a_simple_matrix() {
        let m = vec![
            vec![1, 0, 1, 0, 16],
            vec![0, 1, 1, 0, 23],
            vec![0, 0, 1, -1, 9],
            vec![0, 0, 0, 1, 3],
        ];
        let mut minimum: i64 = 10000000;
        let initial_row = m.len() - 1;
        let mut solution: Vec<i64> = vec![0; m.len()];
        let buttons: Vec<Vec<usize>> = vec![vec![2, 3], vec![1, 3], vec![1, 2, 3], vec![0, 3]];
        let joltages: Vec<i64> = vec![3, 23, 16, 30];
        let constraints = set_constraints(&buttons, &joltages);
        let next = buttons.len() - 1;
        solve_matrix(
            &m,
            initial_row,
            next,
            constraints,
            &mut solution,
            &mut minimum,
        );
        assert_eq!(30, minimum);
    }
    #[test]
    fn set_constraints_on_buttons_groups() {
        let buttons: Vec<Vec<usize>> = vec![vec![2, 3], vec![1, 3], vec![1, 2, 3], vec![0, 3]];
        let joltages: Vec<i64> = vec![3, 23, 16, 30];
        let constraints: Vec<i64> = set_constraints(&buttons, &joltages);
        assert_eq!(vec![16, 23, 16, 3], constraints);
    }
    #[test]
    fn over_specified_matrix_doesnt_matter() {
        let m = vec![
            vec![1, 0, 1, 0, 16],
            vec![0, 1, 1, 0, 23],
            vec![0, 0, 1, -1, 9],
            vec![0, 0, 0, 1, 3],
            vec![0, 0, 0, 1, 3],
            vec![0, 0, 0, 1, 3],
        ];
        let mut minimum: i64 = 10000000;
        let diag_end = min(m.len(), m[0].len() - 1);
        let initial_row = diag_end - 1;

        let buttons: Vec<Vec<usize>> = vec![vec![2, 3], vec![1, 3], vec![1, 2, 3], vec![0, 3]];
        let joltages: Vec<i64> = vec![3, 23, 16, 30];
        let constraints = set_constraints(&buttons, &joltages);
        let mut solution: Vec<i64> = vec![0; buttons.len()];
        let next = buttons.len() - 1;
        solve_matrix(
            &m,
            initial_row,
            next,
            constraints,
            &mut solution,
            &mut minimum,
        );
        assert_eq!(30, minimum);
    }
    #[test]
    fn machine_matrix_from_input() {
        let buttons: Vec<Vec<usize>> = vec![vec![2, 3], vec![1, 3], vec![1, 2, 3], vec![0, 3]];
        let joltages: Vec<i64> = vec![3, 23, 16, 30];
        let m: Vec<Vec<i64>> = vec![
            vec![0, 0, 0, 1, 3],
            vec![0, 1, 1, 0, 23],
            vec![1, 0, 1, 0, 16],
            vec![1, 1, 1, 1, 30],
        ];
        assert_eq!(m, machine_matrix(&buttons, &joltages));
    }
    #[test]
    fn minimum_presses_nominal_case() {
        let buttons: Vec<Vec<usize>> = vec![vec![2, 3], vec![1, 3], vec![1, 2, 3], vec![0, 3]];
        let joltages: Vec<i64> = vec![3, 23, 16, 30];
        assert_eq!(30, minimum_presses(&buttons, &joltages));
    }
    #[test]
    fn minimum_presses_other_nominal_case() {
        let buttons: Vec<Vec<usize>> = vec![
            vec![0, 2, 3, 4],
            vec![2, 3],
            vec![0, 4],
            vec![0, 1, 2],
            vec![1, 2, 3, 4],
        ];
        let joltages: Vec<i64> = vec![7, 5, 12, 7, 2];
        assert_eq!(12, minimum_presses(&buttons, &joltages));
    }

    #[test]
    fn parsing_input() {
        let lines: Vec<&str> = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        ];
        let machines = parse_input(lines);
        assert_eq!(
            vec![vec![3], vec![1, 3], vec![2], vec![2, 3], vec![0, 2], vec![0, 1]],
            machines[0].buttons
        );
    }
    #[test]
    fn sample_result() {
        let path = "testdata/sample.txt";
        let content =read_to_string(path).expect("File not found.");
        let lines = content.lines().collect();
        let machines = parse_input(lines);
        let result = machines.into_iter().fold(0, |acc, machine|
            acc + minimum_presses(&machine.buttons, &machine.joltages)
        );
        assert_eq!(33, result);
    }
}
