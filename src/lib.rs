use rand::prelude::*;
use std::collections::VecDeque;

const BOARD_WIDTH: usize = 60;
const BOARD_HEIGHT: usize = 60;

#[derive(PartialEq, Clone, Copy)]
pub enum Marble {
    Steel,
    Glass,
}

pub struct MarbleBoard {
    // index order: marbles[y][x]
    marbles: [[Marble; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Default for MarbleBoard {
    fn default() -> Self {
        Self {
            marbles: [[Marble::Glass; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
}

impl MarbleBoard {
    pub fn new_shuffled(num_steels: usize) -> Self {
        assert!(num_steels <= BOARD_WIDTH * BOARD_HEIGHT);
        let mut linear = [Marble::Glass; BOARD_WIDTH * BOARD_HEIGHT];
        for i in 0..num_steels {
            linear[i] = Marble::Steel;
        }
        linear.shuffle(&mut thread_rng());

        let mut x = Self::default();
        for (i, m) in x.marbles.iter_mut().enumerate() {
            m.copy_from_slice(&linear[(i * BOARD_WIDTH)..((i + 1) * BOARD_WIDTH)])
        }

        x
    }

    pub fn is_connected(&self) -> bool {
        // Use BFS algorithm to check.
        let mut q = VecDeque::<(usize, usize)>::with_capacity(4000);
        let mut visited = [[false; BOARD_WIDTH]; BOARD_HEIGHT];
        for i in 0..BOARD_WIDTH {
            if self.marbles[0][i] == Marble::Glass {
                continue;
            }
            visited[0][i] = true;
            q.push_back((0, i));
        }

        while !q.is_empty() {
            const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let pos = q.pop_front().unwrap();

            for delta in &DELTAS {
                let newpos = (pos.0 as isize + delta.0, pos.1 as isize + delta.1);
                if newpos.0 == BOARD_HEIGHT as isize {
                    // Connected to - electrode
                    return true;
                }
                if !(0 <= newpos.0
                    && newpos.0 < BOARD_HEIGHT as isize
                    && 0 <= newpos.1
                    && newpos.1 < BOARD_WIDTH as isize)
                    || visited[newpos.0 as usize][newpos.1 as usize]
                    || self.marbles[newpos.0 as usize][newpos.1 as usize] == Marble::Glass
                {
                    continue;
                }

                visited[newpos.0 as usize][newpos.1 as usize] = true;
                q.push_back((newpos.0 as usize, newpos.1 as usize));
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marbleboard_connection() {
        let mut mb = MarbleBoard::default();
        assert!(!mb.is_connected());

        for i in 0..BOARD_HEIGHT {
            mb.marbles[i][2] = Marble::Steel;
        }
        assert!(mb.is_connected());

        mb.marbles[0][2] = Marble::Glass;
        assert!(!mb.is_connected());

        mb.marbles[0][2] = Marble::Steel;
        mb.marbles[2][2] = Marble::Glass;
        assert!(!mb.is_connected());

        for i in 0..BOARD_HEIGHT {
            mb.marbles[i][4] = Marble::Steel;
        }
        assert!(mb.is_connected());

        mb.marbles[4][4] = Marble::Glass;
        assert!(!mb.is_connected());

        mb.marbles[3][3] = Marble::Steel;
        assert!(mb.is_connected());
    }

    #[test]
    fn test_shuffled_generation() {
        fn count_steels(mb: &MarbleBoard) -> usize {
            let mut ret = 0;
            for m in mb.marbles.iter() {
                for n in m.iter() {
                    if *n == Marble::Steel {
                        ret += 1;
                    }
                }
            }
            ret
        }

        for _ in 0..10 {
            for j in 100..400 {
                assert_eq!(j, count_steels(&MarbleBoard::new_shuffled(j)));
            }
        }
    }
}
