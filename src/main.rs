use image::{Rgb, RgbImage, ImageBuffer};
use rand;
use rand::seq::SliceRandom;
use rand::thread_rng;


struct Point (u32, u32);

fn get_points() -> Vec<Point> {
    let dynimage = image::open("target/result.png").unwrap();
    let img = dynimage.into_rgb8();
    let mut v : Vec<Point> = Vec::new();
    let test = Rgb([0,0,0]);
    for x in 0..img.height() {
        for y in 0..img.width(){
            if !img.get_pixel(x, y).eq(&test)
            {
                v.push(Point(x,y));
            }
        }
    }
    return v;
}
struct Line (f64, f64);
impl Line {
    fn get_line_eq(p1 :&Point, p2 :&Point) -> Line {
        let slope = (f64::from(p2.1) - f64::from(p1.1))/(f64::from(p2.0) - f64::from(p1.0));
        let intercept = f64::from(p1.1) - slope * f64::from(p1.0);
        // println!("slope : {}", slope);
        return Line(slope, intercept);
    }
    fn is_over(&self, p :&Point) -> bool {
        if f64::from(p.0) * self.0 + self.1 <= f64::from(p.1) {
            return true;
        }
        return false;
    }
    fn is_on(&self, p :&Point) -> bool {
        let result = f64::from(p.0) * self.0 + self.1 - f64::from(p.1);
        if  result < 1.0 && result > -1.0{
            return true;
        }
        return false;
    }
}
struct Vertex {
    color :Rgb<u8>,
    d1 :(Line, bool),
    d2 :(Line, bool),
    d3 :(Line, bool),
    points :[Point; 3]
}

impl Vertex {
    fn from_points(p1 :&Point, p2 :&Point, p3 :&Point, color :Rgb<u8>) -> Vertex {
        let t1 = Line::get_line_eq(&p1, &p2);
        let t2 = Line::get_line_eq(&p1, &p3);
        let t3 = Line::get_line_eq(&p2, &p3);
        let b1 = t1.is_over(p3);
        let b2 = t2.is_over(p2);
        let b3 = t3.is_over(p1);

        let v  = Vertex{
            color : color,
            d1 : (t1, b1), 
            d2 : (t2, b2), 
            d3 : (t3, b3),
            points : [Point(p1.0, p1.1), Point(p2.0, p2.1), Point(p3.0, p3.1)]
        };
        return v;
    }

    fn is_inside(&self, p :&Point) -> bool{
        return self.d1.0.is_over(&p) == self.d1.1 && self.d2.0.is_over(&p) == self.d2.1 && !self.d3.0.is_over(&p) == self.d2.1;
    }

    fn draw(&self) {
        let mut img = RgbImage::new(100,100);
        for x in 0..img.height() {
            for y in 0..img.width(){
                let p = Point(x,y);
                if self.is_inside(&p)
                {
                    img.put_pixel(x, y, Rgb([255,255,255]));
    
                }
                else
                {
                    img.put_pixel(x, y, Rgb([0,0,0]));
    
                }
                
            }
        }
        img.save("target/vertex.png").unwrap();
    }

    fn draw_borders(&self) {
        let mut img = RgbImage::new(100,100);
        for x in 0..img.height() {
            for y in 0..img.width(){
                let p = Point(x,y);
                if (self.d1.0.is_on(&p) || self.d2.0.is_on(&p) || self.d3.0.is_on(&p)) && self.is_inside(&p)
                {
                    img.put_pixel(x, y, Rgb([255,255,255]));
    
                }
                else
                {
                    img.put_pixel(x, y, Rgb([0,0,0]));
    
                }
                
            }
        }
        img.save("target/vertex.png").unwrap();
    }

    fn is_on_border(&self, p :&Point) -> bool{
        if (self.d1.0.is_on(&p) || self.d2.0.is_on(&p) || self.d3.0.is_on(&p)) && self.is_inside(&p) {
            return true;
        }
        return false;
    }
}



// struct Filter {
//     vertexes : Vec<Vertex>
// }

