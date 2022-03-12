use std::fmt::Display;

use image::{ImageBuffer, Rgb, RgbImage};
use rand;

pub mod point;
use point::Point;
mod line;
use line::Line;

mod tests;
pub struct Vertex {
    color: Rgb<u8>,
    d1: (Line, bool),
    d2: (Line, bool),
    d3: (Line, bool),
    points: [Point; 3],
}

impl Vertex {
    pub fn from_points(p1: &Point, p2: &Point, p3: &Point) -> Vertex {
        let color = Rgb([100, 100, 100]);

        let t1 = Line::get_line_eq(&p1, &p2);
        let t2 = Line::get_line_eq(&p1, &p3);
        let t3 = Line::get_line_eq(&p2, &p3);
        let b1 = t1.is_over(p3);
        let b2 = t2.is_over(p2);
        let b3 = t3.is_over(p1);

        let v = Vertex {
            color: color,
            d1: (t1, b1),
            d2: (t2, b2),
            d3: (t3, b3),
            points: [p1.clone(), p2.clone(), p3.clone()],
        };
        return v;
    }

    fn set_color(&mut self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>){
        let c1 = img.get_pixel(self.points[0].get_x() as u32, self.points[0].get_y() as u32);
        let c2 = img.get_pixel(self.points[1].get_x() as u32, self.points[1].get_y() as u32);
        let c3 = img.get_pixel(self.points[2].get_x() as u32, self.points[2].get_y() as u32);
        self.color = Rgb([
            ((c1.0[0] as u16 + c2.0[0] as u16 + c3.0[0] as u16) / 3) as u8,
            ((c1.0[1] as u16 + c2.0[1] as u16 + c3.0[1] as u16) / 3) as u8,
            ((c1.0[2] as u16 + c2.0[2] as u16 + c3.0[2] as u16) / 3) as u8,
        ]);
        // self.color = Rgb([
        //     rand::random::<u8>(),
        //     rand::random::<u8>(),
        //     rand::random::<u8>(),
        // ]);
    }

    fn is_creator(&self, p: &Point) -> bool {
        for point in &self.points {
            if point == p {
                return true;
            }
        }
        return false;
    }

    fn is_inside(&self, p: &Point) -> bool {
        return self.d1.0.is_over(&p) == self.d1.1
            && self.d2.0.is_over(&p) == self.d2.1
            && self.d3.0.is_over(&p) == self.d3.1;
    }

    fn draw(&self) {
        let mut img = RgbImage::new(100, 100);
        for x in 0..img.height() {
            for y in 0..img.width() {
                let p = Point::new(x as i32, y as i32);
                if self.is_inside(&p) {
                    img.put_pixel(x, y, Rgb([255, 255, 255]));
                } else {
                    img.put_pixel(x, y, Rgb([0, 0, 0]));
                }
            }
        }
        img.save("target/vertex.png").unwrap();
    }

    fn draw_borders(&self) {
        let mut img = RgbImage::new(100, 100);
        for x in 0..img.height() {
            for y in 0..img.width() {
                let p = Point::new(x as i32, y as i32);
                if (self.d1.0.is_on(&p) || self.d2.0.is_on(&p) || self.d3.0.is_on(&p))
                    && self.is_inside(&p)
                {
                    img.put_pixel(x, y, Rgb([255, 255, 255]));
                } else {
                    img.put_pixel(x, y, Rgb([0, 0, 0]));
                }
            }
        }
        img.save("target/vertex.png").unwrap();
    }

    fn is_on_border(&self, p: &Point) -> bool {
        if (self.d1.0.is_on(&p) || self.d2.0.is_on(&p) || self.d3.0.is_on(&p)) && self.is_inside(&p)
        {
            return true;
        }
        return false;
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Vertex [({},{}), ({},{}), ({},{})]",
            self.d1.0, self.d1.1, self.d2.0, self.d2.1, self.d3.0, self.d3.1
        )
    }
}

impl Clone for Vertex {
    fn clone(&self) -> Self {
        return Vertex {
            color: (self.color),
            d1: (self.d1),
            d2: (self.d2),
            d3: (self.d3),
            points: (self.points),
        };
    }
}

pub fn parse_to_vertex(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, points: &Vec<Point>) -> Vec<Vertex> {
    let mut verticies: Vec<Vertex> = Vec::new();

    let mut a_points = Vec::new();

    let p1 = Point::new(-1, -1);
    let p2 = Point::new(-1, 3000);
    let p3 = Point::new(3000, -1);

    a_points.push(p1.clone());
    a_points.push(p2.clone());
    a_points.push(p3.clone());

    a_points.append(&mut points.clone());

    verticies.push(Vertex::from_points(
        &a_points[0],
        &a_points[1],
        &a_points[2],
    ));
    for i in 3..a_points.len() {
        for j in 0..verticies.len() {
            if verticies[j].is_inside(&a_points[i]) {
                verticies.push(Vertex::from_points(
                    &a_points[i],
                    &verticies[j].points[0],
                    &verticies[j].points[1],
                ));
                verticies.push(Vertex::from_points(
                    &a_points[i],
                    &verticies[j].points[0],
                    &verticies[j].points[2],
                ));
                verticies.push(Vertex::from_points(
                    &a_points[i],
                    &verticies[j].points[1],
                    &verticies[j].points[2],
                ));
                verticies.swap_remove(j);
                break;
            }
            println!("part 2");
        }
    }
    let mut indexes_remove = Vec::new();
    for i in 0..verticies.len() {
        if verticies[i].is_creator(&p1)
            || verticies[i].is_creator(&p2)
            || verticies[i].is_creator(&p3)
        {
            indexes_remove.push(i);
        }
    }

    let mut temp = Vec::new();
    for i in 0..verticies.len() {
        if !indexes_remove.contains(&i) {
            temp.push(verticies[i].clone());
        }
    }

    for i in 0..temp.len() {
        temp[i].set_color(img);
    }

    return temp;
}

fn get_vertex_color(verticies: &Vec<Vertex>, p: &Point) -> Rgb<u8> {
    for vertex in verticies {
        if vertex.is_inside(p) {
            return vertex.color;
        }
    }
    return Rgb([0, 0, 0]);
}

pub fn draw_verticies(height: u32, width: u32, verticies: &Vec<Vertex>) {
    for vertex in verticies {
        println!("{}", vertex);
    }
    let mut img = RgbImage::new(width, height);
    for y in 0..img.height() {
        for x in 0..img.width() {
            let p = Point::new(x as i32, y as i32);
            img.put_pixel(x, y, get_vertex_color(verticies, &p));
        }
    }
    img.save("target/vertex.png").unwrap();
}

// pub fn draw_borders(verticies :&Vec<Vertex>) {
//     println!("{} vertexs", verticies.len());
//     let mut img = RgbImage::new(100, 100);
//     for x in 0..img.height() {
//         for y in 0..img.width() {
//             let p = Point::new(x, y);
//             if self.on_any_border(&p) {
//                 img.put_pixel(x, y, Rgb([255, 255, 255]));
//             } else {
//                 img.put_pixel(x, y, Rgb([0, 0, 0]));
//             }
//         }
//     }
//     img.save("target/vertex.png").unwrap();
// }
