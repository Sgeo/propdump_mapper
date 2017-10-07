extern crate image;

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use image::{ImageBuffer, Rgb, GenericImage};

const SCALE: i64 = 10000; // 6550x6550
const RESOLUTION: u32 = 6551;


fn main() {
    let filename = std::env::args().nth(1).expect("No filename provided!");
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_pixel(RESOLUTION, RESOLUTION, Rgb { data: [0, 255, 0] });
    let file = File::open(filename).expect("Unable to open file!");
    let mut file = BufReader::new(&file);
    let mut _header = String::new();
    file.read_line(&mut _header).expect("Unable to discard header!");
    let mut min_image_x: u32 = u32::max_value();
    let mut min_image_y: u32 = u32::max_value();
    let mut max_image_x: u32 = 0;
    let mut max_image_y: u32 = 0;
    let mut min_x: i64 = i64::max_value();
    let mut min_y: i64 = i64::max_value();
    let mut max_x: i64 = i64::min_value();
    let mut max_y: i64 = i64::min_value();
    
    for line in file.split('\n' as u8).map(|line| line.expect("Unable to read a line!")) {
        let line = String::from_utf8_lossy(&line);
        if line.split(' ').count() < 5 {
            continue;
        }
        //println!("{:?}", line);
        
        let (x, y): (i64, i64);
        if let Ok(coords) = line_to_coords(&line) {
            x = coords.0;
            y = coords.1;
        } else {
            continue;
        }
        
        let image_x: u32 = ((-x/SCALE) + 3275) as u32;
        let image_y: u32 = ((-y/SCALE) + 3275) as u32;
        //println!("{}, {}", x, y);
        //println!("{}, {}", image_x, image_y);
        if(image_x < min_image_x) {
            min_image_x = image_x;
        }
        if(image_y < min_image_y) {
            min_image_y = image_y;
        }
        if(max_image_x < image_x) {
            max_image_x = image_x;
        }
        if(max_image_y < image_y) {
            max_image_y = image_y;
        }
        
        if(x < min_x) {
            min_x = x;
        }
        if(y < min_y) {
            min_y = y;
        }
        if(max_x < x) {
            max_x = x;
        }
        if(max_y < y) {
            max_y = y;
        }
        image.put_pixel(image_x, image_y, Rgb { data: [255, 0, 0] });
    }
    println!("Min X: {}, Min Y: {}, Max X: {}, Max Y: {}", min_x, min_y, max_x, max_y);
    println!("Min Image X: {}, Min Image Y: {}, Max Image X: {}, Max Image Y: {}", min_image_x, min_image_y, max_image_x, max_image_y);
    image.save("map.png").expect("Unable to save image!");
}

fn line_to_coords(line: &str) -> Result<(i64, i64), ()> {
    let mut split_line = line.split(' ');
    let (x, y): (i64, i64) = (split_line.nth(2).ok_or(())?.parse().map_err(|_| ())?, split_line.nth(1).ok_or(())?.parse().map_err(|_| ())?);
    return Ok((x, y))
}