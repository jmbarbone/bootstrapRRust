use stats::mean;
use stats::stdev;
use rand::seq;
use rand::random;
use std::num::sqrt;

pub extern fn bootstrap(x: Vec<f32>, R: i32) -> Vec<f32> {
    
    let n = x.len() as i32;
    let est = mean(x.iter()) as f32;
    let sqrt_n = x.stats::sqrt() as f32;
    let mut out = vec![0.0; n];

    for i in 0..R {
        let samp = sample_iter(x, 1..n);
        let samp_est = mean(samp) as f32;
        let mut samp_se = stats::stdev(samp) as f32 / sqrt_n as f32;

        if samp_se == 0 {
            let samp_se = 1 / (2 * n) as f32;
        }

        out.push((samp_est - est) / sampe_se as f32);
    }
    out
}
