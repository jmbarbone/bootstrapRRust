// extern crate rand;
use rand::Rng;
use libc::c_double;
use libc::c_int;
use std::slice;
// use std::mem::forget;

// x = the vector of numbers to sample
// _r = the number of resamples to perform on the vector
// size = the size of the vector
#[no_mangle]
pub extern fn bootstrap_rs(x: *const c_double, _r: c_int, size: c_int) -> *const c_double {
    let n: usize = size as usize;
    let n2: f64 = 1.0 / (2.0 * n as f64);

    // Slice apart pointer and transfer to vector
    let numbers: Vec<f64> = unsafe{
        // assert!(!x.is_null());
        slice::from_raw_parts(x, n)
    }.iter().cloned().collect();

    let estimate: f64 = mean(&numbers);

    vec![0.0 as f64; _r as usize]
        .into_iter()
        .map(|_| {
            let samp: Vec<f64> = sample_with_replacement(&numbers, &n);
            let samp_est: f64 = mean(&samp);
            let mut samp_se: f64 = sterr(&samp);

            if samp_se == 0.0 {
                samp_se = n2;
            }

            (samp_est - estimate) / samp_se
        }).collect::<Vec<f64>>().as_mut_ptr()
}

pub fn sample_with_replacement(x_vector: &Vec<f64>, n_replacements: &usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let size: usize = x_vector.len();
    
    vec![0.0 as f64; *n_replacements]
        .into_iter()
        .map(|_| {
            x_vector[rng.gen_range(0, size) as usize]
        }).collect::<Vec<f64>>()
}

pub fn mean(numbers: &Vec<f64>) -> f64 {
    numbers.iter().sum::<f64>() / (numbers.len() as f64)
}

pub fn variance(numbers: &Vec<f64>) -> f64 {
    let n: usize = numbers.len();
    let estimate: f64 = mean(&numbers);

    numbers
        .iter()
        .map(|&x| {
            let d: f64 = &x - estimate;
            d * d
        }).sum::<f64>() / (n as f64 - 1.0)
}

pub fn stdev(numbers: &Vec<f64>) -> f64 {
    variance(&numbers).sqrt() as f64
}

pub fn sterr(numbers: &Vec<f64>) -> f64 {
    stdev(&numbers) / (numbers.len() as f64).sqrt()
}
