pub fn gen_primes(up_to: usize) -> Vec<usize> {
    let mut erasten = vec![true; up_to + 1];
    let mut primes = Vec::new();
    for i in 2..up_to + 1 {
        if erasten[i] {
            primes.push(i);
            let mut j = i;
            while j <= up_to {
                erasten[j] = false;
                j += i;
            }
        }
    }
    primes
}

pub fn divisors(n: usize, primes: &Vec<usize>) -> Vec<usize> {
    let mut divisors = vec![1];
    let mut n = n;
    for &prime in primes {
        let mut e = 1;
        let l = divisors.len();
        while n % prime == 0 {
            n /= prime;
            e *= prime;
            for i in 0..l {
                divisors.push(divisors[i] * e);
            }
        }
        if n == 1 {
            break;
        }
    }
    if n > 1 {
        for i in 0..divisors.len() {
            divisors.push(divisors[i] * n);
        }
    }
    divisors
}

// def divisors_of_number(a: int, primes: list[int] = []) -> list[int]:
//     if not primes:
//         primes = erasten(int(a**0.5))
//     dividor = [1]
//     for p in primes:
//         e = 0
//         while a % p == 0:
//             e += 1
//             a //= p
//         dividor += [x * p**ex for x in dividor for ex in range(1, e + 1)]
//     return dividor
