#' Bootstraps
#' 
#' Bootstrap functions in R and Rust
#' 
#' @param x A vector of numbers
#' @param R Number of replications
#' @param times Number of times to run benchmarks
#' @param ... Additional arguments passed to [microbenchmark::microbenchmark()]
#' 
#' @export
#' @name bootstrap
#' @importFrom stats sd runif
#' @examples
#' numbers <- runif(100)
#' resamples <- 100
#' bootstrap_r(numbers, resamples)
#' bootstrap_loop(numbers, resamples)
#' bootstrap_rs(numbers, resamples)
#' }

bootstrap_r <- function(x, R) {
  estimate <- mean(x, na.rm = TRUE)
  n <- length(x)
  
  vapply(seq(R), function(i) {
    samp <- sample(x, length(x), replace = TRUE)
    samp_estimate <- mean(samp, na.rm = TRUE)
    se <- sd(samp, na.rm = TRUE) / sqrt(n)
    
    if(se == 0) {
      se <- 1 / (2 * n)
    }
    
    (samp_estimate - estimate) / se
    
  }, double(1))
}

#' @rdname bootstrap
#' @export
bootstrap_loop <- function(x, R) {
  estimate <- mean(x, na.rm = TRUE)
  n <- length(x)
  out <- double(R)
  
  for(i in seq_along(out)) {
    samp <- sample(x, length(x), replace = TRUE)
    samp_estimate <- mean(samp, na.rm = TRUE)
    se <- sd(samp, na.rm = TRUE) / sqrt(n)
    
    if(se == 0) {
      se <- 1 / (2 * n)
    }
    
    out[i] <- (samp_estimate - estimate) / se
  }
  
  out
  
}

#' @rdname bootstrap
#' @export
#' @useDynLib bootstrapRRust bootstrap_wrapper
#' @export
bootstrap_rs <- function(x, R) {
  .Call(bootstrap_wrapper, as.double(x), as.integer(R))
}


#' @rdname bootstrap
#' @export
#' @importFrom microbenchmark microbenchmark
run_benchmarks <- function(x = runif(2000), R = 2000, times = 10, ...) {
  microbenchmark(
    `base R vapply` = bootstrap_r(x, R),
    `base R loop` = bootstrap_loop(x, R),
    `Rust` = bootstrap_rs(x, R), 
    times = times,
    ...
  )
}

#' @rdname bootstrap
#' @export
main <- function() {
  x <- runif(2000)
  R <- 200
  bsr <- bootstrap_r(x, R)
  bsrs <- bootstrap_rs(x, R)
  plot(density(bsr), main = "Bootstrapped distributed of test statistic")
  lines(density(bsrs))
}
