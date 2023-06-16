use self::LifeState::*; //only need to type 'Awake' instead of 'LifeState::Awake'
use crate::{Percentage, Celsius};
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
    pub rad_exposure: i32,
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

        let heart_percent = falloff(
            self.heartrate as f32,
            80.0..120.0,
            |l| l.powi(2),
            |u| u*10.0
        );
        let bp_percent = falloff( // TODO: make more accurate to medical data
            self.blood_pressure.0 as f32,
            100.0..120.0,
            |l| 0.5 * l.powi(2),
            |u| 0.5 * u.powi(2)
            )
            * falloff(
            self.blood_pressure.1 as f32,
            80.0..100.0,
            |l| 0.5 * l.powi(2),
            |u| 0.5 * u.powi(2)
            );
        let breath_percent = falloff(
            self.respiration as f32,
            12.0..18.0,
            |l| 0.007 * l.powi(2),
            |u| 0.01 * u
            );
        let oxy_percent = falloff(
            self.oximetry as f32,
            95.0..100.0,
            |l| 0.0286 * l,
            |u| 0.013 * u
            );
        let temp_percent = falloff(
            self.temperature as f32,
            36.5..37.3,
            |l| 0.5 * l.powi(2),
            |u| 0.125 * u.powi(2)
            );
        let hydration_percent = falloff(
            self.hydration as f32,
            135.0..145.0,
            |l| 0.2 * l,
            |u| 0.2 * u
            );
        let rad_percent = falloff(
            self.rad_exposure as f32,
            0.0..500.0,
            |_l| 1.0,
            |_u| 1.0 //- 0.0000000494 * (u-4500.0).powi(2) TODO
            );

        //TODO: finish calculating astronaut health based on all factors
        heart_percent * bp_percent * breath_percent * oxy_percent * temp_percent * hydration_percent * rad_percent * 100.0
    }
}

// MATHEMATICAL FUNCTIONS

fn falloff<L,U>(input: f32, bound: Range<f32>, lower_func: L, upper_func: U) -> f32
where U: Fn(f32) -> f32, L: Fn(f32) -> f32 {
    // slope is % per each 1 unit outside of range
    let multiplier: f32 = match input {
        input if input < bound.start => {
            let diff = bound.start - input;
            dbg!(lower_func(diff));
            1.0 - 0.01 * lower_func(diff)
        },
        input if input > bound.end => {
            let diff = input - bound.end;
            dbg!(upper_func(diff));
            1.0 - 0.01 * upper_func(diff)
        },
        _ => 1.0
    };

    dbg!(multiplier);
    match multiplier {
        multiplier if multiplier < 0.0 => 0.0,
        multiplier if multiplier > 1.0 => 1.0,
        _ => multiplier
    }
}
