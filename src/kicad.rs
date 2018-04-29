// (c) 2016-2018 Joost Yervante Damad <joost@damad.be>

use element::*;

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

pub fn to_footprint(elements:&Vec<Element>) -> Footprint {
    let mut f = Footprint::default();
    for e in elements {
        e.apply_footprint(&mut f);
    }
    f
}
