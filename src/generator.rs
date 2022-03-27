use rand::Rng;

pub fn gen_ran() -> u8 {
    let mut rnd = rand::thread_rng();
    let n: u8 = rnd.gen();
    n
}