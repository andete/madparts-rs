// (c) 2018 Joost Yervante Damad <joost@damad.be>

use error::MpError;
use pyo3::{PyDict, ObjectProtocol};

#[derive(Debug)]
pub enum Element {
    Rect,
    Line,
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
            "rect" => Ok(Element::Rect),
            "line" => Ok(Element::Line),
            x => Err(MpError::Other(format!("Unknown type: {}", x))),
        }
    }
}
