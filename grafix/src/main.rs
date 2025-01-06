use libm::{cos, sin};

fn sng(x:f64) -> i64{
    if x>0.0{
        return 1;
    }
    0 
}

fn generate<F>(min: i64, max: i64, check: F) -> Vec<Vec<i64>>
where
    F: Fn(i64, i64) -> bool,
{
    let size = (max - min) as usize;
    let mut some_vec: Vec<Vec<i64>> = vec![vec![0; size + 1]; size + 1];
    let mut znach = min;
    for i in 0..=size {
        if i == 0 {
            continue;
        }
        some_vec[0][i] = znach as i64;
        znach += 1;
    }
    let mut znach = max - 1;
    for i in 0..=size {
        if i == 0 {
            continue;
        }
        some_vec[i][0] = znach as i64;
        znach -= 1;
    }

    for x in 1..=size {
        for y in 1..=size {
            if check(some_vec[0][y], some_vec[x][0]) {
                some_vec[x][y] = 1;
            }
        }
    }

    some_vec
}

fn trace(some_vec: Vec<Vec<i64>>, min: i64, max: i64) {
    let size = (max - min) as usize;
    print!("  ");
    for y in 1..=size {
        print!("{:>3} ", some_vec[0][y]);
    }
    println!();

    for x in 1..=size {
        print!("{:>3} ", some_vec[x][0]);

        for y in 1..=size {
            if some_vec[x][y] == 0 {
                print!("    ");
            } else if some_vec[x][y] == 1 {
                print!("*   ");
            }
        }
        println!();
    }
}

fn main() {
    let min = -10;
    let max = 10;
    //Sgn(sin(14,88x) ** cos(14,88x+π/2))
    let check_equal = |x: i64, y: i64| {
        sng(sin(x as f64 * 14.88).powf(cos(14.88 + std::f64::consts::PI))) == y
    };
    let result = generate(min, max, check_equal);
    trace(result, min, max);
}
