use rustfft::{num_complex::Complex, FftPlanner};
const FFT_SIZE: usize = 1024;
const FFT_ROOM: usize = 256; //some headroom on either end to discard artifacts
#[allow(dead_code)]

pub fn fit(mut input: Vec<(f64, f64)>, top: f64, cutoff: f64) -> Vec<(f64, f64)> {
    //first we order the points by the X axis, so that there is no looping
    input.sort_unstable_by(|(x1, _), (x2, _)| { x1.total_cmp(x2)});
    
    let probe_distance = 0.075;

    
    //normalize input to 1.0 < y < 2.0  (arbitrary, mostly just want to make sure theres no negatives)
    let max = input.iter().map(|(_, y)| {*y}).reduce(f64::max).unwrap();
    let min = input.iter().map(|(_, y)| {*y}).reduce(f64::min).unwrap();
    input = input.iter().map(|(x, y)| {
        let n = (y - min) / (max - min) + 1.0;
        (*x, n)
    }).collect();


    let left_probe_x = input.first().unwrap().0 + top * probe_distance;
    let left_probe_y = vec_interpolate(&input, left_probe_x, 0.0, 0.0);
    let left_slope =   //rise over run
        (left_probe_y - input.first().unwrap().1) / 
        (left_probe_x - input.first().unwrap().0);

    let right_probe_x = input.last().unwrap().0 - top * probe_distance;
    let right_probe_y = vec_interpolate(&input, right_probe_x, 0.0, 0.0);
    let right_slope =   //rise over run
        (input.last().unwrap().1 - right_probe_y) / 
        (input.last().unwrap().0 - right_probe_x);




    let mut buffer: Vec<Complex<f64>> = Vec::new();
    for i in 0..FFT_SIZE {
        //index expressed as 0.0-1.0
        let interp = (i as f64 - FFT_ROOM as f64) / (FFT_SIZE-(1+FFT_ROOM*2)) as f64;    
        let ix = top * interp;   //the associated X value

        let n = vec_interpolate(&input, ix, left_slope, right_slope);

        buffer.push(Complex::new(n, 0.0))
    }


    let mut planner = FftPlanner::new();
    let fft_forward = planner.plan_fft_forward(FFT_SIZE);
    let fft_inverse = planner.plan_fft_inverse(FFT_SIZE);

    fft_forward.process(&mut buffer);

    //low pass filter
    let cutoff_bins = ((FFT_SIZE as f64 / 2.0) * cutoff) as usize;
    buffer.splice(
        cutoff_bins..FFT_SIZE-cutoff_bins, 
        std::iter::repeat(Complex::new(0.0, 0.0)).take(((FFT_SIZE/2)-(cutoff_bins)) *2)
    );


    fft_inverse.process(&mut buffer);

    //scale and format the output
    buffer
        .iter()
        .enumerate()
        .map(|(x, complex)| {
            ((x as f64 - FFT_ROOM as f64) / (FFT_SIZE-FFT_ROOM*2) as f64 * top, ((complex.to_polar().0 / (FFT_SIZE as f64) - 1.0) * (max - min) + min))
        }).collect()
}



pub fn vec_interpolate(input: &Vec<(f64, f64)>, t: f64, left_slope: f64, right_slope: f64) -> f64 {
    //find the nearest points
    let mut lower = (&0.0, &0.0);
    for (x, y) in input {
        if x < &t {lower = (x, y)}
    }
    let mut higher = (&0.0, &0.0);
    for (x, y) in input.iter().rev() {
        if x > &t {higher = (x, y)}
    }

    //lerp between lower and higher point
    let lerp_t = (t - lower.0) / (higher.0 - lower.0 + 0.0001);
    let mut out = lower.1 + (lerp_t * (higher.1 - lower.1));

    //exceptions for first and last points, extrapolate using slope
    if higher == (&0.0, &0.0) { 
        out = lower.1 + right_slope * (t - lower.0);
    }
    if lower == (&0.0, &0.0) {
        out = higher.1 + left_slope * (t - higher.0);
    }
    out
}