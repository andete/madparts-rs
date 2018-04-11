// (c) 2018 Joost Yervante Damad <joost@damad.be>

use cairo;

use error::MpError;

use serde_json;

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
    fn bounding_box(&self) -> Bound;
}

pub trait DrawElement {
    fn draw_element(&self, &cairo::Context);
}

#[derive(Debug)]
pub enum Element {
    Rect(Rect),
    Line(Line),
    Text(Text),
}

#[derive(Debug, Deserialize)]
pub struct Rect {
    pub x:f64,
    pub y:f64,
    pub dx:f64,
    pub dy:f64,
}

#[derive(Debug, Deserialize)]
pub struct Line {
    pub x1:f64,
    pub y1:f64,
    pub x2:f64,
    pub y2:f64,
    pub w:f64,
}

#[derive(Debug, Deserialize)]
pub struct Text {
    pub x:f64,
    pub y:f64,
    pub dy:f64,
    pub txt:String,
}

impl TryFrom<String> for Element {
    type Error = MpError;
    
    fn try_from(json: String) -> Result<Self, Self::Error> {
        let v: serde_json::Value = serde_json::from_str(&json)?;
        match v["t"] {
            serde_json::Value::Null => {
                Err(MpError::Other(format!("missing 't' in Element {}", json)))
            }
            serde_json::Value::String(ref s) => {
                match s.as_str() {
                    "Rect" => {
                        let r:Rect = serde_json::from_str(&json)?;
                        Ok(Element::Rect(r))
                    },
                    "Line" => {
                        let r:Line = serde_json::from_str(&json)?;
                        Ok(Element::Line(r))
                    },
                    "Text" => {
                        let t:Text = serde_json::from_str(&json)?;
                        Ok(Element::Text(t))
                    },
                    x => Err(MpError::Other(format!("Unknown type: {}", x))),
                }
            }
            _ => {
                Err(MpError::Other(format!("unknown 't' in Element {}", json)))
            }
        }
    }
}

impl BoundingBox for Line {
    fn bounding_box(&self) -> Bound {
        let min_x = self.x1.min(self.x2) - self.w/2.0;
        let min_y = self.y1.min(self.y2) - self.w/2.0;
        let max_x = self.x1.max(self.x2) + self.w/2.0;
        let max_y = self.y1.max(self.y2) + self.w/2.0;
        Bound { min_x, min_y, max_x, max_y }
    }
}

impl BoundingBox for Rect {
    fn bounding_box(&self) -> Bound {
        let min_x = self.x - self.dx/2.0;
        let max_x = self.x + self.dx/2.0;
        let min_y = self.y - self.dy/2.0;
        let max_y = self.y + self.dy/2.0;
        Bound { min_x, min_y, max_x, max_y }
    }
}

impl BoundingBox for Text {
    fn bounding_box(&self) -> Bound {
        // TODO: create a dummy context, size doesn't matter
        // but should be big enough to hold the text...
        // https://www.cairographics.org/documentation/cairomm/reference/toy-text_8cc-example.html
        // https://www.cairographics.org/tutorial/#L2textalign

        let img = cairo::ImageSurface::create(cairo::enums::Format::ARgb32, 2000, 100).unwrap();
        let cr = cairo::Context::new(&img);
        
        cr.select_font_face("Sans", cairo::enums::FontSlant::Normal, cairo::enums::FontWeight::Normal);
        cr.set_font_size(self.dy);
        let ext = cr.text_extents(&self.txt);
        let w = ext.width;
        let h = ext.height;
        info!("text size: {} x {}", w, h);
        // TODO!
        let min_x = self.x-w/2.0;
        let max_x = self.x+w/2.0;
        let min_y = self.y-h/2.0;
        let max_y = self.y+h/2.0;
        Bound { min_x, min_y, max_x, max_y }
    }
}

impl BoundingBox for Element {
    fn bounding_box(&self) -> Bound {
        match *self {
            Element::Line(ref l) => l.bounding_box(),
            Element::Rect(ref r) => r.bounding_box(),
            Element::Text(ref t) => t.bounding_box(),
        }
    }
}

impl DrawElement for Line {
    fn draw_element(&self, cr:&cairo::Context) {
        cr.set_line_cap(cairo::enums::LineCap::Round);
        cr.move_to(self.x1,self.y1);
        cr.set_line_width(self.w);
        cr.line_to(self.x2,self.y2);
        cr.stroke();
    }
}

impl DrawElement for Rect {
    fn draw_element(&self, cr:&cairo::Context) {
        cr.rectangle(self.x-self.dx/2.0, self.y-self.dy/2.0, self.dx, self.dy);
        cr.set_source_rgba(1.0, 0.0, 0.0, 0.80);
        cr.fill();
    }
}

impl DrawElement for Text {
    fn draw_element(&self, cr:&cairo::Context) {
        // TODO
        cr.select_font_face("Sans", cairo::enums::FontSlant::Normal, cairo::enums::FontWeight::Normal);
        cr.set_font_size(self.dy);
        let ext = cr.text_extents(&self.txt);
        let w = ext.width;
        let h = ext.height;
        cr.rectangle(self.x-w/2.0, self.y-h/2.0, w, h);
        cr.set_source_rgba(1.0, 0.0, 0.0, 0.80);
        cr.fill();
        cr.move_to(self.x-w/2.0-ext.x_bearing,self.y+h/2.0);
        cr.show_text(&self.txt);
    }
}

impl DrawElement for Element {
    fn draw_element(&self, cr:&cairo::Context) {
        match *self {
            Element::Line(ref l) => l.draw_element(cr),
            Element::Rect(ref r) => r.draw_element(cr),
            Element::Text(ref t) => t.draw_element(cr),
        }
    }
}

pub fn bound(v:&Vec<Element>) -> Bound {
    let mut s = Bound::default();
    for e in v {
        s = s.combine(&e.bounding_box());
    }
    s
}
