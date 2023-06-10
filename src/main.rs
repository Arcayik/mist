mod astro;
use astro::Astronaut;

type Percentage = u8;
type Rad = u32;
type Celsius = f32;

fn main() {
    let mut astro = Astronaut::default();
    astro.heartrate = 125;
    dbg!(&astro);
    println!("{}%", astro.calculate_overall());
}
