
use super::shape::Shape;

type Coordinate  = Shape;

pub fn get_lorenz_predictions(data: &Vec<f32>, shape: Shape) -> Vec<f32> {
    let ptr = data.as_ptr();
    let position = vec![
        Coordinate { x:1, y:0, z:0 },

        Coordinate { x:1, y:1, z:0 },
        Coordinate { x:0, y:1, z:0 },

        Coordinate { x:1, y:0, z:1 },
        Coordinate { x:0, y:0, z:1 },

        Coordinate { x:0, y:1, z:1 },
        Coordinate { x:1, y:1, z:1 },
        ];
    let offsets : Vec<isize> = position.iter().map(|p| calculate_offset(&shape, p) as isize ).collect();

    let mut first_1d : Vec<f32> = data.iter().enumerate().take(shape.x as usize).skip(1).map(|(i,_)| {
        unsafe { *ptr.offset(i as isize - offsets[0]) }
    }).collect();

    let mut first_2d : Vec<f32> = data.iter().enumerate().take(shape.x as usize * shape.y as usize).skip(shape.x as usize).map(|(i,_)| {
        unsafe {
            *ptr.offset(i as isize - offsets[0]) * 1f32 +
            *ptr.offset(i as isize - offsets[1]) * -1f32 +
            *ptr.offset(i as isize - offsets[2]) * 1f32
        }
    }).collect();

    let mut first_3d : Vec<f32> = data.iter().enumerate().skip(shape.x as usize * shape.y as usize).take(shape.x as usize).map(|(i,_)| {
        unsafe {
            *ptr.offset(i as isize - offsets[0]) * 1f32 +
            *ptr.offset(i as isize - offsets[1]) * -1f32 +
            *ptr.offset(i as isize - offsets[2]) * 1f32 +
            *ptr.offset(i as isize - offsets[3]) * -1f32 +
            *ptr.offset(i as isize - offsets[4]) * 1f32
        }
    }).collect();

    let mut remainder : Vec<f32> = data.iter().enumerate().skip(shape.x as usize * shape.y as usize + shape.x as usize).map(|(i,_)| {
        unsafe {
            *ptr.offset(i as isize - offsets[0]) * 1f32 +
            *ptr.offset(i as isize - offsets[1]) * -1f32 +
            *ptr.offset(i as isize - offsets[2]) * 1f32 +
            *ptr.offset(i as isize - offsets[3]) * -1f32 +
            *ptr.offset(i as isize - offsets[4]) * 1f32 +
            *ptr.offset(i as isize - offsets[5]) * -1f32 +
            *ptr.offset(i as isize - offsets[6]) * 1f32
        }
    }).collect();

    first_1d.insert(0, 0f32);
    first_1d.append(&mut first_2d);
    first_1d.append(&mut first_3d);
    first_1d.append(&mut remainder);
    first_1d
}

pub fn calculate_offset(shape: &Coordinate, pos: &Coordinate) -> usize {
    let agg_dims = calculate_dims(shape);
    let result = agg_dims.z * pos.z + agg_dims.y * pos.y + agg_dims.x * pos.x;
    result as usize
}

pub fn calculate_dims(shape: &Coordinate) -> Coordinate {
    let dx = 1;
    let dy = dx * shape.x;
    let dz = dy * shape.y;
    Coordinate {
        x: dx,
        y: dy,
        z: dz,
    }
}
