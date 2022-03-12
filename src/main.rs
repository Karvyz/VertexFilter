use image::{ImageBuffer};

mod vertex;

fn main() {
    let dynimage = image::open("target/input.png").unwrap();
    let img = dynimage.into_rgb8();

    // let img = ImageBuffer::new(200,200);

    let points = vertex::point::generate_points(img.height(), img.width());
    vertex::point::draw_points(img.height(), img.width(), &points);

    // let points = vertex::point::get_points();
    for point in &points {
        println!("{}",point);
    }

    let verticies = vertex::parse_to_vertex(&img, &points);
    println!("nverticies : {}", &verticies.len());
    vertex::draw_verticies(img.height(), img.width(), &verticies)
}