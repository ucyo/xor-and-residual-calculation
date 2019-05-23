
#[derive(Debug)]
pub struct Shape {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub fn parse_shape(matches: &clap::ArgMatches) -> Shape {
    let shape: Vec<i32> = matches.values_of("shape")
           .unwrap()
           .map(|x| String::from(x).parse::<i32>().unwrap_or_else(|e| panic!("Shape: {}", e)))
           .collect();

    Shape {z:shape[0], y:shape[1], x:shape[2]}
}
