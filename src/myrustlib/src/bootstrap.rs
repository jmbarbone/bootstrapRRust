use rand::Rng;

//#[no_mangle]
pub extern "C" fn bootstrap_rs(x: &Vec<f32>, _r: &i32) -> Vec<f32> {
    
    let n = x.len() as i32;
    let estimate = mean(&x) as f32;
    let mut out = vec![];

    for _ in 0..*_r {

        let samp: Vec<f32> = sample_with_replacement(x, &(n as usize));
        let samp_est: f32 = mean(&samp);
        let mut samp_se: f32 = sterr(&samp);

        if samp_se == 0.0 {
            samp_se = 1.0 / (2.0 * n as f32);
        }

        out.push((samp_est - estimate) / samp_se);
    }
    out
}

fn sample_with_replacement(x_vector: &Vec<f32>, n_replacements: &usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    let size:usize = x_vector.len();
    
    vec![0; *n_replacements]
        .into_iter()
        .map(|_| {
            x_vector[rng.gen_range(0, size) as usize]
        }).collect()
}

fn mean(numbers: &Vec<f32>) -> f32 {
    numbers.iter().sum::<f32>() as f32 / numbers.len() as f32
}

fn variance(numbers: &Vec<f32>) -> f32 {
    let n: usize = numbers.len();
    let estimate: f32 = mean(&numbers);
    let mut diffs: Vec<f32> = vec![];
    for i in numbers.iter() {
        let d = i - estimate;
        diffs.push(d * d);
    }
    diffs.iter().sum::<f32>() / (n - 1) as f32
}

fn stdev(numbers: &Vec<f32>) -> f32 {
    variance(&numbers).sqrt() as f32
}

fn sterr(numbers: &Vec<f32>) -> f32 {
    stdev(&numbers) as f32 / (numbers.len() as f32).sqrt() as f32
}

