use clap::{Arg, Command};
use rayon::prelude::*;
use std::{
    thread,
    time::Duration,
    time::Instant,
};

fn main() {
    const SIZE: usize = 40;

    let starting_state: [[u8; SIZE]; SIZE] = [
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];

    let matches = Command::new("cgol-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Jackson Ly (JumpyJacko)")
        .about("A small version of Conway's Game of Life")
        .arg(Arg::new("fill")
            .short('f')
            .long("fill")
            .default_value(".")
            .help("Pick characters to fill whitespace\n    (use only one of that character)"))
        .arg(Arg::new("line")
            .short('l')
            .long("line")
            .default_value("#")
            .help("Pick characters to use for the lines\n    (use only one of that character)"))
        .arg(Arg::new("frame-time")
            .short('t')
            .long("frame-time")
            .default_value("200")
            .help("Input how long to hold a frame\n    (in millis)"))
        .get_matches();

    let fill_char: &str = matches.get_one::<String>("fill").unwrap();
    let line_char: &str = matches.get_one::<String>("line").unwrap();
    let frame_time: u64 = matches.get_one::<String>("frame-time").unwrap().parse::<u64>().unwrap();

    fn check_neighbours(state: &[[u8; 40]; 40], x: usize, y: usize) -> u8 {
        /*
        If the index is (x, y)
        [[x-1, y-1],[x, y-1],[x+1, y-1]],
        [[ x-1, y ],[ x, y ],[ x+1, y ]],
        [[x-1, y+1],[x, y+1],[x+1, y+1]],

        Then, how do I quickly and efficiently check for neighbors
        */
        let mut occupied_neighbours: u8 = 0;

        // Trying to parallel compute neighbours
        // let i = 0i32..=2;
        // let j = 0i32..2;

        // i.into_par_iter()
        //     .for_each(move |_| j.into_iter()
        //     .for_each(|_| {
        //         let m_y: i32 = y as i32 +(i-1);
        //         let m_x: i32 = x as i32 +(j-1);
                
        //         // Check if out of bounds and skipping this index if it is (would panic otherwise)
        //         if m_y < 0 || m_x < 0 || m_y > SIZE as i32 - 1 || m_x > SIZE as i32 - 1 {
        //             ;
        //         }

        //         if state[(y as i32 +(i-1)) as usize][(x as i32 +(j-1)) as usize] == 1 {
        //             occupied_neighbours += 1;
        //         }
        //     }));

        // Checks the surrounding cells with a for loop
        for i in 0i32..=2 {
            for j in 0i32..=2 {
                let m_y: i32 = y as i32 +(i-1);
                let m_x: i32 = x as i32 +(j-1);
                
                // Check if out of bounds and skipping this index if it is (would panic otherwise)
                if m_y < 0 || m_x < 0 || m_y > SIZE as i32 - 1 || m_x > SIZE as i32 - 1 {
                    continue;
                }

                if state[(y as i32 +(i-1)) as usize][(x as i32 +(j-1)) as usize] == 1 {
                    occupied_neighbours += 1;
                }
            }
        }

        // Working if statement (about on par or faster than above for loop)
        // if y != 0 && x != 0 && state[y-1][x-1] == 1 {
        //     occupied_neighbours += 1;
        // }
        // if y != 0 && state[y-1][x] == 1 {
        //     occupied_neighbours += 1;
        // }
        // if y != 0 && x < SIZE - 1 && state[y-1][x+1] == 1 {
        //     occupied_neighbours += 1;
        // }
        // if x != 0 && state[y][x-1] == 1 {
        //     occupied_neighbours += 1;
        // }
        // if state[y][x] == 1 {
        //     occupied_neighbours += 1;
        // }
        // if x < SIZE - 1 && state[y][x+1] == 1 {
        //     occupied_neighbours += 1;
        // }
        // if y < SIZE - 1 && x != 0 && state[y+1][x-1] == 1 {
        //     occupied_neighbours += 1;
        // }
        // if y < SIZE - 1 && state[y+1][x] == 1 {
        //     occupied_neighbours += 1;
        // }
        // if y < SIZE - 1 && x < SIZE - 1 && state[y+1][x+1] == 1 {
        //     occupied_neighbours += 1;
        // }


        // Subtracts 1 from total cound in case the current cell is occupied (as it is included in the count)
        if state[y][x] == 1 {
            occupied_neighbours -= 1;
        }

        return occupied_neighbours;
    }

    print!("\x1B[2J\x1B[1;1H");

    let mut screen: [[u8; SIZE]; SIZE] = starting_state;
    let mut neighbour_array: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    // "Renders" 2D array from 0 and 1 to '..' and '##'
    for row in screen.iter_mut() {
        for cell in row.iter_mut() {
            print!(" {}", if *cell as u8 == 1 {line_char} else {fill_char});
        }
        print!("\n");
    }

    loop {
        let timer = Instant::now();

        print!("\x1B[1;1H");

        // for y in 0..SIZE {
        //     for x in 0..SIZE {
        //         neighbour_array[y][x] = check_neighbours(&screen, x, y);
        //         // print!(" {}", neighbour_array[y][x]);
        //     }
        //     // println!("");
        // }
        // for y in neighbour_array.iter_mut().enumerate() {
        //     for x in y.1.iter_mut().enumerate() {
        //         *x.1 = check_neighbours(&screen, x.0, y.0);
        //     }
        // }

        neighbour_array.par_iter_mut()
            .enumerate()
            .for_each(|y| y.1.par_iter_mut().enumerate()
            .for_each(|x| *x.1 = check_neighbours(&screen, x.0, y.0)));

        for y in 0..SIZE {
            for x in 0..SIZE {
                let c_state: u8 = screen[y][x];
                let neighbours: u8 = neighbour_array[y][x];

                // Get rid of the else { screen[y][x] = 0 } if u want it to grow like mold
                if c_state == 1 {
                    if neighbours == 2 || neighbours == 3 {
                        screen[y][x] = 1;
                    } else {
                        screen[y][x] = 0;
                    }
                } else if c_state == 0 {
                    if neighbours == 3 {
                        screen[y][x] = 1;
                    } else {
                        screen[y][x] = 0;
                    }
                } else {
                    screen[y][x] = 0;
                }
            }
        }

        
        // "Renders" 2D array from 0 and 1 to '..' and '##'
        for row in screen.iter_mut() {
            for cell in row.iter_mut() {
                print!(" {}", if *cell as u8 == 1 {line_char} else {fill_char});
            }
            print!("\n");
        }

        let duration = timer.elapsed().as_micros();
        println!("calculation duration: {} μs      ", duration);
        println!("     frame hold time: {} ms      ", frame_time);

        thread::sleep(Duration::from_millis(frame_time));
    }
}
