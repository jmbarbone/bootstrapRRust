#' Bootstraps
#' 
#' Bootstrap functions in R, C++, and rust
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
#' bootstrap_r(runif(100), 100)

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
  
  for(i in out) {
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
#' @importFrom microbenchmark microbenchmark
run_benchmarks <- function(x = runif(2000), R = 2000, times = 10, ...) {
  microbenchmark(
    bootstrap_r(x, R),
    bootstrap_loop(x, R),
    bootstrap_cpp(x, R),
    times = times,
    ...
  )
}
