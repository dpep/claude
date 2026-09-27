# Numerical Methods and Statistics

Reference for the `algorithmist` persona. Floating point is not real arithmetic, and statistics from a handful of samples are not facts. Use the libraries (BLAS/LAPACK-backed linear algebra, SciPy-grade solvers), keep error in mind where values are built, and round to the precision you actually have.

## When you see this → reach for

| Problem shape | Reach for | Trade-off | Use |
|---|---|---|---|
| Money, or decimal quantities humans check | decimal or integer minor units | slower than floats | `rust_decimal`, Python `decimal`, integer cents |
| Comparing floats | tolerance (relative + absolute), never `==` | the tolerance is a choice | `approx` crates, `math.isclose` |
| Sorting or ordering floats with NaN | total order | NaN placement is explicit | `f64::total_cmp` |
| Summing many floats of mixed magnitude | pairwise or Kahan (compensated) summation | a few extra flops | NumPy `sum` (partial pairwise), Python `math.fsum` (correctly rounded) |
| Subtracting nearly equal numbers | reformulate (e.g. `log1p`, `expm1`, `hypot`) | know the identities | stdlib math functions |
| Matrices: solve, factor, least squares, SVD | BLAS/LAPACK-backed library | a dependency | `faer`, `nalgebra`, `ndarray` (Rust); NumPy/SciPy |
| Root of f(x) = 0 | bisection (safe) or Brent; Newton if you have the derivative and a good start | Newton can diverge | SciPy `optimize.root_scalar`, `argmin` |
| Minimize a smooth function | quasi-Newton (L-BFGS) | needs gradients or estimates | SciPy `optimize.minimize` (`L-BFGS-B`), `argmin` |
| Minimize without gradients | Nelder-Mead, or a grid/random search first | slow in high dimensions | SciPy, `argmin` |
| Fit or estimate between samples | linear or spline interpolation | splines can overshoot | SciPy `interpolate` |
| Integrate a function | adaptive quadrature | — | SciPy `integrate.quad` |
| Reproducible randomness (tests, simulations) | seeded, portable PRNG | not for secrets | `rand_chacha`/`rand_pcg` with a fixed seed; NumPy `default_rng(seed)` |
| Randomness for secrets | OS CSPRNG | — | see [cryptography.md](./cryptography.md) |
| Mean and variance in one pass / streaming | Welford's algorithm | — | a few lines; `statrs`, NumPy |
| Percentiles (p50, p99) of a stream | quantile sketch or HDR histogram | bounded error | HdrHistogram, t-digest, DDSketch |
| Is A faster/better than B? | interleaved runs + confidence interval (bootstrap or t-test) | needs enough runs | `criterion` (Rust), SciPy `stats`, hyperfine |

## Reach-for notes

- **Floats:** about 15–16 significant decimal digits for `f64`, 7 for `f32`. `0.1 + 0.2 != 0.3`. Catastrophic cancellation — subtracting nearly equal values — loses digits silently. Accumulate in `f64` even when storing `f32`. Goldberg's paper is the must-read.
- **Summation order changes the answer.** Parallel reductions are therefore not bit-reproducible unless the reduction order is fixed. Decide whether you need that before promising it.
- **Linear algebra:** never invert a matrix to solve a system; use a factorization (LU, QR, Cholesky). Watch the condition number: an ill-conditioned problem has no accurate answer to find.
- **Solvers:** bracket first (bisection or Brent always converges on a sign change), then accelerate. Check convergence criteria and iteration caps.
- **Randomness:** Rust's `StdRng` is documented as non-portable — its algorithm may change between versions — so tests that need reproducible streams should name a generator (ChaCha, PCG) explicitly ([docs](https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html)).
- **Percentiles:** averaging percentiles across machines or intervals is wrong. Merge histograms or sketches, then read the percentile (DDSketch and t-digest are mergeable; see [probabilistic-structures.md](./probabilistic-structures.md)).
- **A/B and benchmark comparisons:** interleave A and B so drift hits both, take enough runs to estimate spread, and report an interval, not a point. The bootstrap (resample the observed runs) needs no distributional assumption. A difference inside the noise band is "no measurable difference".
- **Round to the precision you have.** A statistic from a dozen samples has two or three significant figures. Round where the value is built, so JSON output doesn't carry false digits.

## Classic failure modes

- Floats for currency.
- `==` on computed floats; NaN in a comparator.
- Naive one-pass variance (`E[x²] − E[x]²`), which cancels catastrophically. Use Welford.
- Inverting matrices; ignoring conditioning.
- Seeding from time in a test, then chasing an unreproducible failure.
- Averaging p99s; reporting a mean where a distribution matters.
- Declaring a winner from one run each.

**Depth:** Goldberg, "What Every Computer Scientist Should Know About Floating-Point Arithmetic" (ACM Computing Surveys, 1991); Higham, *Accuracy and Stability of Numerical Algorithms*; Press et al., *Numerical Recipes*; Skiena's catalog: "Numerical Problems"; [cp-algorithms.com](https://cp-algorithms.com/) numerical methods.
