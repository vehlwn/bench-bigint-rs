fn ceil_sqrt(n: &num_bigint::BigInt) -> num_bigint::BigInt {
    let tmp = n.sqrt();
    if tmp.pow(2) == *n {
        return tmp;
    } else {
        return tmp + 1;
    }
}

fn is_exact_square(n: &num_bigint::BigInt) -> bool {
    const RESIDUES_MOD_100: [i32; 22] = [
        0, 1, 4, 9, 16, 21, 24, 25, 29, 36, 41, 44, 49, 56, 61, 64, 69, 76, 81, 84, 89, 96,
    ];
    use num::ToPrimitive;
    let rem: num_bigint::BigInt = n % 100;
    if !RESIDUES_MOD_100.contains(&rem.to_i32().unwrap()) {
        return false;
    }
    let tmp = n.sqrt();
    return tmp.pow(2) == *n;
}

fn get_fermat_factor(n: &num_bigint::BigInt) -> (num_bigint::BigInt, num_bigint::BigInt) {
    let mut x = ceil_sqrt(n);
    let mut y2 = x.pow(2) - n;
    while !is_exact_square(&y2) {
        x += 1;
        y2 = x.pow(2) - n;
    }
    let y = y2.sqrt();
    return (&x + &y, &x - &y);
}

fn main() {
    let n: num_bigint::BigInt = ((1_u64 << 53) - 1).into();
    println!("n = {}", n);
    let (a, b) = get_fermat_factor(&n);
    println!("a = {}", a);
    println!("b = {}", b);
}
