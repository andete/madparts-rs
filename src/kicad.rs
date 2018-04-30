// (c) 2016-2018 Joost Yervante Damad <joost@damad.be>

use element::*;
use std::fs;
use std::io::Write;
use chrono::{DateTime,Local};
use error::MpError;

use std::path::PathBuf;

#[derive(Default)]
pub struct Footprint {
    pub name:Option<Text>,
    pub reference:Option<Text>,
    pub desc:String,
    pub tags:String,
    pub pad:Vec<Pad>,
    pub smd:Vec<Smd>,
    pub lines:Vec<Line>,
    
}

fn to_footprint(elements:&Vec<Element>) -> Footprint {
    let mut f = Footprint::default();
    for e in elements {
        e.apply_footprint(&mut f);
    }
    f
}

pub fn save(elements:&Vec<Element>, filename:PathBuf) -> Result<(), MpError> {
    let footprint = to_footprint(elements);
    let mut f = fs::File::create(filename)?;
    // TODO
    let name = footprint.name.as_ref().ok_or(MpError::Save("footprint is missing a name".into()))?;
    let reference = footprint.reference.as_ref().ok_or(MpError::Save("footprint is missing a reference".into()))?;
    let local: DateTime<Local> = Local::now();
    let ts = local.timestamp();
    write!(f, "(module {} (layer F.Cu) (tedit {:X})\n", name.txt, ts)?;
    write!(f, "  (tags \"\")\n")?; // TODO tags
    write!(f, "  (attr smd)\n")?; // TODO pth
    
    write!(f, "  (fp_text reference REF** (at {} {}) layer {})\n", reference.x, reference.y, reference.layer)?;
    write!(f, "    (effects (font (size {} {}) (thickness {})))", reference.dy, reference.dy, reference.thickness)?;
    write!(f, "  )\n")?;
    
    write!(f, "  (fp_text value {} (at {} {}) layer {})\n", name.txt, name.x, name.y, name.layer)?;
    write!(f, "    (effects (font (size {} {}) (thickness {})))\n", name.dy, name.dy, name.thickness)?;
    write!(f, "  )\n")?;

    // TODO: maybe at some point allow overriding
    write!(f, "  (fp_text user %R (at 0 0) layer F.Fab)\n")?;
    write!(f, "    (effects (font (size 0.8 0.8) (thickness 0.1)))\n")?;
    write!(f, "  )\n")?;


    for line in &footprint.lines {
        write!(f, "  (fp_line (start {} {}) (end {} {}) (layer {}) (width {}))\n", line.x1, line.y1, line.x2, line.y2, line.layer, line.w)?;
    }

    for pad in &footprint.smd {
        write!(f, "  (pad {} smd rect (at {} {}) (size {} {}) (layers F.Cu F.Paste F.Mask))\n", pad.name, pad.x, pad.y, pad.dx, pad.dy)?;
    }

    for pad in &footprint.pad {
        write!(f, "  (pad {} thru_hole circle (at {} {}) (size {} {}) (drill {}) (layers *.Cu *.Mask))\n", pad.name, pad.x, pad.y, pad.dx, pad.dy, pad.drill)?;
    }

    // TODO model...
    
    write!(f, ")\n")?;
    Ok(())
}
