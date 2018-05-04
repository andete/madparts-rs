// (c) 2018 Joost Yervante Damad <joost@damad.be>

use cairo;

use error::MpError;

use serde_json;

use settings::{Layer, LAYER};

use std::convert::TryFrom;

use kicad;

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
    fn draw_element(&self, &cairo::Context, layer:Layer);
}

pub trait ApplyFootprint {
    fn apply_footprint(&self, &mut kicad::Footprint);
}

#[derive(Debug, Clone)]
pub enum Element {
    Rect(Rect),
    Line(Line),
    Name(Name),
    Reference(Reference),
    Smd(Smd),
    Pad(Pad),
    PythonError(PythonError),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rect {
    pub x:f64,
    pub y:f64,
    pub dx:f64,
    pub dy:f64,
    pub w:f64,
    pub filled:bool,
    pub layer:Layer,
    pub corner:Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Line {
    pub x1:f64,
    pub y1:f64,
    pub x2:f64,
    pub y2:f64,
    pub w:f64,
    pub layer:Layer,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Text {
    pub x:f64,
    pub y:f64,
    pub dy:f64,
    pub txt:String,
    pub thickness:f64,
    pub shorten:Option<bool>,
    pub layer:Layer,
}


impl Text{
    pub fn shortened_text(&self) -> String {
        let text = if self.shorten.is_some() {
            let mut text = self.txt.clone();
            text.truncate(4);
            text.push_str("...");
            text
        } else {
            self.txt.clone()
        };
        text
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Smd {
    pub name:String,
    pub x:f64,
    pub y:f64,
    pub dx:f64,
    pub dy:f64,
    pub layers:Vec<Layer>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Pad {
    pub name:String,
    pub x:f64,
    pub y:f64,
    pub dx:f64,
    pub dy:f64,
    pub drill: f64,
    pub layers:Vec<Layer>,
}

#[derive(Debug, Clone)]
pub struct Name {
    pub text:Text,
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub text:Text,
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
                    "Rect" | "FFab" | "FCrtYd" | "FPaste" | "FMask" => {
                        let r:Rect = serde_json::from_str(&json)?;
                        Ok(Element::Rect(r))
                    },
                    "Line" => {
                        let r:Line = serde_json::from_str(&json)?;
                        Ok(Element::Line(r))
                    },
                    "Name" => {
                        let mut text:Text = serde_json::from_str(&json)?;
                        text.shorten = Some(true);
                        Ok(Element::Name(Name { text }))
                    },
                    "Reference" => {
                        let text:Text = serde_json::from_str(&json)?;
                        Ok(Element::Reference(Reference { text }))
                    },
                    "Pad" => {
                        let r:Pad = serde_json::from_str(&json)?;
                        Ok(Element::Pad(r))
                    },
                    "Smd" => {
                        let r:Smd = serde_json::from_str(&json)?;
                        Ok(Element::Smd(r))
                    },
                    "PythonError" => {
                        let r:PythonError = serde_json::from_str(&json)?;
                        Ok(Element::PythonError(r))
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
        let min_x = self.x - self.dx/2.0 - self.w/2.0;
        let max_x = self.x + self.dx/2.0 + self.w/2.0;
        let min_y = self.y - self.dy/2.0 - self.w/2.0;
        let max_y = self.y + self.dy/2.0 + self.w/2.0;
        Bound { min_x, min_y, max_x, max_y }
    }
}

impl BoundingBox for Smd {
    fn bounding_box(&self) -> Bound {
        let min_x = self.x - self.dx/2.0;
        let max_x = self.x + self.dx/2.0;
        let min_y = self.y - self.dy/2.0;
        let max_y = self.y + self.dy/2.0;
        Bound { min_x, min_y, max_x, max_y }
    }
}

impl BoundingBox for Pad {
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
        // create a dummy cairo Context to be able to calculate the
        // text size
        let img = cairo::ImageSurface::create(cairo::enums::Format::ARgb32, 2000, 100).unwrap();
        let cr = cairo::Context::new(&img);

        let text = self.shortened_text();
        
        cr.select_font_face("Sans", cairo::enums::FontSlant::Normal, cairo::enums::FontWeight::Normal);
        cr.set_font_size(self.dy);
        let ext = cr.text_extents(&text);
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
            Element::Name(ref t) => t.text.bounding_box(),
            Element::Reference(ref t) => t.text.bounding_box(),
            Element::Smd(ref r) => r.bounding_box(),
            Element::Pad(ref r) => r.bounding_box(),
            Element::PythonError(_) => unreachable!(),
        }
    }
}

impl DrawElement for Line {
    fn draw_element(&self, cr:&cairo::Context, layer:Layer) {
        if layer == self.layer {
            LAYER[&layer].color.set_source(cr);
            cr.set_line_width(self.w);
            cr.set_line_cap(cairo::enums::LineCap::Round);
            cr.move_to(self.x1,self.y1);
            cr.line_to(self.x2,self.y2);
            cr.stroke();
        }
    }
}

impl DrawElement for Rect {
    fn draw_element(&self, cr:&cairo::Context, layer:Layer) {
        if layer == self.layer {
            LAYER[&layer].color.set_source(cr);
            if self.filled {
                cr.rectangle(self.x-self.dx/2.0, self.y-self.dy/2.0, self.dx, self.dy);
                cr.fill();
            } else {
                cr.set_line_width(self.w);
                cr.set_line_join(cairo::enums::LineJoin::Round);
                if let Some(corner) = self.corner {
                    cr.move_to(self.x-self.dx/2.0,self.y-self.dy/2.0+corner);
                    cr.line_to(self.x-self.dx/2.0+corner,self.y-self.dy/2.0);
                } else {
                    cr.move_to(self.x-self.dx/2.0,self.y-self.dy/2.0);
                }
                cr.line_to(self.x+self.dx/2.0,self.y-self.dy/2.0);
                cr.line_to(self.x+self.dx/2.0,self.y+self.dy/2.0);
                cr.line_to(self.x-self.dx/2.0,self.y+self.dy/2.0);
                cr.close_path();
                cr.stroke();
            }
        }
    }
}

impl DrawElement for Smd {
    fn draw_element(&self, cr:&cairo::Context, layer:Layer) {
        if layer == Layer::FCu {
            LAYER[&layer].color.set_source(cr);
            cr.rectangle(self.x-self.dx/2.0, self.y-self.dy/2.0, self.dx, self.dy);
            cr.fill();
            cr.select_font_face("Sans", cairo::enums::FontSlant::Normal, cairo::enums::FontWeight::Normal);
            let l = self.name.len() as f64;
            cr.set_font_size((self.dx/l).min(self.dy)*0.9);
            cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
            let ext = cr.text_extents(&self.name);
            let w = ext.width;
            let h = ext.height;
            cr.move_to(self.x-w/2.0-ext.x_bearing, self.y+h/2.0);
            cr.show_text(&self.name);
        }
    }
}

impl DrawElement for Pad {
    fn draw_element(&self, cr:&cairo::Context, layer:Layer) {
        if layer == Layer::FCu {
            LAYER[&layer].color.set_source(cr);
            // TODO: other types then circle
            cr.stroke();
            cr.set_line_width(0.0);
            cr.arc(self.x, self.y, self.dx/2.0, 0.0, 360.0);
            cr.fill();
            cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
            cr.arc(self.x, self.y, self.drill/2.0, 0.0, 360.0);
            cr.fill();
            cr.select_font_face("Sans", cairo::enums::FontSlant::Normal, cairo::enums::FontWeight::Normal);
            let l = self.name.len() as f64;
            cr.set_font_size((self.dx/l).min(self.dy)*0.9);
            cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
            let ext = cr.text_extents(&self.name);
            let w = ext.width;
            let h = ext.height;
            cr.move_to(self.x-w/2.0-ext.x_bearing, self.y+h/2.0);
            cr.show_text(&self.name);
        }
    }
}



impl DrawElement for Text {
    fn draw_element(&self, cr:&cairo::Context, layer:Layer) {
        // TODO
        cr.select_font_face("Sans", cairo::enums::FontSlant::Normal, cairo::enums::FontWeight::Normal);
        cr.set_font_size(self.dy);
        let text = self.shortened_text();
        let ext = cr.text_extents(&text);
        let w = ext.width;
        let h = ext.height;
        //cr.rectangle(self.x-w/2.0, self.y-h/2.0, w, h);
        //cr.fill();
        LAYER[&layer].color.set_source(cr);
        cr.move_to(self.x-w/2.0-ext.x_bearing,self.y+h/2.0);
        cr.show_text(&text);
    }
}

impl DrawElement for Name {
    fn draw_element(&self, cr:&cairo::Context, layer:Layer) {
        if layer == Layer::FFab {
            self.text.draw_element(cr, layer);
        }
    }
}

impl DrawElement for Reference {
    fn draw_element(&self, cr:&cairo::Context, layer:Layer) {
        if layer == Layer::FSilkS {
            self.text.draw_element(cr, layer);
        }
    }
}

impl DrawElement for Element {
    fn draw_element(&self, cr:&cairo::Context, layer:Layer) {
        match *self {
            Element::Line(ref l) => l.draw_element(cr, layer),
            Element::Rect(ref r) => r.draw_element(cr, layer),
            Element::Name(ref t) => t.draw_element(cr, layer),
            Element::Reference(ref t) => t.draw_element(cr, layer),
            Element::Smd(ref t) => t.draw_element(cr, layer),
            Element::Pad(ref t) => t.draw_element(cr, layer),
            Element::PythonError(_) => unreachable!(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PythonError {
    pub message:String
}


pub fn bound(v:&Vec<Element>) -> Bound {
    let mut s = Bound::default();
    for e in v {
        s = s.combine(&e.bounding_box());
    }
    s
}

impl ApplyFootprint for Element {
    fn apply_footprint(&self, f:&mut kicad::Footprint) {
        match *self {
            Element::Rect(ref e) => e.apply_footprint(f),
            Element::Line(ref e) => e.apply_footprint(f),
            Element::Name(ref e) => e.apply_footprint(f),
            Element::Reference(ref e) => e.apply_footprint(f),
            Element::Smd(ref e) => e.apply_footprint(f),
            Element::Pad(ref e) => e.apply_footprint(f),
            Element::PythonError(_) => unreachable!(),
        }
    }
}

impl ApplyFootprint for Rect {
    fn apply_footprint(&self, f:&mut kicad::Footprint) {
        if self.filled {
            f.rects.push(self.clone());
        } else {
            // TODO: corner export
            f.lines.push(Line {
                x1:self.x-self.dx/2.0, y1:self.y-self.dy/2.0,
                x2:self.x+self.dx/2.0, y2:self.y-self.dy/2.0,
                w:self.w, layer:self.layer,
            });
            f.lines.push(Line {
                x1:self.x+self.dx/2.0, y1:self.y-self.dy/2.0,
                x2:self.x+self.dx/2.0, y2:self.y+self.dy/2.0,
                w:self.w, layer:self.layer,
            });
            f.lines.push(Line {
                x1:self.x+self.dx/2.0, y1:self.y+self.dy/2.0,
                x2:self.x-self.dx/2.0, y2:self.y+self.dy/2.0,
                w:self.w, layer:self.layer,
            });
            f.lines.push(Line {
                x1:self.x-self.dx/2.0, y1:self.y+self.dy/2.0,
                x2:self.x-self.dx/2.0, y2:self.y-self.dy/2.0,
                w:self.w, layer:self.layer,
            });
        }
    }
}

impl ApplyFootprint for Line {
    fn apply_footprint(&self, f:&mut kicad::Footprint) {
        f.lines.push(self.clone());
    }
}

impl ApplyFootprint for Name {
    fn apply_footprint(&self, f:&mut kicad::Footprint) {
        f.name = Some(self.text.clone());
    }
}

impl ApplyFootprint for Reference {
    fn apply_footprint(&self, f:&mut kicad::Footprint) {
        f.reference = Some(self.text.clone());
    }
}

impl ApplyFootprint for Smd {
    fn apply_footprint(&self, f:&mut kicad::Footprint) {
        f.smd.push(self.clone())
    }
}

impl ApplyFootprint for Pad {
    fn apply_footprint(&self, f:&mut kicad::Footprint) {
        f.pad.push(self.clone())
    }
}
