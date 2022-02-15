use image::{Rgb, RgbImage};
use rand;


fn generate_img()
{     
    let mut img = RgbImage::new(100,100);
    for x in 0..img.height() {
        for y in 0..img.width(){
            let test = rand::random::<f32>() * 100.0;
            println!("{}",test);
            if test < 0.03
            {
                img.put_pixel(x, y, Rgb([255,255,255]));

            }
            else
            {
                img.put_pixel(x, y, Rgb([0,0,0]));

            }
            
        }
    }
    img.save("target/result.png").unwrap();
}


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
        return Line(slope, intercept);
    }
    fn is_over(&self, p :Point) -> bool {
        if f64::from(p.0) * self.0 + self.1 < f64::from(p.1) {
            return true;
        }
        return false;
    }
}
struct Vertex {
    d1 :Line,
    d2 :Line,
    d3 :Line
}

impl Vertex {
    fn from_points(p1 :&Point, p2 :&Point, p3 :&Point) -> Vertex {
        let v  = Vertex{
            d1 : Line::get_line_eq(&p1, &p2), 
            d2 : Line::get_line_eq(&p2, &p3), 
            d3 : Line::get_line_eq(&p1, &p3)
        };
        return v;
    }

    fn draw(&self) {
        let mut img = RgbImage::new(100,100);
        for x in 0..img.height() {
            for y in 0..img.width(){
                if self.d1.is_over(Point(x,y)) && self.d2.is_over(Point(x,y)) && !self.d3.is_over(Point(x,y))
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
}

fn main() {
    let v = get_points();
    for x in &v {
        println!("{}  {}", x.0, x.1);
    }
    println!("{}", Line::get_line_eq(&v[0], &v[1]).1);
    let v = Vertex::from_points(&v[0], &v[1], &v[2]);
    v.draw();

}
