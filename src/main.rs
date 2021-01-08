extern crate termion;
extern crate image;

use std::env;
use std::io;
use std::process;
use std::convert::TryFrom;

use image::{ImageBuffer, RgbImage};
use md5::{Md5, Digest};
use termion::{color};

const USAGE: &str = "Usage error: \n Set string argument! \n\nidenticon test_string";

fn main() {
    match get_source() {
        Ok(value) => {
            let hash = get_hash(&value);
            let matrix = get_matrix(&hash);

            print_matrix((hash[0], hash[1], hash[2]), matrix);
            save_matrix((hash[0], hash[1], hash[2]), matrix, &value);
        }

        Err(error) => {
            panic!(error);
        }
    }
}

fn save_matrix((red, green, blue): (u8, u8, u8), matrix: [[u8; 7]; 7], name: &String) {
    let mut img: RgbImage = ImageBuffer::new(350, 350);
    let path = "./".to_owned() + name as &str + ".png";

    let background = image::Rgb([250, 250, 250]);
    let color = image::Rgb([red, green, blue]);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if check_index((x, y), matrix) {
            *pixel = color;
        } else {
            *pixel = background;
        }
    }

    img.save(path).unwrap();
}

fn print_matrix((red, green, blue): (u8, u8, u8), matrix: [[u8; 7]; 7]) {
    for row in matrix.iter() {
        for cell in row.iter() {
            if *cell > 0 {
                print!(
                    "{color}██{reset}",
                    color = color::Fg(color::Rgb(red, green, blue)),
                    reset = color::Fg(color::Reset),
                );
            } else {
                print!(
                    "{color}██{reset}",
                    color = color::Fg(color::Rgb(250, 250, 250)),
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

fn check_index((x, y): (u32, u32), matrix: [[u8; 7]; 7]) -> bool {
    let dx = usize::try_from(x / 50).unwrap();
    let dy = usize::try_from(y / 50).unwrap();

    return matrix[dy][dx] == 1
}

fn get_hash(source: &String) -> [u8; 16] {
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
