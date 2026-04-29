use std::cmp::min;

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
        assert!(matrix[diag_index][diag_index] != 0);
        if matrix[diag_index][diag_index] < 0 {
            scale(&mut matrix[diag_index], -1)
        };
        print_matrix(matrix.to_vec());
        println!("***");
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
    assert!(matrix[row][row] > 0);
    if next > row {
        for guess in 0..constraints[next] + 1 {
            solution[next] = guess;
            solve_matrix(matrix, row, next - 1, constraints, solution, minimum);
            return;
        }
    }
    assert!(matrix[row][next] > 0);
    let mut row_target_sum: i64 = *matrix[row].last().expect("vector is empty");
    for k in (row + 1)..solution.len() {
        row_target_sum = row_target_sum - matrix[row][k] * solution[k];
    }
    assert!(row_target_sum % matrix[row][row] == 0);
    solution[row] = row_target_sum / matrix[row][row];
    if row > 0 {
        solve_matrix(matrix, row - 1, next, constraints, solution, minimum);
    } else {
        *minimum = min(*minimum, solution.iter().sum());
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
    let mut solution: Vec<i64> = vec![0; diag_end];
    let constraints = set_constraints(&buttons, &joltages);
    solve_matrix(
        &matrix,
        initial_row,
        0,
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
        solve_matrix(&m, initial_row, 0, constraints, &mut solution, &mut minimum);
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

        let mut solution: Vec<i64> = vec![0; diag_end];
        let buttons: Vec<Vec<usize>> = vec![vec![2, 3], vec![1, 3], vec![1, 2, 3], vec![0, 3]];
        let joltages: Vec<i64> = vec![3, 23, 16, 30];
        let constraints = set_constraints(&buttons, &joltages);
        solve_matrix(&m, initial_row, 0, constraints, &mut solution, &mut minimum);
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
        assert_eq!(30, minimum_presses(&buttons, &joltages));
    }
}
