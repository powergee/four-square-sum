#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use std::collections::{HashSet};
use num_bigint::{BigInt, ToBigInt, RandBigInt};
use num_integer::Integer;
use num_traits::{Zero, One};
use num_traits::sign::Signed;
use num_traits::cast::ToPrimitive;

type Quadruple = [BigInt; 4];

lazy_static! {
    static ref ZERO: BigInt = Zero::zero();
    static ref ONE: BigInt = BigInt::from(1);
    static ref TWO: BigInt = BigInt::from(2);
    static ref THREE: BigInt = BigInt::from(3);
    static ref FOUR: BigInt = BigInt::from(4);
    static ref EIGHT: BigInt = BigInt::from(8);
    static ref BASE_L: BigInt = BigInt::from(2);
    static ref BASE_R: BigInt = BigInt::from(std::i64::MAX);
    static ref POLLARD_CONST: BigInt = BigInt::from(20);
}

fn is_perfect_square(n: &BigInt) -> bool {
    let sqrt = n.sqrt();
    &sqrt * &sqrt == *n
}

fn do_miller_rabin_test(n: &BigInt, a: &BigInt) -> bool {
    let init_d = n-&*ONE;
    let mut d: BigInt = init_d.clone();
    while d.is_even() {
        d = d.div_floor(&*TWO);
    }

    let mut x = a.modpow(&d, &n);

    if x.is_one() || x.eq(&init_d) {
        return true;
    }
    while !d.eq(&init_d) {
        x = x.modpow(&*TWO, &n);
        d *= &*TWO;
        if x == init_d {
            return true;
        }
    }
    false
}

const ROBUST_TEST_SET: &'static [(i128, &'static [i128])] = &[
    (2047, &[ 2 ]),
    (1373653, &[ 2, 3 ]),
    (9080191, &[ 31, 73 ]),
    (25326001, &[ 2, 3, 5 ]),
    (3215031751, &[ 2, 3, 5, 7 ]),
    (4759123141, &[ 2, 7, 61 ]),
    (1122004669633, &[ 2, 13, 23, 1662803 ]),
    (2152302898747, &[ 2, 3, 5, 7, 11 ]),
    (3474749660383, &[ 2, 3, 5, 7, 11, 13 ]),
    (341550071728321, &[ 2, 3, 5, 7, 11, 13, 17 ]),
    (3825123056546413051, &[ 2, 3, 5, 7, 11, 13, 17, 19, 23 ]),
    (18446744073709551616, &[ 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37 ]),
];

fn is_prime(n: &BigInt) -> bool {
    if *n == *TWO {
        return true;
    } else if n <= &ONE || n.is_even() {
        return false;
    }
    
    let found = ROBUST_TEST_SET.iter().position(|pair| n < &BigInt::from(pair.0));
    match found {
        Some(i) => !(ROBUST_TEST_SET[i].1).into_iter().any(|a| !do_miller_rabin_test(n, &(&*ONE*a))),
        None => {
            let mut rng = rand::thread_rng();
            for _i in 0..15 {
                if !do_miller_rabin_test(n, &rng.gen_bigint_range(&BASE_L, &BASE_R)) {
                    return false;
                }
            }
            return true;
        }
    }
}

fn factorize_rec(n: &BigInt, result: &mut Vec<BigInt>) {
    if n.le(&*ONE) {
        return;
    } else if n.is_multiple_of(&*TWO) {
        result.push((*TWO).clone());
        factorize_rec(&n.div_floor(&*TWO), result);
        return;
    } else if is_prime(n) {
        result.push(n.clone());
        return;
    }

    let mut rng = rand::thread_rng();
    let mut xs: BigInt = (*ZERO).clone();
    let mut xt: BigInt = (*ZERO).clone();
    let mut c: BigInt = (*ZERO).clone();
    let mut factor = n.clone();
    let f = |x: &BigInt, n: &BigInt, c: &BigInt| ((x.modpow(&*TWO, n) + c) % n);

    loop {
        if factor.eq(n) {
            xs = rng.gen_bigint_range(&*TWO, &(n+(&*ONE)));
            xt = xs.clone();
            c = rng.gen_bigint_range(&*ONE, &*POLLARD_CONST);
        }
        xs = f(&xs, n, &c);
        xt = f(&xt, n, &c);
        xt = f(&xt, n, &c);
        factor = (&xs-&xt).gcd(n);
        if factor != *ONE && factor != *n { break; }
    };

    factorize_rec(&factor, result);
    factorize_rec(&n.div_floor(&factor), result);
}

fn factorize(n: &BigInt) -> Vec<BigInt> {
    let mut result: Vec<BigInt> = vec![];
    factorize_rec(n, &mut result);
    result.sort_unstable();
    result
}

// Brahmagupta–Fibonacci identity
fn merge_as_two(a: &Quadruple, b: &Quadruple) -> Quadruple {
    [
        (&a[0]*&b[0] - &a[1]*&b[1]),
        (&a[0]*&b[1] + &a[1]*&b[0]),
        (*ZERO).clone(),
        (*ZERO).clone()
    ]
}

