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
    println!(
        "reduce row {:?} with source row {:?}",
        target_row, source_row
    );
    if target_row[column] == 0 {
        return;
    };
    assert!(source_row[column] > 0);
    if target_row[column] < 0 {
        scale(target_row, -1)
    };
    let scale_to: i64 = lcm(target_row[column], source_row[column]);
    scale(target_row, scale_to / target_row[column]);
    scale(source_row, scale_to / source_row[column]);
    for i in 0..target_row.len() {
        target_row[i] -= source_row[i]
    }
}

fn swap(v: &mut [i64], w: &mut [i64]) {
    println!("swap rows {:?} and {:?}", v, w);
    for i in 0..v.len() {
        let tmp: i64 = v[i];
        v[i] = w[i];
        w[i] = tmp;
    }
}

fn reduce(matrix: &mut Vec<Vec<i64>>) {
    println!("initial:");
    print_matrix(matrix.to_vec());
    let mut pivot_row: usize = 0;
    for d in 0..matrix.len() {
        println!("column {d}:");
        for r in d..matrix.len() {
            println!("row {r}:");
            if matrix[r][d] != 0 {
                if r != d {
                    let (a, b) = matrix.split_at_mut(r);
                    println!("a:{:?} b:{:?}", a, b);
                    swap(&mut a[d], &mut b[0]);
                }
                pivot_row = r;
                break;
            };
        }
        assert!(matrix[d][d] != 0);
        if matrix[d][d] < 0 {
            scale(&mut matrix[d], -1)
        };
        println!("d={d}, row reductions for");
        print_matrix(matrix.to_vec());
        for r in d + 1..matrix.len() {
            let (a, b) = matrix.split_at_mut(r);
            reduce_row(&mut b[0], &mut a[d], d);
            print_matrix(matrix.to_vec());
        }
        println!("** column {d} done **");
        print_matrix(matrix.to_vec());
    }
}

fn solve_matrix(
    matrix: &Vec<Vec<i64>>,
    row: usize,
    solution: &mut Vec<i64>,
    minimum: &mut i64,
) {
    println!("{:?}", solution);
    assert!(matrix[row][row] > 0);
    let mut row_target_sum: i64 = *matrix[row].last().expect("vector is empty");
    for k in (row + 1)..solution.len() {
        row_target_sum = row_target_sum - matrix[row][k] * solution[k];
    }
    assert!(row_target_sum % matrix[row][row] == 0);
    solution[row] = row_target_sum / matrix[row][row];
    if row > 0 {
        solve_matrix(matrix, row - 1, solution, minimum);
    } else {
        *minimum = min(*minimum, solution.iter().sum());
        return;
    }
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
        let mut minimum:i64 = 10000000;
        let initial_row = m.len() - 1;
        let mut solution: Vec<i64> = vec![0; m.len()];
        solve_matrix(&m, initial_row, &mut solution, &mut minimum);
        assert_eq!(30, minimum);

    }
}
