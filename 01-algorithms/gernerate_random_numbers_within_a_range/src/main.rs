use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..1.0));

    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }

    let normal = Normal::new(2.0, 3.0).unwrap();
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 0) distribution", v);
}
