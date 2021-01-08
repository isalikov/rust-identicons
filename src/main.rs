extern crate termion;

use std::env;
use std::io;
use std::process;

use md5::{Md5, Digest};
use termion::{color};

const USAGE: &str = "show usage help";

fn main() {
    match get_source() {
        Ok(value) => {
            let hash = get_hash(value);
            let matrix = get_matrix(&hash);
            print_matrix((hash[0], hash[1], hash[2]), matrix);

        }

        Err(error) => {
            panic!(error);
        }
    }
}

fn print_matrix((red, green, blue): (u8, u8, u8), matrix: [[u8; 7]; 7]) {
    for row in matrix.iter() {
        for cell in row.iter() {
            if *cell > 0 {
                print!(
                    "{red}██{reset}",
                    red = color::Fg(color::Rgb(red, green, blue)),
                    reset = color::Fg(color::Reset),
                );
            } else {
                print!(
                    "{red}██{reset}",
                    red = color::Fg(color::Blue),
                    reset = color::Fg(color::Reset),
                );
            }
        }

        print!("\n");
    }
}

fn get_matrix(hash: &[u8]) -> [[u8; 7]; 7] {
    let mut matrix = [[0; 7]; 7];

    for index in 0..=14 {
        let (row, col) = get_index(index);

        matrix[row][col] = hash[index] % 2
    }

    for row in 1..=5 {
        for col in 4..=5 {
            if col == 4 {
                matrix[row][col] = matrix[row][2]
            }

            if col == 5 {
                matrix[row][col] = matrix[row][1]
            }
        }
    }

    matrix
}

fn get_index(index: usize) -> (usize, usize) {
    (index / 3 + 1, index % 3 + 1)

}

fn get_hash(source: String) -> [u8; 16] {
    let mut hasher = Md5::new();
    hasher.update(source);

    let result = hasher.finalize();

    let mut bytes = [0; 16];
    bytes.copy_from_slice(&result);


    bytes
}

fn get_source() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{:?}", USAGE);
        process::exit(0);
    }

    let result = &args[1];
    Ok(result.to_string())
}