// impl Filter {







//     fn draw_borders(&self) {
//         println!("{} vertexs", self.vertexes.len());
//         let mut img = RgbImage::new(100,100);
//         for x in 0..img.height() {
//             for y in 0..img.width(){
//                 let p = Point(x,y);
//                 if self.on_any_border(&p)
//                 {
//                     img.put_pixel(x, y, Rgb([255,255,255]));
    
//                 }
//                 else
//                 {
//                     img.put_pixel(x, y, Rgb([0,0,0]));
    
//                 }
                
//             }
//         }
//         img.save("target/vertex.png").unwrap();
//     }


// }









fn generate_points(height :u32, width :u32) -> Vec<Point> {
    let mut vec = Vec::new();
    vec.push(Point(1,1));
    vec.push(Point(0, height - 1));
    vec.push(Point(width - 1, 0));
    for x in 0..width {
        for y in 0..height{
            let test = rand::random::<f32>() * 100.0;
            if test < 0.03
            {
                vec.push(Point(x,y));
            }
        }
    }
    let mut rng = thread_rng();
    vec.shuffle(&mut rng);
    return vec;
}

fn draw_points(height :u32, width :u32, points :&Vec<Point>) {     
    let mut img = RgbImage::new(width,height);
    for point in points {
        img.put_pixel(point.0, point.1, Rgb([255,255,255]))
    }
    img.save("target/result.png").unwrap();
}

fn get_color(img :&ImageBuffer<Rgb<u8>, Vec<u8>>) -> Rgb<u8> {
    return Rgb([rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()])
}

fn parse_to_vertex(img :&ImageBuffer<Rgb<u8>, Vec<u8>>,points :&Vec<Point>) -> Vec<Vertex> {
    let mut verticies:Vec<Vertex> = Vec::new();

    if points.len() < 3 {
        return verticies;
    }
    verticies.push(Vertex::from_points(&points[0], &points[1], &points[2], get_color(img)));
    for i in 3..points.len() {
        for j in 0..verticies.len() {
            if verticies[j].is_inside(&points[i]) {
                verticies.push(Vertex::from_points(&points[i], &verticies[j].points[0], &verticies[j].points[1], get_color(img)));
                verticies.push(Vertex::from_points(&points[i], &verticies[j].points[0], &verticies[j].points[2], get_color(img)));
                verticies.push(Vertex::from_points(&points[i], &verticies[j].points[1], &verticies[j].points[2], get_color(img)));
                verticies.swap_remove(j);
            }
        }
    }
    return verticies;
}

fn get_vertex_color(verticies :&Vec<Vertex>, p :&Point) -> Rgb<u8> {
    for vertex in verticies {
        if vertex.is_inside(p) {
            return vertex.color;
        }
    }
    return Rgb([0,0,0]);
}

fn draw_verticies(height :u32, width :u32, verticies :&Vec<Vertex>) {
    let mut img = RgbImage::new(width,height);
    for y in 0..img.height() {
        for x in 0..img.width(){
            let p = Point(x,y);
            img.put_pixel(x, y, get_vertex_color(verticies, &p));      
        }
    }
    img.save("target/vertex.png").unwrap();
}


fn main() {
    // // let v = get_points();
    // let v = vec![Point(20,10),Point(80,5),Point(5,80),Point(25,25)];
    // // for x in &v {
    // //     println!("{}  {}", x.0, x.1);
    // // }
    // // println!("{}", Line::get_line_eq(&v[0], &v[1]).1);
    // let vertex = Vertex::from_points(&v[0], &v[1], &v[2]);
    // vertex.draw_borders();
    // let f = Filter::parse_to_vertex(&v);
    // f.draw_borders();

    let dynimage = image::open("target/input.png").unwrap();
    let img = dynimage.into_rgb8();
    let points = generate_points(img.height(), img.width());
    draw_points(img.height(), img.width(), &points);

    let verticies = parse_to_vertex(&img, &points);
    draw_verticies(img.height(), img.width(), &verticies)
}