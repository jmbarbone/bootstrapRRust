// extern crate rand;
use rand::Rng;
use libc::c_double;
use libc::c_int;
use std::slice;
use std::mem::forget;


// x = the vector of numbers to sample
// _r = the number of resamples to perform on the vector
// size = the size of the vector
#[no_mangle]
pub extern fn bootstrap_rs(x: *const c_double, _r: c_int, size: c_int) -> *const c_double {
    let n = size as usize;

    // Slice apart pointer
    let slices = unsafe{
        assert!(!x.is_null());
        slice::from_raw_parts(x, n)
    };

    // Transfer slices into vec f64
    let mut numbers: Vec<f64> = vec![0.0; n];

    for i in 0..n {
        numbers[i] = slices[i] as f64;
    }

    forget(slices);

    // Perform first sample
    let estimate = mean(&numbers) as f64;
    let mut result: Vec<f64> = vec![0.0; _r as usize];

    for i in 0.._r as usize {

        let samp: Vec<f64> = sample_with_replacement(&numbers, &(n as usize));
        let samp_est: f64 = mean(&samp);
        let mut samp_se: f64 = sterr(&samp);

        if samp_se == 0.0 {
            samp_se = 1.0 / (2.0 * n as f64);
        }

        result[i] = ((samp_est - estimate) / samp_se) as f64;
    }

    forget(estimate);
    let out = result.as_mut_ptr();
    forget(result);
    out
}

pub fn sample_with_replacement(x_vector: &Vec<f64>, n_replacements: &usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let size: usize = x_vector.len();
    
    vec![0; *n_replacements]
        .into_iter()
        .map(|_| {
            x_vector[rng.gen_range(0, size) as usize]
        }).collect()
}

pub fn mean(numbers: &Vec<f64>) -> f64 {
    numbers.iter().sum::<f64>() as f64 / numbers.len() as f64
}

pub fn variance(numbers: &Vec<f64>) -> f64 {
    let n: usize = numbers.len();
    let estimate: f64 = mean(&numbers);
    let mut diffs: Vec<f64> = vec![];
    for i in numbers.iter() {
        let d: f64 = i - estimate;
        diffs.push(d * d);
    }
    diffs.iter().sum::<f64>() / (n - 1) as f64
}

pub fn stdev(numbers: &Vec<f64>) -> f64 {
    variance(&numbers).sqrt() as f64
}

pub fn sterr(numbers: &Vec<f64>) -> f64 {
    stdev(&numbers) as f64 / (numbers.len() as f64).sqrt() as f64
}

