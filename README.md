
<!-- README.md is generated from README.Rmd. Please edit that file -->

# bootstrapRRust

<!-- badges: start -->

<!-- badges: end -->

The goal of bootstrapRRust is to simply demonstrate implementation of a
Rust function into R.

## Installation

You can install the development version from
[GitHub](https://github.com/jmbarbone/bootstrapRRust) with:

``` r
# install.packages("devtools")
devtools::install_github("jmbarbone/bootstrapCppRs")
```

## Example

This is a basic example which shows you how to solve a common problem:

``` r
library(bootstrapRRust)
run_benchmarks(x = rchisq(1000, 2), R = 2000)
#> Unit: nanoseconds
#>                  expr       min        lq      mean    median        uq
#>     bootstrap_r(x, R) 206178800 211468400 222038400 220384900 222727300
#>  bootstrap_loop(x, R) 211716400 214731800 232728210 221919450 228895100
#>    bootstrap_rs(x, R)       700      1000     11830      4750      5100
#>        max neval cld
#>  266293800    10   b
#>  295296500    10   b
#>      89300    10  a
```
