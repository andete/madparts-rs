// (c) 2018 Joost Yervante Damad <joost@damad.be>

use error::MpError;
use pyo3::{PyDict, ObjectProtocol};

#[derive(Debug,Default)]
pub struct Bound {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl Bound {
    fn combine(&self, b:&Bound) -> Bound {
        Bound {
            min_x:self.min_x.min(b.min_x),
            min_y:self.min_y.min(b.min_y),
            max_x:self.max_x.max(b.max_x),
            max_y:self.max_y.max(b.max_y),
        }
    }
}

trait BoundingBox {
    fn bounding_box(&Self) -> Bound;
}

#[derive(Debug)]
pub enum Element {
    Rect(Rect),
    Line(Line),
}

#[derive(Debug)]
pub struct Rect {
    pub x:f64,
    pub y:f64,
    pub dx:f64,
    pub dy:f64,
}

#[derive(Debug)]
pub struct Line {
    pub x1:f64,
    pub y1:f64,
    pub x2:f64,
    pub y2:f64,
    pub w:f64,
}

impl<'a> TryFrom<&'a PyDict> for Element {
    type Error = MpError;
    
    fn try_from(dict: &'a PyDict) -> Result<Self, Self::Error> {
        let t:String = dict.get_item("t").unwrap().extract()?; // TODO
        match t.as_str() {
            "rect" => Ok(Element::Rect(Rect::try_from(dict)?)),
            "line" => Ok(Element::Line(Line::try_from(dict)?)),
            x => Err(MpError::Other(format!("Unknown type: {}", x))),
        }
    }
}


impl<'a> TryFrom<&'a PyDict> for Rect {
    type Error = MpError;
    
    fn try_from(dict: &'a PyDict) -> Result<Self, Self::Error> {
        let x:f64 = dict.get_item("x").unwrap().extract()?; // TODO
        let y:f64 = dict.get_item("y").unwrap().extract()?; // TODO
        let dx:f64 = dict.get_item("dx").unwrap().extract()?; // TODO
        let dy:f64 = dict.get_item("dy").unwrap().extract()?; // TODO
        Ok(Rect { x, y, dx, dy })
    }
}

impl<'a> TryFrom<&'a PyDict> for Line {
    type Error = MpError;
    
    fn try_from(dict: &'a PyDict) -> Result<Self, Self::Error> {
        let x1:f64 = dict.get_item("x1").unwrap().extract()?; // TODO
        let y1:f64 = dict.get_item("y1").unwrap().extract()?; // TODO
        let x2:f64 = dict.get_item("x2").unwrap().extract()?; // TODO
        let y2:f64 = dict.get_item("y2").unwrap().extract()?; // TODO
        let w:f64 = dict.get_item("w").unwrap().extract()?; // TODO
        Ok(Line { x1, y1, x2, y2, w })
    }
}

impl BoundingBox for Line {
    fn bounding_box(line:&Line) -> Bound {
        let min_x = line.x1.min(line.x2) - line.w/2.0;
        let min_y = line.y1.min(line.y2) - line.w/2.0;
        let max_x = line.x1.max(line.x2) + line.w/2.0;
        let max_y = line.y1.max(line.y2) + line.w/2.0;
        Bound { min_x, min_y, max_x, max_y }
    }
}

impl BoundingBox for Rect {
    fn bounding_box(rect:&Rect) -> Bound {
        let min_x = rect.x - rect.dx/2.0;
        let max_x = rect.x + rect.dx/2.0;
        let min_y = rect.y - rect.dy/2.0;
        let max_y = rect.y + rect.dy/2.0;
        Bound { min_x, min_y, max_x, max_y }
    }
}

impl BoundingBox for Element {
    fn bounding_box(e:&Element) -> Bound {
        match *e {
            Element::Line(ref l) => BoundingBox::bounding_box(l),
            Element::Rect(ref r) => BoundingBox::bounding_box(r),
        }
    }
}

pub fn bound(v:&Vec<Element>) -> Bound {
    let mut s = Bound::default();
    for e in v {
        s = s.combine(&BoundingBox::bounding_box(e));
    }
    s
}
