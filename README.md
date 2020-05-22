
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
devtools::install_github("jmbarbone/bootstrapRRust")
```

## Example

This is a basic example which shows you how to solve a common problem:

``` r
library(bootstrapRRust)
run_benchmarks(x = rchisq(1000, 2), R = 2000)
#> Unit: nanoseconds
#>                  expr       min        lq      mean    median        uq
#>     bootstrap_r(x, R) 156498000 160187000 181642920 163806950 175532300
#>  bootstrap_loop(x, R) 156720800 160389300 174359400 167800550 194908100
#>    bootstrap_rs(x, R)       700      3300     12550      4000      5100
#>        max neval cld
#>  282047000    10   b
#>  200857900    10   b
#>      90700    10  a
```
