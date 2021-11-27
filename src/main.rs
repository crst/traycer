use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::time::{SystemTime};

use png;

mod camera;
mod color;
mod patterns;
mod light;
mod linalg;
mod material;
mod ray;
mod scene;
mod shapes;
mod utils;
mod world;

use clap::Parser;


#[derive(clap::Parser)]
#[clap(version = "1.0", author = "Christopher Strecker <chris@foldl.de>")]
struct Opts {
    #[clap(short, long, default_value = "../scenes/test.json")]
    scene_file: String,

    #[clap(short, long, default_value = "image.png")]
    out_file: String,
}


fn main() {
    let render_start = SystemTime::now();


    // Parse the scene file.
    let opts: Opts = Opts::parse();
    let parsed_scene = scene::parse_scene(&opts.scene_file);

    // Render the scene.
    let camera = scene::make_camera(&parsed_scene);
    let world = scene::make_world(&parsed_scene);
    let image = camera.render(&world);

    // Write the output image.
    let path = Path::new(&opts.out_file);
    let file = File::create(path).unwrap();
    let ref mut outbuffer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(
        outbuffer,
        parsed_scene.camera.width as u32,
        parsed_scene.camera.height as u32
    );
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image.concat().concat()).unwrap();


    let render_duration = render_start.elapsed().unwrap().as_millis();
    println!("Rendered {:?} in {:?} milliseconds to {:?}",
             opts.scene_file, render_duration, opts.out_file);

}
