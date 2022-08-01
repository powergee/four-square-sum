# Lagrange's four-square calculator

Every natural number can be represented as follows:

$$
N = a^2+b^2+c^2+d^2\\
$$

$$
\text{($a, b, c, d$ are inegers.)}
$$

**Lagrange's four-square theorem**, also known as **Bachet's conjecture**, states that *every natural number can be represented as the sum of four integer squares.* ([ref](https://en.wikipedia.org/wiki/Lagrange%27s_four-square_theorem))

This algorithm can calculate one of the below:
* **an arbitrary solution** of any non-negative integer $N$ by **randomized polynomial time.**
* **an optimal solution**, which use the fewest numbers, of any non-negative integer $N$ by $\mathcal{O}(\sqrt[4]{N})$.

The randomized polynomial algorithm is [based on MICHAEL 0. RABIN et al.](https://onlinelibrary.wiley.com/doi/10.1002/cpa.3160390713)

## Deploy on local

Node.js environment must be pre-installed.

```bash
git clone https://github.com/powergee/four-square-sum.git
cd ./four-square-sum

npm install         # Install dependencies and compiled wasm module
npm start           # Start local server
```

## Algorithm

* Rust(with `num_bigint`) Implementation: [lib.rs](https://github.com/powergee/four-square-sum-wasm/blob/master/src/lib.rs)
* JS(with `BigInt`) Implementation: [FourSquare.js](https://github.com/powergee/four-square-sum/blob/main/src/algorithms/js/FourSquare.js)

### Common background

* If and only if $N$ is a perfect square, it is represented by only **one square**: $(\sqrt{N})^2$
* If and only if $N$ is of the form $4^k (8m+7)$ for integers $k$ and $m$, it is represented by **four squares**, and it can't be represented by three or fewer squares. (proved by [Adrien-Marie Legendre](https://en.wikipedia.org/wiki/Adrien-Marie_Legendre))
* Otherwise, $N$ can be represented by **two or three squares**.

So, if there are any efficient algorithms to find a representation of $N\not=4^k (8m+7)$ with three(or two) squares, we can find solutions for any non-negative integers. Let's say this algorithm as *findSumOfThree(N)*.

Than the algorithm *findSolution(N)* which find a solution for any non-negative integers works like below.

* $N\text{ is a perfect square}$: Return $(\sqrt{N})^2+0+0+0$.
* $N\text{ is a multiple of 4}$: Calculate *findSolution(N/4)* and multiply $2$ to each element.
* $N = 8m+7$: Calculate *findSumOfThree(N-4)* and obtain $N-4=x^2+y^2+z^2$. Than, $N=2^2+x^2+y^2+z^2$ is satisfied, so return $N=2^2+x^2+y^2+z^2$.
* $\text{Otherwise}$: return *findSumOfThree(N)*.

This project implements *findSumOfThree(N)* by two ways: (1)calculate an arbitrary solution by **randomized polynomial-time algorithms**, (2)or an optimal solution which use the fewest numbers by **Pollard-rho factorization algorithm** $\mathcal{O}(\sqrt[4]{N})$.

### 1. An arbitrary solution

This approach is mainly based on [the paper by Michael O. Rabin and Jeffery O. Shallit](https://onlinelibrary.wiley.com/doi/10.1002/cpa.3160390713) to find an arbitrary solution in randomized polynomial-time.

First of all, they revisited the fact that there is a randomized algorithm to represent any prime numbers $p=4k+1$ by a sum of two squares, requiring an $\mathcal{O}(\log p)$ operations. (Rabin [2])

They found $N (N\not=4^k (8m+7))$ can be written as $x+p$ or $x+2p$ ( $p$ is a prime number of the form $4k+1$) and using an algorithm mentioned in the previous paragraph, $N$ can be represented with three squares. To select $p$ efficiently, a pseudo-random function must be used.

For more mathematical prooves and implementations, review their paper (especially, section 4).

### 2. An optimal solution which use the fewest numbers

As described shortly on previous sub-section, we have efficient algorithm to find three square sum representation of an integer $N (N\not=4^k (8m+7))$.

However, it is not guaranteed to find an optimal solution which use the fewest numbers. If and only if $N$ is a composite number with primes $p_i$ where $p_i\equiv1 \mod 4$, $N$ is represented by **only two squares**, using [Brahmagupta–Fibonacci identity](https://en.wikipedia.org/wiki/Brahmagupta%E2%80%93Fibonacci_identity).

To implement this approach, prime factorization must be preceded. **Pollard-rho factorization algorithm** can factorize an positive integer $N$ in $\mathcal{O}(\sqrt{p})$ where $p$ is a smallest prime factor.

## References

* [1] Rabin, M.O. and Shallit, J.O. (1986), Randomized algorithms in number theory. Comm. Pure Appl. Math., 39: S239-S256. [https://doi.org/10.1002/cpa.3160390713](https://doi.org/10.1002/cpa.3160390713)
* [2] Rabin, M. O., Efficient Algorithms, Lecture Notes MIT, 1977, transcribed by M. Lui.
* [3] Lagrange's four-square theorem, [https://en.wikipedia.org/w/index.php?title=Lagrange%27s_four-square_theorem&oldid=1070361127](https://en.wikipedia.org/w/index.php?title=Lagrange%27s_four-square_theorem&oldid=1070361127) (last visited Feb. 24, 2022).
* [4] Legendre's three-square theorem, [https://en.wikipedia.org/w/index.php?title=Legendre%27s_three-square_theorem&oldid=1030003352](https://en.wikipedia.org/w/index.php?title=Legendre%27s_three-square_theorem&oldid=1030003352) (last visited Feb. 24, 2022).
* [5] Fermat's theorem on sums of two squares, [https://en.wikipedia.org/w/index.php?title=Fermat%27s_theorem_on_sums_of_two_squares&oldid=1064865677](https://en.wikipedia.org/w/index.php?title=Fermat%27s_theorem_on_sums_of_two_squares&oldid=1064865677) (last visited Feb. 24, 2022).
* [6] Pollard's rho algorithm, [https://en.wikipedia.org/w/index.php?title=Pollard%27s_rho_algorithm&oldid=1044484908](https://en.wikipedia.org/w/index.php?title=Pollard%27s_rho_algorithm&oldid=1044484908) (last visited Feb. 24, 2022).
* [7] Brahmagupta–Fibonacci identity, [https://en.wikipedia.org/w/index.php?title=Brahmagupta%E2%80%93Fibonacci_identity&oldid=1072168983](https://en.wikipedia.org/w/index.php?title=Brahmagupta%E2%80%93Fibonacci_identity&oldid=1072168983) (last visited Feb. 24, 2022).
