use image::{Rgb, RgbImage};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }

    pub fn get_x(&self) -> i32 {
        return self.x;
    }
    pub fn get_y(&self) -> i32 {
        return self.y;
    }
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn distance(&self, p: &Point) -> f64 {
        return f64::sqrt(
            f64::powf(f64::from(p.x) - f64::from(self.x), 2.0)
                + f64::powf(f64::from(p.y) - f64::from(self.y), 2.0),
        );
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


pub fn get_points() -> Vec<Point> {
    let dynimage = image::open("target/points.png").unwrap();
    let img = dynimage.into_rgb8();
    let mut v: Vec<Point> = Vec::new();
    let test = Rgb([0, 0, 0]);
    for x in 0..img.height() {
        for y in 0..img.width() {
            if !img.get_pixel(x, y).eq(&test) {
                v.push(Point::new(x as i32, y as i32));
            }
        }
    }
    return v;
}

pub fn generate_points(height: u32, width: u32) -> Vec<Point> {
    let mut points = Vec::new();
    for x in 0..width {
        for y in 0..height {
            let test = rand::random::<f32>() * 100.0;
            if test < 0.1 {
                points.push(Point::new(x as i32, y as i32));
            }
        }
    }
    let mut rng = thread_rng();
    points.shuffle(&mut rng);
    return points;
}

// pub fn generate_points(height: u32, width: u32) -> Vec<Point> {

//     for x in 0..width {
//         for y in 0..height {
//             let test = rand::random::<f32>() * 100.0;
//             if test < 0.1 {
//                 vec.push(Point::new(x as i32, y as i32));
//             }
//         }
//     }
//     let mut rng = thread_rng();
//     vec.shuffle(&mut rng);
//     points.append(&mut vec);
//     return points;
// }

pub fn closest(point_list: &Vec<Point>, ref_point: &Point) -> Point {
    let mut i = 0;
    let mut closest = point_list[0].clone();
    while &point_list[i] != ref_point && i < point_list.len() {
        if ref_point.distance(&point_list[i]) < ref_point.distance(&closest) {
            closest = point_list[i].clone();
        }
        i += 1;
    }
    return closest;
}


pub fn draw_points(height :u32, width :u32, points :&Vec<Point>) {     
    let mut img = RgbImage::new(width,height);
    for point in points {
        if point.get_x() >= 0 && point.get_x() < width as i32 && point.get_y() >= 0 && point.get_y() < height as i32{
        img.put_pixel(point.x as u32, point.y as u32, Rgb([255,255,255]))
        }
    }
    img.save("target/points.png").unwrap();
}

