use std::collections::HashMap;
use std::fmt;
use cairo;

#[derive(Clone, Copy)]
pub struct Color {
    red:f64,
    green:f64,
    blue:f64,
    alpha:f64,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Layer {
    Background,
    Grid,
    Axes,
    #[serde(rename = "F.Cu")]
    FCu,
    #[serde(rename = "*.Cu")]
    Cu,
    #[serde(rename = "F.SilkS")]
    FSilkS,
    #[serde(rename = "F.Fab")]
    FFab,
    #[serde(rename = "F.CrtYd")]
    FCrtYd,
    #[serde(rename = "F.Mask")]
    FMask,
    #[serde(rename = "*.Mask")]
    Mask,
    #[serde(rename = "F.Paste")]
    FPaste,
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Layer::Background => write!(f, "Background"),
            Layer::Grid => write!(f, "Grid"),
            Layer::Axes => write!(f, "Axes"),
            Layer::FCu => write!(f, "F.Cu"),
            Layer::Cu => write!(f, "*.Cu"),
            Layer::FSilkS => write!(f, "F.SilkS"),
            Layer::FFab => write!(f, "F.Fab"),
            Layer::FCrtYd => write!(f, "F.CrtYd"),
            Layer::FMask => write!(f, "F.Mask"),
            Layer::Mask => write!(f, "*.Mask"),
            Layer::FPaste => write!(f, "F.Paste"),
        }
    }
}

pub struct LayerStat {
    pub color:Color,
    pub z:i64,
}

impl Color {
    pub fn set_source(&self, cr:&cairo::Context) {
        cr.set_source_rgba(self.red, self.green, self.blue, self.alpha);
    }
}

// kicad: see in common/colors.cpp
// and common/class_colors_design_settings.cpp

lazy_static! {
    pub static ref LAYER: HashMap<Layer, LayerStat> = {
        let mut m = HashMap::new();
        m.insert(Layer::Background, LayerStat {
            color:Color { red:0.0, green:0.0, blue:0.0, alpha:1.0 },
            z:-100,
        });
        m.insert(Layer::Grid, LayerStat {
            color:Color { red:0.52, green:0.52, blue:0.52, alpha:1.0 },
            z:-90,
        });
        m.insert(Layer::Axes, LayerStat {
            color:Color { red:0.0, green:0.0, blue:0.52, alpha:1.0 },
            z:-80,
        });
        
        let color = Color { red:1.0, green:0.0, blue:0.0, alpha:0.52 };
        m.insert(Layer::FCu, LayerStat { color, z:1 });
        m.insert(Layer::Cu, LayerStat { color, z:1 });
        
        m.insert(Layer::FMask, LayerStat {
            color:Color { red:0.0, green:0.00, blue:1.00, alpha:0.52 },
            z:8,
        });
        m.insert(Layer::FPaste, LayerStat {
            color:Color { red:1.0, green:0.82, blue:0.26, alpha:0.83 },
            z:9,
        });
        m.insert(Layer::FSilkS, LayerStat {
            color:Color { red:1.0, green:1.0, blue:1.0, alpha:0.83 },
            z:11,
        });
        m.insert(Layer::FFab, LayerStat {
            color:Color { red:1.0, green:1.0, blue:0.0, alpha:0.76 },
            z:12,
        });
        m.insert(Layer::FCrtYd, LayerStat {
            color:Color { red:0.5, green:0.5, blue:0.5, alpha:0.76 },
            z:13,
        });
        
        m
    };
    pub static ref LAYER_Z: Vec<(i64, Layer)> = {
        let mut v = vec![];
        for (ref k, ref x) in LAYER.iter() {
            v.push((x.z, **k));
        }
        v.sort_by(|(i,_),(j,_)| i.cmp(j));
        v
    };
}

/*
color_schemes['default'] = {
  'background': (0.0, 0.0, 0.0, 1.0),
  'grid': (0.5, 0.5, 0.5, 1.0),
  'axes': (1.0, 0.0, 0.0, 1.0),
  'name': (1.0, 1.0, 1.0, 1.0),
  'value': (1.0, 1.0, 1.0, 1.0),
  'silk': (1.0, 1.0, 1.0, 1.0),
  'bsilk': (0.7, 0.7, 0.7, 0.3),
  'docu': (1.0, 1.0, 0.0, 0.7),
  'smd': (0.0, 0.0, 1.0, 1.0),
  'pad': (0.0, 1.0, 0.0, 1.0),
  'meta':  (1.0, 1.0, 1.0, 1.0),
  'restrict':  (0.0, 1.0, 0.0, 0.3),
  'stop':  (0.0, 1.0, 1.0, 0.3),
  'keepout':  (1.0, 0.0, 0.5, 0.7),
  'bkeepout':  (0.7, 0.0, 0.35, 0.4),
  'vrestrict':  (0.0, 1.0, 0.0, 0.4),
  'unknown':  (1.0, 0.0, 1.0, 0.7),
  'hole': (1.0, 1.0, 1.0, 0.7),
  'edge': (1.0, 1.0, 1.0, 0.7),
  'paste': (0.0, 1.0, 0.0, 0.7),
  'bpaste': (0.0, 0.7, 0.0, 0.3),
  'comments': (0.82, 0.66, 0.63, 0.8),
  'assembly': (0.9, 0.6, 0.3, 0.8),
  'bassembly': (0.65, 0.5, 0.2, 0.5),
  'user1': (0.3, 0.6, 0.0, 0.7),
  'user2': (0.3, 0.0, 0.6, 0.7)
  }
*/
