use rand::{Rng, thread_rng, rngs::ThreadRng};

pub fn rand_usize(min: usize, max: usize) -> usize {
    let mut rng: ThreadRng = thread_rng();
    rng.gen_range(min..=max)
}

pub fn rand_string(min: usize, max: usize) -> String {
    let characters: &str = "abcdefghijklm nopqrstuvwxyz ABCDEFGHIJKLM NOPQRSTUVWXYZ 0123456789 ,.!?";
    let mut rng: ThreadRng = thread_rng();
    let len: usize = rand_usize(min, max);
    (0..len).map(|_| {
        let idx = rng.gen_range(0..characters.len());
        characters.chars().nth(idx).unwrap()
    })
    .collect()
}