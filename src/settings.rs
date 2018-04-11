use std::collections::HashMap;
use cairo;

pub struct Color {
    red:f64,
    green:f64,
    blue:f64,
    alpha:f64,
}

impl Color {
    pub fn set_source(&self, cr:&cairo::Context) {
        cr.set_source_rgba(self.red, self.green, self.blue, self.alpha);
    }
}

lazy_static! {
    pub static ref COLOR_SCHEME: HashMap<&'static str, Color> = {
        let mut m = HashMap::new();
        m.insert("background", Color { red:0.0, green:0.0, blue:0.0, alpha:1.0 });
        m
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
