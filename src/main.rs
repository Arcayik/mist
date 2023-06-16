mod astro;
use astro::Astronaut;

type Percentage = u8;
type Celsius = f32;

fn main() {
    let mut astro = Astronaut::default();

    astro.rad_exposure = 1000;
    dbg!(&astro);
    println!("{}%", astro.calculate_overall());
}
