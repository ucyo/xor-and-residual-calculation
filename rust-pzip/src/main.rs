#![feature(duration_float)]

use clap::{App, load_yaml};
use byteorder::{ByteOrder, LittleEndian};
use std::io::{Read, BufReader};
use std::fs;
use crate::residuals::ResidualTrait;

mod shape;
mod prediction;
mod residuals;
mod encode;


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
    let start = std::time::Instant::now();
    let ifile = String::from(matches.value_of("input").unwrap());
    let shape = shape::parse_shape(&matches);
    let size = (shape.x * shape.y * shape.z) as usize;
    let cut = 31;

    // read f32 file
    let mut data: Vec<f32> = vec![0f32; size];
    let filesize = read_f32_data(ifile, size, &mut data);

    // get predictions
    let predictions = prediction::get_lorenz_predictions(&data, shape);

    // map data to integer
    let data : Vec<u32> = data.iter().map(|&x| x.to_bits()).collect();
    let predictions : Vec<u32> = predictions.iter().map(|&x| x.to_bits()).collect();

    dbg!(data.into_iter().take(10).collect::<Vec<u32>>());
    dbg!(predictions.into_iter().take(10).collect::<Vec<u32>>());
    panic!();

    // calculate residuals
    let residuals = calculate_shifted_residuals(cut, &data, &predictions);

    let fc = encode::encode_bwt_range(&residuals);
    let cr = filesize as f64 / fc.nbytes() as f64;
    let throughput = (size as f64 * 4_f64 /1024_f64/1024_f64) / start.elapsed().as_float_secs();

    println!("{} ratio={:.2} throughput={:.2} MiB/s", fc, cr , throughput);
}


pub fn read_f32_data(filename: String, size: usize, data: &mut [f32]) -> usize {
    let file = fs::File::open(filename).unwrap();
    let mut bytes: Vec<u8> = Vec::with_capacity(size * 4);
    let s = BufReader::with_capacity(size * 4, file).read_to_end(&mut bytes).unwrap();
    LittleEndian::read_f32_into(&bytes, data);
    s
}

pub fn calculate_shifted_residuals(cut: u32, data: &[u32], pred: &[u32]) -> Vec<u32> {
    let mut rctx = residuals::RContext::new(cut);
    let r = residuals::RShifted{};
    let diff : Vec<u32> = data.iter().zip(pred.iter()).map(|(&t,&p)| r.residual(t, p, &mut rctx)).collect();
    diff
}