fn get_remainders(a: &mut BigInt, b: &mut BigInt, bound: &BigInt) -> Vec<BigInt> {
    let bound_sqrt = bound.sqrt();
    let mut result: Vec<BigInt> = vec![];
    while !b.is_zero() && result.len() < 2 {
        let r = a.div_rem(b).1;
        if r.gt(&*ZERO) && r.le(&bound_sqrt) {
            result.push(r.clone());
        }
        *a = (*b).clone();
        *b = r;
    }
    result
}

// p is one or a prime number (1 or 2 or 4k+1)
fn find_sum_of_two(p: &BigInt) -> Quadruple {
    if p.is_one() {
        return [
            (*ONE).clone(),
            (*ZERO).clone(),
            (*ZERO).clone(),
            (*ZERO).clone()
        ];
    } else if p == &*TWO {
        return [
            (*ONE).clone(),
            (*ONE).clone(),
            (*ZERO).clone(),
            (*ZERO).clone()
        ];
    }

    let mut rems;
    let mut rng = rand::thread_rng();
    let p_dec = p-(&*ONE);
    let p_dec_half = &p_dec/(&*TWO);
    let p_dec_quarter = &p_dec_half/(&*TWO);

    loop {
        let mut q = rng.gen_bigint_range(&*ZERO, &p);
        while q.modpow(&p_dec_half, p) != p_dec {
            q = rng.gen_bigint_range(&*ZERO, &p);
        }
        let mut x = q.modpow(&p_dec_quarter, &p);
        rems = get_remainders(&mut (p.clone()), &mut x, &p);

        if rems.len() >= 2 {
            break [
                rems[0].clone(),
                rems[1].clone(),
                (*ZERO).clone(),
                (*ZERO).clone()
            ];
        }
    }
}

// MICHAEL 0. RABIN et al. "Randomized Algorithms in Number Theory"
fn find_sum_of_three(n: &BigInt) -> Quadruple {
    if let Some(exp) = check_rabin_exceptions(n) {
        return exp;
    } else if n.is_multiple_of(&*FOUR) {
        let sub = find_sum_of_three(&(n/(&*FOUR)));
        return [ &sub[0]*(&*TWO), &sub[1]*(&*TWO), &sub[2]*(&*TWO), &sub[3]*(&*TWO) ];
    } else if is_perfect_square(n) {
        return [ n.sqrt(), (*ZERO).clone(), (*ZERO).clone(), (*ZERO).clone() ];
    }

    match n.div_rem(&*EIGHT).1.to_i64().unwrap() {
        7 => {
            panic!("find_sum_of_three failed!")
        },
        3 => {
            let mut rng = rand::thread_rng();
            let mut x;
            let mut p;
            loop {
                x = rng.gen_bigint_range(&*ZERO, &(n.sqrt()+(&*ONE)));
                p = (n - &x*&x) / (&*TWO);

                if (n-&x*&x).is_multiple_of(&*TWO) && (is_prime(&p) || p.is_one()) { break; }
            }
            let two = find_sum_of_two(&p);
            [ x, &two[0]+&two[1], (&two[0]-&two[1]).abs(), (*ZERO).clone() ]
        },
        _ => {
            let mut rng = rand::thread_rng();
            let mut x;
            let mut p;
            loop {
                x = rng.gen_bigint_range(&*ZERO, &(n.sqrt()+(&*ONE)));
                p = n - &x*&x;
                if is_prime(&p) { break; }
            }
            let two = find_sum_of_two(&p);
            [ x, two[0].clone(), two[1].clone(), (*ZERO).clone() ]
        }
    }
}


fn check_rabin_exceptions(n: &BigInt) -> Option<Quadruple> {
    if n > &9634_i32.to_bigint().unwrap() {
        None
    } else {
        let result = match n.to_i64().unwrap() {
            5 => [ 2, 1, 0, 0 ],
            10 => [ 3, 1, 0, 0 ],
            13 => [ 3, 2, 0, 0 ],
            34 => [ 5, 3, 0, 0 ],
            37 => [ 6, 1, 0, 0 ],
            58 => [ 7, 3, 0, 0 ],
            61 => [ 6, 5, 0, 0 ],
            85 => [ 9, 2, 0, 0 ],
            130 => [ 11, 3, 0, 0 ],
            214 => [ 14, 3, 3, 0 ],
            226 => [ 15, 1, 0, 0 ],
            370 => [ 19, 3, 0, 0 ],
            526 => [ 21, 9, 2, 0 ],
            706 => [ 25, 9, 0, 0 ],
            730 => [ 27, 1, 0, 0 ],
            829 => [ 27, 10, 0, 0 ],
            1414 => [ 33, 18, 1, 0 ],
            1549 => [ 35, 18, 0, 0 ],
            1906 => [ 41, 15, 0, 0 ],
            2986 => [ 45, 31, 0, 0 ],
            7549 => [ 85, 18, 0, 0 ],
            9634 => [ 97, 15, 0, 0 ],
            _ => [ -1, -1, -1, -1 ],
        };
        if result[0] > 0 {
            Some([
                BigInt::from(result[0]),
                BigInt::from(result[1]),
                BigInt::from(result[2]),
                BigInt::from(result[3])
            ])
        } else {
            None
        }
    }
}

