use super::point::Point;

#[derive(Clone, Copy)]
pub struct Line(f64, f64);

impl Line {
    pub fn get_line_eq(p1: &Point, p2: &Point) -> Line {

        let slope = ((f64::from(p2.get_y()) - f64::from(p1.get_y())))
            / (f64::from(p2.get_x()) - f64::from(p1.get_x()));
        println!("p1 {}, p2 {} -> slope : {}", p1, p2, slope);

        if slope.is_nan() {
            println!("wtf p1 {}  p2 {}", p1, p2)
        }
        if slope == f64::INFINITY || slope == f64::NEG_INFINITY {
            let intercept = f64::from(p1.get_x());
            return Line(slope, intercept);
        }
        let intercept = f64::from(p1.get_y()) - slope * f64::from(p1.get_x());
        return Line(slope, intercept);
    }

    pub fn is_over(&self, p: &Point) -> bool {
        if self.0 == f64::INFINITY {
            if f64::from(p.get_x()) <= self.1 {
                return true;
            }
            return false;
        }
        if self.0 == f64::NEG_INFINITY {
            if f64::from(p.get_x()) >= self.1 {
                return true;
            }
            return false;
        }

        if f64::from(p.get_x()) * self.0 + self.1 <= f64::from(p.get_y()) {
            return true;
        }
        return false;
    }
    
    pub fn is_on(&self, p: &Point) -> bool {
        let result = f64::from(p.get_x()) * self.0 + self.1 - f64::from(p.get_y());
        if result < 1.0 && result > -1.0 {
            return true;
        }
        return false;
    }
}
impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}x +{}", self.0, self.1)
    }
}


#[cfg(test)]
mod tests {
    use crate::vertex::{line::{Line}, self};
    #[test]
    fn test_is_over() {
        let p1 = vertex::point::Point::new(0,0);
        let p2 = vertex::point::Point::new(0,-1);
        let p3 = vertex::point::Point::new(1,0);
        let p4 = vertex::point::Point::new(-1, 0);

        let l1 = Line(0.0, 0.0);
        assert!(l1.is_over(&p1));
        assert!(!l1.is_over(&p2));
        assert!(l1.is_over(&p3));
        assert!(l1.is_over(&p4));

        let l2 = Line(1.0, 0.0);
        assert!(l2.is_over(&p1));
        assert!(!l2.is_over(&p2));
        assert!(!l2.is_over(&p3));
        assert!(l2.is_over(&p4));
        
        let l2 = Line(-1.0, 0.0);
        assert!(l2.is_over(&p1));
        assert!(!l2.is_over(&p2));
        assert!(l2.is_over(&p3));
        assert!(!l2.is_over(&p4));

        let l3 = Line(f64::INFINITY, 0.0);
        assert!(l3.is_over(&p1));
        assert!(l3.is_over(&p2));
        assert!(!l3.is_over(&p3));
        assert!(l3.is_over(&p4));

        let l3 = Line(f64::NEG_INFINITY, 0.0);
        assert!(l3.is_over(&p1));
        assert!(l3.is_over(&p2));
        assert!(l3.is_over(&p3));
        assert!(!l3.is_over(&p4));
    }
}