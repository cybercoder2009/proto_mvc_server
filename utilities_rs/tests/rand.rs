#[cfg(test)]
mod tests {

    use utilities_rs::rand::{rand_usize, rand_string};    

    #[test]
    fn test_rand_usize() {
        let min: usize = 0;
        let max: usize = 100;
        for _ in 0..100 {
            let rng: usize = rand_usize(min, max);
            assert!(rng >= min);
            assert!(rng <= max);
        }
    }

    #[test]
    fn test_rand_string() {
        let min: usize = 0;
        let max: usize = 100;
        println!("rand string = {}", rand_string(min, max));
    }
}