fn find_solution(n: &BigInt, find_optimal: bool) -> Quadruple {
    if n.is_one() {
        return [
            (*ONE).clone(),
            (*ZERO).clone(),
            (*ZERO).clone(),
            (*ZERO).clone()
        ];
    } else if n == &*TWO {
        return [
            (*ONE).clone(),
            (*ONE).clone(),
            (*ZERO).clone(),
            (*ZERO).clone()
        ];
    } else if n == &*THREE {
        return [
            (*ONE).clone(),
            (*ONE).clone(),
            (*ONE).clone(),
            (*ZERO).clone()
        ];
    }

    // n이 4의 배수라면 이를 제거해줌.
    // 아래에서 n = 4^a mod 8 = 7 인지 체크하여 4개의 수가 필요한지를 체크하기 때문.
    if n.is_multiple_of(&*FOUR) {
        let sub = find_solution(&n.div_floor(&*FOUR), find_optimal);
        return [ &sub[0]*(&*TWO), &sub[1]*(&*TWO), &sub[2]*(&*TWO), &sub[3]*(&*TWO) ];
    }

    // 완전 제곱수라면 1개로 표현. (필요충분)
    if is_perfect_square(n) {
        return [ n.sqrt(), (*ZERO).clone(), (*ZERO).clone(), (*ZERO).clone() ];
    }

    // n mod 8 = 7 이라면 무조건 4개의 수가 필요. (필요충분)
    // n = 4 + (n-4)로 분할하면 (n-4) mod 8 = 3 이므로 3개 내로 표현 가능할 테고,
    // 그럼 { 2, solution of (n-4) }가 해가 될 것.
    if n.div_rem(&*EIGHT).1.to_i64().unwrap() == 7 {
        let sub = find_sum_of_three(&(n-(&*FOUR)));
        return [ (*TWO).clone(), sub[0].clone(), sub[1].clone(), sub[2].clone() ];
    }

    // 위의 조건들에 모두 해당하지 않는다면, n은 2개 또는 3개의 제곱수로 나타낼 수 있음.
    // 따라서 MICHAEL 0. RABIN et al.의 알고리즘을 이용하여 3개의 제곱수를 사용하는 해를 찾아도 되지만,
    // 가능한 적은 제곱수를 사용하는 "최적해"를 구하려고 한다면
    // Pollard-rho 소인수 분해 알고리즘으로 n을 소인수분해해 4k+3 꼴의 소인수가 존재하는지 확인해야 함. (O(n^(1/4)))
    if find_optimal {
        let primes = factorize(n);
        let mut even_acc: BigInt = (*ONE).clone();
        let mut f_with_odd_exp = HashSet::new();
        for p in primes {
            if f_with_odd_exp.contains(&p) {
                even_acc *= &p;
                f_with_odd_exp.remove(&p);
            } else {
                f_with_odd_exp.insert(p);
            }
        }

        // n mod 8 != 7 이라면 2개 or 3개로 만들 수 있음.
        // oddCount에 포함된 p에는 2 or 4k+1 꼴 or 4k+3 꼴의 소수가 있을 수 있는데
        // 2와 4k+1은 2개의 제곱수로 나타낼 수 있으나,
        // 4k+3는 3개의 제곱수 필요.
        if f_with_odd_exp.iter().any(|p| p.div_rem(&*FOUR).1.to_i64().unwrap() == 3) {
            return find_sum_of_three(n);
        }

        let f_with_odd_exp: Vec<BigInt> = f_with_odd_exp.into_iter().collect();
        let mut result = find_sum_of_two(&f_with_odd_exp[0]);
        for i in 1..(f_with_odd_exp.len()) {
            result = merge_as_two(&result, &find_sum_of_two(&f_with_odd_exp[i]));
        }
        return [
            &result[0]*&even_acc,
            &result[1]*&even_acc,
            &result[2]*&even_acc,
            &result[3]*&even_acc
        ];
    } else {
        return find_sum_of_three(n);
    }
}

#[wasm_bindgen(js_name = findSolution)]
pub fn find_solution_str(n: String, find_optimal: bool) -> String {
    let n_int = n.parse::<BigInt>().unwrap();
    let result = find_solution(&n_int, find_optimal);
    return [
        result[0].to_str_radix(10_u32),
        result[1].to_str_radix(10_u32),
        result[2].to_str_radix(10_u32),
        result[3].to_str_radix(10_u32)
    ].join(" ");
}

#[cfg(test)]
mod test;