/* global BigInt */

const ROBUST_TEST_SET = [
    [ 2047n, [ 2n ] ],
    [ 1373653n, [ 2n, 3n ] ],
    [ 9080191n, [ 31n, 73n ] ],
    [ 25326001n, [ 2n, 3n, 5n ] ],
    [ 3215031751n, [ 2n, 3n, 5n, 7n ] ],
    [ 4759123141n, [ 2n, 7n, 61n ] ],
    [ 1122004669633n, [ 2n, 13n, 23n, 1662803n ] ],
    [ 2152302898747n, [ 2n, 3n, 5n, 7n, 11n ] ],
    [ 3474749660383n, [ 2n, 3n, 5n, 7n, 11n, 13n ] ],
    [ 341550071728321n, [ 2n, 3n, 5n, 7n, 11n, 13n, 17n ] ],
    [ 3825123056546413051n, [ 2n, 3n, 5n, 7n, 11n, 13n, 17n, 19n, 23n ] ],
    [ 18446744073709551616n, [ 2n, 3n, 5n, 7n, 11n, 13n, 17n, 19n, 23n, 29n, 31n, 37n ] ]
];

function getBitWidth(n) {
    let e = 1n;
    while ((1n<<e) <= n) {
        ++e;
    }
    return e-1n;
}

class RNG {
    constructor() {
        this.seed = 1n;
    }

    next31Bit() {
        this.seed = (1103515245n * this.seed + 12345n) % 0x7fffffffn;
        return this.seed;
    }

    nextRange(bound) {
        const width = getBitWidth(bound);
        let result = 0n;
        for (let pos = 0n; pos < width; pos += 31n) {
            result = (result << 31n) | this.next31Bit();
        }
        return result % bound;
    }
}

function getMin(a, b) {
    return a > b ? b : a;
}

// Heron's Method
function getIntSqrt(n) {
    if (n <= 1n) {
        return n;
    }

    let x0 = n;
    let x1 = getMin(1n<<(getBitWidth(n)/2n+1n), n/2n);
    while (x1 < x0) {
        x0 = x1;
        x1 = (x0 + n/x0) / 2n;
    }
    return x0;
}

function isPerfectSquare(n) {
    const sqrt = getIntSqrt(n);
    return sqrt * sqrt === n;
}

function getGCD(a, b) {
    return b === 0n ? a : getGCD(b, a % b);
}

function getAbs(a) {
    return a > 0 ? a : -a;
}

function getModPow(val, exp, mod) {
    let result = 1n, subPower = val % mod;
    while (exp > 0n) {
        if ((exp & 1n) === 1n) {
            result = (result * subPower) % mod;
        }
        exp >>= 1n;
        subPower = (subPower * subPower) % mod;
    }
    return result;
}

function doMillerRabinTest(n, a) {
    let d = n - 1n;
    while (d % 2n === 0n) {
        d /= 2n;
    }

    let x = getModPow(a, d, n);
    if (x === 1n || x === n-1n) {
        return true;
    }
    while (d !== n-1n) {
        x = (x*x) % n;
        d *= 2n;
        if (x === n-1n) {
            return true;
        }
    }
    return false;
}

function isPrime(n) {
    if (n === 2n) {
        return true;
    } else if (n <= 1n || n % 2n === 0n) {
        return false;
    }

    let robustTests = ROBUST_TEST_SET.find(([bound,]) => n < bound)?.[1];
    if (robustTests === undefined) {
        const rng = new RNG();
        robustTests = Array.from({length: 15}, () => rng.next31Bit());
    }
    return robustTests.every(a => {
        return doMillerRabinTest(n, a)
    });
}

