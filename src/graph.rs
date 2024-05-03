use std::f64::consts::PI;

pub fn turn_rate_of_load_factor(true_airspeed: f64, load_factor: f64) -> f64 /* turn rate */ {
    (load_factor * 9.8 * 3.6 * 180_f64) / (PI * true_airspeed)
}

pub fn turn_rate_of_turn_radius(true_airspeed: f64, turn_radius: f64) -> f64 {
    let squared_fraction = (turn_radius*3.6/180.0).powi(2);

    (turn_radius*3.6*true_airspeed) / (squared_fraction * 180.0 * PI)
}

pub fn range_iterator(from: f64, to: f64, steps: i32) -> impl Iterator<Item = f64> {
    let range = to-from;
    let step_size = range / (steps-1) as f64;
    
    (0..=steps-1).map(move |x| {
        (x as f64 * step_size) + from
    })
}


pub fn turn_radius(true_airspeed: f64, turn_rate: f64) -> f64 {
    (true_airspeed / 3.6) / (turn_rate * PI / 180.0)
}

pub fn load_factor(true_airspeed: f64, turn_rate: f64) -> f64 {
    (true_airspeed / 3.6) * (turn_rate * PI / 180.0) / 9.8
}