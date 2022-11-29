use image::{self, ImageBuffer, Luma};
use noise::{core::open_simplex::open_simplex_4d, permutationtable::PermutationTable, utils::*};
use rand::prelude::*;
use tqdm::tqdm;

const DEV: u8 = 1;

// #[allow(dead_code)]
// #[cfg(feature = "images")]
pub fn write_to_file(map: &NoiseMap, filename: &str, blur_amt: f32) {
    use std::{fs, path::Path};

    let target_dir: &Path;
    if DEV == 0 {
        target_dir = Path::new("noise_imgs/");
    } else {
        target_dir = Path::new("test_imgs/");
    }

    if !target_dir.exists() {
        fs::create_dir(target_dir).expect("failed to create example_images directory");
    }

    let target = target_dir.join(Path::new(filename));
    let target = target.to_str().unwrap();

    // collect the values from f64 into u8 in a separate vec
    let (width, height) = map.size();
    let mut pixels: Vec<u8> = Vec::with_capacity(width * height);

    for i in tqdm(map.iter()) {
        pixels.push(((i * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8);
    }

    let img_buf: ImageBuffer<Luma<_>, Vec<u8>> =
        match ImageBuffer::from_vec(map.size().0 as u32, map.size().1 as u32, pixels) {
            Some(x) => x,
            None => panic!("Something went wrong creating the ImageBuffer."),
        };

    let img_buf = image::imageops::blur(&img_buf, blur_amt);

    let _ = img_buf.save(&target);

    // let _ = image::save_buffer(
    //     &Path::new(&target),
    //     &*pixels,
    //     map.size().0 as u32,
    //     map.size().1 as u32,
    //     image::ColorType::L8,
    // );
}

fn main() {
    let mut rng = thread_rng();
    let xb = [1600.0, 2000.0, 2400.0];
    let yb = &xb.map(|i| i / 1.777777);
    // let img_size = (1920, 1080);
    let img_size = (4096, 2160);
    let img_blur = 2.0;
    let hasher = PermutationTable::new(rng.gen_range(0..265));

    if DEV == 0 {
        for i in tqdm(0..xb.len()) {
            write_to_file(
                &PlaneMapBuilder::new_fn(open_simplex_4d, &hasher)
                    .set_size(img_size.0, img_size.1)
                    .set_x_bounds(-xb[i], xb[i])
                    .set_y_bounds(-yb[i], yb[i])
                    .build(),
                format!(
                    "open_simplex_4d_withBlur_{}x{}_b{}x{}.png",
                    img_size.0, img_size.1, xb[i], yb[i] as usize,
                )
                .as_str(),
                img_blur,
            );
        }
    } else {
        for i in tqdm(0..xb.len()) {
            write_to_file(
                &PlaneMapBuilder::new_fn(open_simplex_4d, &hasher)
                    .set_size(img_size.0, img_size.1)
                    .set_x_bounds(-xb[i], xb[i])
                    .set_y_bounds(-yb[i], yb[i])
                    .build(),
                format!(
                    "open_simplex_4d_withBlur_{}x{}_b{}x{}_{}.png",
                    img_size.0,
                    img_size.1,
                    xb[i],
                    yb[i] as usize,
                    rng.gen::<f32>()
                )
                .as_str(),
                img_blur,
            );
        }
    }
}