function factorizeRecursively(n, result) {
    if (n <= 1n) {
        return;
    } else if (isPrime(n)) {
        result.push(n);
        return;
    } else if (n % 2n === 0n) {
        result.push(2n);
        factorizeRecursively(n/2n, result);
        return;
    }

    const rng = new RNG();
    let xs, xt, c, factor = n;
    const f = (x) => (x*x % n + c) % n;

    do {
        if (factor === n) {
            xs = xt = rng.nextRange(n-2n) + 2n;
            c = rng.nextRange(20n) + 1n;
        }
        xs = f(xs);
        xt = f(f(xt));
        factor = getGCD(getAbs(xs-xt), n);
    } while (factor === 1n || factor === n);

    factorizeRecursively(factor, result);
    factorizeRecursively(n/factor, result);
}

function factorize(n) {
    const result = [];
    factorizeRecursively(n, result);
    return result.sort((a, b) => (a < b) ? -1 : ((a > b) ? 1 : 0));
}

// Brahmagupta–Fibonacci identity
function mergeAsTwo(a, b) {
    return [
        getAbs(a[0]*b[0] - a[1]*b[1]),
        a[0]*b[1] + a[1]*b[0],
        0n, 0n
    ];
}

function getRemainders(a, b, bound) {
    const boundSqrt = getIntSqrt(bound);
    const result = [];
    while (b !== 0n) {
        const r = a % b;
        if (r <= boundSqrt) {
            result.push(r);
        }
        a = b;
        b = r;
    }
    return result;
}

// p is one or a prime number (1 or 2 or 4k+1)
function findSumOfTwo(p) {
    if (p === 1n) {
        return [ 1n, 0n, 0n, 0n ];
    }
    if (p === 2n) {
        return [ 1n, 1n, 0n, 0n ];
    }

    let rems = [];
    const rng = new RNG();
    do {
        let q = rng.nextRange(p);
        while (getModPow(q, (p-1n)/2n, p) !== p-1n) {
            q = rng.nextRange(p);
        }
        const x = getModPow(q, (p-1n)/4n, p);
        rems = getRemainders(p, x, p);
    } while (rems.length <= 2);
    return [ rems[0], rems[1], 0n, 0n ];
}

const RABIN_EXCEPTIONS = {
    "5": [ 2n, 1n, 0n, 0n ],
    "10": [ 3n, 1n, 0n, 0n ],
    "13": [ 3n, 2n, 0n, 0n ],
    "34": [ 5n, 3n, 0n, 0n ],
    "37": [ 6n, 1n, 0n, 0n ],
    "58": [ 7n, 3n, 0n, 0n ],
    "61": [ 6n, 5n, 0n, 0n ],
    "85": [ 9n, 2n, 0n, 0n ],
    "130": [ 11n, 3n, 0n, 0n ],
    "214": [ 14n, 3n, 3n, 0n ],
    "226": [ 15n, 1n, 0n, 0n ],
    "370": [ 19n, 3n, 0n, 0n ],
    "526": [ 21n, 9n, 2n, 0n ],
    "706": [ 25n, 9n, 0n, 0n ],
    "730": [ 27n, 1n, 0n, 0n ],
    "829": [ 27n, 10n, 0n, 0n ],
    "1414": [ 33n, 18n, 1n, 0n ],
    "1549": [ 35n, 18n, 0n, 0n ],
    "1906": [ 41n, 15n, 0n, 0n ],
    "2986": [ 45n, 31n, 0n, 0n ],
    "7549": [ 85n, 18n, 0n, 0n ],
    "9634": [ 97n, 15n, 0n, 0n ],
};

// MICHAEL 0. RABIN et al. "Randomized Algorithms in Number Theory"
function findSumOfThree(n) {
    if (RABIN_EXCEPTIONS[n]) {
        return RABIN_EXCEPTIONS[n];
    } else if (n % 4n === 0n) {
        const sub = findSumOfThree(n/4n);
        return [ sub[0]*2n, sub[1]*2n, sub[2]*2n, sub[3]*2n ];
    } else if (n % 8n === 7n) {
        return [ 0n, 0n, 0n, 0n ]; // Exception - Impossible
    } else if (n % 8n === 3n) {
        let x, p;
        const rng = new RNG();
        do {
            x = rng.nextRange(getIntSqrt(n)+1n);
            p = (n - x*x) / 2n;
        } while ((n-x*x) % 2n !== 0n || !(isPrime(p) || p === 1n));
        const two = findSumOfTwo(p);
        return [ x, two[0]+two[1], getAbs(two[0]-two[1]), 0n ];
    } else if (isPerfectSquare(n)) {
        return [ getIntSqrt(n), 0n, 0n, 0n ];
    } else {
        let x, p;
        const rng = new RNG();
        do {
            x = rng.nextRange(getIntSqrt(n)+1n);
            p = (n - x*x);
        } while (!isPrime(p));
        const two = findSumOfTwo(p);
        return [ x, two[0], two[1], 0n ];
    }
}

