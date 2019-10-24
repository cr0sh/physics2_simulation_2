use phys::MarbleBoard;
use rayon::prelude::*;

const NUM_TESTS: usize = 50000;

fn main() {
    println!("num_steels,,prob");
    for num_steels in 0..=3600 {
        let count = [num_steels;NUM_TESTS].par_iter().map(|x| if MarbleBoard::new_shuffled(*x).is_connected(){1} else {0})
            .sum::<usize>();
        println!("{},,{}", num_steels, f64::from(count as u32) / f64::from(NUM_TESTS as u32));
    }
}
