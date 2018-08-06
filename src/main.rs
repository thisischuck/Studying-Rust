#![allow(dead_code)] // ! -> all the crate

#[allow(unused_imports)]
use std::io;

const N: usize = 3;

fn main() {
    //let mut n = [[1; N]; N];
    let _b = [[1, 2, 3], [3, 1, 2], [2, 3, 1]];

    let s: &str = "ABCDEAAAA";
    let _s2: &str = "EDCA";

    /*if latin_s(&b) {
        println!("YAS");
    } else {
        println!("BOO");
    }
    println!("{:?}", b);*/

    println!("{:?}", mais_frequente(s));
}

fn fibonanci(x: i64) -> i64 {
    if x == 1 {
        return x;
    };
    x * fibonanci(x - 1)
}

fn potencia(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    base * potencia(base, exp - 1)
}

// Gives Ownership for now
// I don't need to use it after.
// So there is no point in receiving the reference just to keep ownership
fn bits_um(mut n: u64) -> i32 {
    let mut i = 0;

    while n != 0 {
        if n % 10 == 1 {
            i += 1;
        }
        n = n / 10;
    }
    i
}

fn trailingz(mut n: u64) -> i32 {
    let mut i = 0;
    while n % 10 == 0 {
        i += 1;
        n = n / 10;
    }
    i
}

fn q_dig(mut n: u64) -> i32 {
    let mut cont = 0;

    while n != 0 {
        n = n / 10;
        cont += 1;
    }

    cont
}

fn tri_sup(m: [[i32; N]; N]) -> bool {
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if i > j {
                if m[i][j] != 0 {
                    return false;
                }
            }
        }
    }
    true
}

// Does not give Ownership anymore. Finally Learned xD
fn transposta(m: &mut [[i32; N]; N]) {
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if i == j {
                continue;
            } else if i > j {
                let x = m[i][j];
                m[i][j] = m[j][i];
                m[j][i] = x;
            }
        }
    }
}

// b does not need to be mutable cause we are only reading from it
fn add_to(a: &mut [[i32; N]; N], b: &[[i32; N]; N]) {
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            a[i][j] += b[i][j];
        }
    }
}

fn sum_diag(m: &mut [[i32; N]; N]) {
    for i in 0..m.len() {
        let mut sum = 0;
        for j in 0..m[i].len() {
            sum += m[i][j];
        }
        m[i][i] = sum;
    }
}

fn sum_diag_col(m: &mut [[i32; N]; N]) {
    for i in 0..m.len() {
        let mut sum = 0;
        for j in 0..m[i].len() {
            sum += m[j][i];
        }
        m[i][i] = sum;
    }
}

// check if it's soduko basicaly.
fn latin_s(m: &[[i32; N]; N]) -> bool {
    // 1 2 3                  1 3 2
    // 3 1 2 -> transposta -> 2 1 3
    // 2 3 1                  3 2 1
    let mut check_values: [i32; N] = [0; N];

    for i in 0..N {
        check_values[i] = i as i32;
    }

    println!("{:?}", check_values);

    // Transpose the matrix so the collums become lines. And run the checks in parallel
    let mut m2 = m.clone();
    transposta(&mut m2);

    for check_iter in 0..N {
        for i in 0..N {
            if exists_in_line(&m[i], check_values[check_iter])
                && exists_in_line(&m2[i], check_values[check_iter])
            {
                return true;
            }
        }
    }
    false
}

// Checks if it exists line by line
fn exists_in_line(l: &[i32; N], n: i32) -> bool {
    for i in 0..N {
        if l[i] == n {
            return true;
        }
    }
    false
}

fn contida(s_input: &str, s: &str) -> bool {
    for i in s_input.chars() {
        if char_exists_in_string(i, s) == false {
            return false;
        }
    }
    true
}

fn char_exists_in_string(c: char, s: &str) -> bool {
    for s_char in s.chars() {
        if s_char == c {
            return true;
        }
    }
    false
}

fn mais_frequente(s: &str) -> (i32, char) {
    let mut max_num_pos = (-1, '0');

    for i in s.chars() {
        if i == max_num_pos.1 {
            continue;
        } else {
            let j = quantas_vezes_existe(i, s);
            if max_num_pos.0 < j {
                max_num_pos = (j, i);
            }
        }
    }
    max_num_pos
}

fn quantas_vezes_existe(c: char, s: &str) -> i32 {
    let mut count = 0;
    for i in s.chars() {
        if i == c {
            count += 1;
        }
    }
    count
}

fn iguais_consecutivos(c: &str) -> i32 {
    1
}
