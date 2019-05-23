
use clap::{App, load_yaml};
use byteorder::{ByteOrder, LittleEndian};
use std::io::{Read, BufReader};
use std::fs;

mod shape;
mod prediction;


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
    let shape = shape::parse_shape(&matches);
    let size = (shape.x * shape.y * shape.z) as usize;

    // read f32 file
    let mut data: Vec<f32> = vec![0f32; size];
    read_f32_data(ifile, size, &mut data);

    // get predictions
    let predictions = prediction::get_lorenz_predictions(&data, shape);

    // map data to integer
    let data : Vec<u32> = data.iter().map(|&x| x.to_bits()).collect();
    let predictions : Vec<u32> = predictions.iter().map(|&x| x.to_bits()).collect();
}


pub fn read_f32_data(filename: String, size: usize, data: &mut [f32]) {
    let file = fs::File::open(filename).unwrap();
    let mut bytes: Vec<u8> = Vec::with_capacity(size * 4);
    BufReader::with_capacity(size * 4, file).read_to_end(&mut bytes).unwrap();
    LittleEndian::read_f32_into(&bytes, data);
}
