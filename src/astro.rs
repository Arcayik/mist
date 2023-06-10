use self::LifeState::*; //only need to type 'Awake' instead of 'LifeState::Awake'
use crate::{Percentage, Rad, Celsius};
use std::ops::Range;

#[derive(Debug)] #[allow(unused)]
pub struct Astronaut {
    pub name: String,
    pub state: LifeState,
    pub heartrate: i32,
    pub blood_pressure: (i32, i32),
    pub respiration: i32,
    pub oximetry: Percentage,
    pub temperature: Celsius,
    pub hydration: i32,
    pub rad_exposure: Rad,
}

#[derive(Debug)] #[allow(unused)]
pub enum LifeState {
    Awake,
    Sleeping,
    Cryo,
    Deceased,
}

impl Default for Astronaut {
    fn default() -> Astronaut {
        Astronaut {
            name:           String::from(""),
            state:          Awake,
            heartrate:      80,
            blood_pressure: (120, 80),
            respiration:    15,
            oximetry:       100,
            temperature:    36.5,
            hydration:      140,
            rad_exposure:   0,
        }
    }
}

impl Astronaut {
    pub fn calculate_overall(&self) -> f32 {
        match self.state {
            Deceased => return 0.0,
            _ => {}
        };

        println!("falloff: {}", falloff_linear(50.0, 80.0..120.0, 2.0, 0.5));
        let heart_percent = falloff_custom(
            self.heartrate as f32,
            80.0..120.0,
            |l| l.powi(2),
            |u| u*10.0
        );
        let bp_percent = falloff_custom(
            self.blood_pressure.0 as f32,
            100.0..120.0,
            |l| 0.5 * l.powi(2),
            |u| 0.5 * u.powi(2)
            );
        //TODO: finish calculating astronaut health based on all factors
        (heart_percent + bp_percent) / 2.0
    }
}

// MATHEMATICAL FUNCTIONS

fn falloff_linear(input: f32, bound: Range<f32>, lower_slope: f32, upper_slope: f32) -> f32 {
    // slope is % per each 1 unit outside of range
    let multiplier: f32 = match input {
        input if input < bound.start => {
            1.0 - 0.01 * (bound.start - input) * lower_slope
        },
        input if input > bound.end => {
            1.0 - 0.01 * (input - bound.end) * upper_slope
        },
        _ => 1.0
    };

    if multiplier.is_sign_positive() { return multiplier; }
    else { return 0.0; }
}

fn falloff_custom<L,U>(input: f32, bound: Range<f32>, lower_func: L, upper_func: U) -> f32
where U: Fn(f32) -> f32, L: Fn(f32) -> f32 {
    // slope is % per each 1 unit outside of range
    let multiplier: f32 = match input {
        input if input < bound.start => {
            let diff = bound.start - input;
            1.0 - 0.01 * lower_func(diff)
        },
        input if input > bound.end => {
            let diff = input - bound.end;
            1.0 - 0.01 * upper_func(diff)
        },
        _ => 1.0
    };

    if multiplier.is_sign_positive() { return multiplier; }
    else { return 0.0; }
}
