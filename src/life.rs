use rand::distributions::{Uniform, Distribution};
extern crate rand;

pub fn gen_board(size: i32) -> Vec<u8>
{
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..2);
    (0..size*size).map(|_| die.sample(&mut rng)).collect()
}

pub fn life(grid: Vec<u8>, size: i32) -> Vec<u8>
{
    // Bad, allocating for every frame. But whatever.
    let mut next = grid.clone();

    for cx in 0..size {
        for cy in 0..size {
            let mut sum = 0;

            for y in cy-1..cy+2 {
                for x in cx-1..cx+2 {
                    if !(cx == x && cy == y) {
                        if x >= 0 && x < size && y >= 0 && y < size {
                            sum += grid[(size*y + x) as usize];
                        }
                    }
                }
            }
            let neigbours = sum;
            if grid[(size*cy + cx) as usize] == 1 {
                if neigbours < 2 || neigbours > 3 {
                    next[(size*cy + cx) as usize] = 0;
                }
            }
            else
            {
                if neigbours == 3 {
                    next[(size*cy + cx) as usize] = 1;
                }
            }
        }
    }
    next
}

// viva:
// < 2: muerte
// 2,3: vive
// > 3, muerte

// muerta:
// 3: revive

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let size = 3;
        let mut board: Vec<u8> = vec![0; size*size];
        board[0*size + 1] = 1;
        board[1*size + 1] = 1;
        board[2*size + 1] = 1;
        let mut board_expected: Vec<u8> = vec![0; size*size];
        board_expected[1*size + 0] = 1;
        board_expected[1*size + 1] = 1;
        board_expected[1*size + 2] = 1;
        assert_eq!(board_expected, life(board, size as i32));
    }
}
