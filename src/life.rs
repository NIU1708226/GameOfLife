use rand::distributions::{Uniform, Distribution};
extern crate rand;

pub fn gen_board(size: i32) -> Vec<u8>
{
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..2);
    (0..size*size).map(|_| die.sample(&mut rng)).collect()
}

pub fn life(grid: &Vec<u8>, size: i32) -> Vec<u8>
{
    // Bad, allocating for every frame. But whatever.
    let mut next = grid.clone();

    for _x in 0..size {
        for _y in 0..size {
            let mut sum = 0;

            for y in _y-1.._y+2 {
                for x in _x-1.._x+2 {
                    if !(_x == x && _y == y) {
                        if x >= 0 && x < size && y >= 0 && y < size {
                            sum += grid[(size*y + x) as usize];
                        }
                    }
                }
            }
            let neigbours = sum;
            if grid[(size*_y + _x) as usize] == 1 {
                if neigbours < 2 || neigbours > 3 {
                    next[(size*_y + _x) as usize] = 0;
                }
            }
            else
            {
                if neigbours == 3 {
                    next[(size*_y + _x) as usize] = 1;
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
