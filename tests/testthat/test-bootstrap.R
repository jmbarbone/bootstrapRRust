context("It just works")

x <- runif(100)
R <- 100

test_that("vapply", {
  expect_visible(bootstrap_r(x, R))
})

test_that("loop", {
  expect_visible(bootstrap_loop(x, R))
})

test_that("Rust", {
  expect_visible(bootstrap_rs(x, R))
})

test_that("benches", {
  expect_visible(run_benchmarks(runif(100), 10))
})
