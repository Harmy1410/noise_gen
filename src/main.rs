use image;
use noise::{
    core::perlin::{perlin_2d, perlin_3d, perlin_4d},
    permutationtable::PermutationTable,
    utils::*,
};

// #[allow(dead_code)]
// #[cfg(feature = "images")]
pub fn write_to_file(map: &NoiseMap, filename: &str) {
    use std::{fs, path::Path};

    let target_dir = Path::new("example_images/");

    if !target_dir.exists() {
        fs::create_dir(target_dir).expect("failed to create example_images directory");
    }

    let target = target_dir.join(Path::new(filename));
    let target = target.to_str().unwrap();

    // collect the values from f64 into u8 in a separate vec
    let (width, height) = map.size();
    let mut pixels: Vec<u8> = Vec::with_capacity(width * height);

    for i in map.iter() {
        pixels.push(((i * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8);
    }

    let _ = image::save_buffer(
        &Path::new(&filename),
        &*pixels,
        map.size().0 as u32,
        map.size().1 as u32,
        image::ColorType::L8,
    );

    println!("\nFinished generating {}", filename);
}

fn main() {
    let hasher = PermutationTable::new(0);
    write_to_file(
        &PlaneMapBuilder::new_fn(perlin_2d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_2d_seed=0.png",
    );
    write_to_file(
        &PlaneMapBuilder::new_fn(perlin_3d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_3d_seed=0.png",
    );
    write_to_file(
        &PlaneMapBuilder::new_fn(perlin_4d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_4d_seed=0.png",
    );

    let hasher = PermutationTable::new(1);
    write_to_file(
        &PlaneMapBuilder::new_fn(perlin_2d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_2d_seed=1.png",
    );
    write_to_file(
        &PlaneMapBuilder::new_fn(perlin_3d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_3d_seed=1.png",
    );
    write_to_file(
        &PlaneMapBuilder::new_fn(perlin_4d, &hasher)
            .set_size(1024, 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build(),
        "perlin_4d_seed=1.png",
    );
}
