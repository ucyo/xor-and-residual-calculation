
use clap::{App, load_yaml};
use byteorder::{ByteOrder, LittleEndian};
use std::io::{Read, BufReader};
use std::fs;


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    if matches.is_present("compress") {
        compress(&matches.subcommand_matches("compress").unwrap());
    } else {
        App::from_yaml(yaml).print_help().unwrap();
    }
}

fn compress(matches: &clap::ArgMatches) {
    let ifile = String::from(matches.value_of("input").unwrap());
    let shape = parse_shape(&matches);
    let size = (shape.x * shape.y * shape.z) as usize;
    let cut = 31;

    // read f32 file
    let mut data: Vec<f32> = vec![0f32; size];
    read_f32_data(ifile, size, &mut data);
}

fn parse_shape(matches: &clap::ArgMatches) -> Shape {
    let shape: Vec<i32> = matches.values_of("shape")
           .unwrap()
           .map(|x| String::from(x).parse::<i32>().unwrap_or_else(|e| panic!("Shape: {}", e)))
           .collect();

    Shape {z:shape[0], y:shape[1], x:shape[2]}
}

#[derive(Debug)]
pub struct Shape {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub fn read_f32_data(filename: String, size: usize, data: &mut [f32]) {
    let file = fs::File::open(filename).unwrap();
    let mut bytes: Vec<u8> = Vec::with_capacity(size * 4);
    let s = BufReader::with_capacity(size * 4, file).read_to_end(&mut bytes).unwrap();
    LittleEndian::read_f32_into(&bytes, data);
}