export function findSolution(n, findOptimal = false) {
    if (typeof n !== 'bigint') {
        n = BigInt(n);
    }
    if (n === 1n) {
        return [ 1n, 0n, 0n, 0n ];
    } else if (n === 2n) {
        return [ 1n, 1n, 0n, 0n ];
    } else if (n === 3n) {
        return [ 1n, 1n, 1n, 0n ];
    }

    // n이 4의 배수라면 이를 제거해줌.
    // 아래에서 n = 4^a mod 8 = 7 인지 체크하여 4개의 수가 필요한지를 체크하기 때문.
    if (n % 4n === 0n) {
        const sub = findSolution(n/4n);
        return [ sub[0]*2n, sub[1]*2n, sub[2]*2n, sub[3]*2n ];
    }
    
    // 완전 제곱수라면 1개로 표현. (필요충분)
    if (isPerfectSquare(n)) {
        return [ getIntSqrt(n), 0n, 0n, 0n ];
    }

    // n mod 8 = 7 이라면 무조건 4개의 수가 필요. (필요충분)
    // n = 4 + (n-4)로 분할하면 (n-4) mod 8 = 3 이므로 3개 내로 표현 가능할 테고,
    // 그럼 { 2, solution of (n-4) }가 해가 될 것.
    if (n % 8n === 7n) {
        const sub = findSumOfThree(n-4n);
        return [ 2n, sub[0], sub[1], sub[2] ];
    }

    // 위의 조건들에 모두 해당하지 않는다면, n은 2개 또는 3개의 제곱수로 나타낼 수 있음.
    // 따라서 MICHAEL 0. RABIN et al.의 알고리즘을 이용하여 3개의 제곱수를 사용하는 해를 찾아도 되지만,
    // 가능한 적은 제곱수를 사용하는 "최적해"를 구하려고 한다면
    // Pollard-rho 소인수 분해 알고리즘으로 n을 소인수분해해 4k+3 꼴의 소인수가 존재하는지 확인해야 함. (O(n^(1/4)))
    if (findOptimal) {
        const primes = factorize(n);
        let evenAcc = 1n;
        // oddCount에 들어가는 것은 지수가 홀수인 소인수.
        const oddSet = new Set();
        primes.forEach(p => {
            if (oddSet.has(p)) {
                evenAcc *= p;
                oddSet.delete(p);
            } else {
                oddSet.add(p);
            }
        });

        // n mod 8 != 7 이라면 2개 or 3개로 만들 수 있음.
        // oddCount에 포함된 p에는 2 or 4k+1 꼴 or 4k+3 꼴의 소수가 있을 수 있는데
        // 2와 4k+1은 2개의 제곱수로 나타낼 수 있으나,
        // 4k+3는 3개의 제곱수 필요.
        let mod3Exist = false;
        oddSet.forEach(p => {
            if (p % 4n === 3n) {
                mod3Exist = true;
            }
        });

        if (mod3Exist) {
            return findSumOfThree(n);
        }

        let result = null;
        oddSet.forEach(p => {
            if (result === null) {
                result = findSumOfTwo(p);
            } else {
                result = mergeAsTwo(result, findSumOfTwo(p));
            }
        });
        return [ result[0]*evenAcc, result[1]*evenAcc, result[2]*evenAcc, result[3]*evenAcc ];
    } else {
        return findSumOfThree(n);
    }
}