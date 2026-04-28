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

fn reduce_row(target_row: &mut [i64], source_row: &mut [i64], column: usize) {
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

fn reduce(matrix: &mut Vec<Vec<i64>>) {
    fn swap(v: &mut [i64], w: &mut [i64]) {
        for i in 0..v.len() {
            let tmp: i64 = v[i];
            v[i] = w[i];
            w[i] = tmp;
        }
    }
    for d in 0..matrix.len() {
        for r in d..matrix.len() {
            if matrix[r][d] != 0 {
                let (a, b) = matrix.split_at_mut(r);
                swap(&mut a[r - 1], &mut b[0]);
                break;
            };
        }
        if matrix[d][d] < 0 {
            scale(&mut matrix[d], -1)
        };
        for r in d + 1..matrix.len() {
            let (a, b) = matrix.split_at_mut(r);
            reduce_row(&mut a[r - 1], &mut b[0], d)
        }
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
}
