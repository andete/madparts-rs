// (c) 2018 Joost Yervante Damad <joost@damad.be>

use tempfile::NamedTempFile;
use error::MpError;
use kicad;
use DrawState;

pub fn run_klc(draw_state:&DrawState) -> Result<String, MpError> {
    let mut f = NamedTempFile::new()?;
    kicad::save(&draw_state.elements, f.as_file_mut())?;
    // TODO: run KLC on f.path()
    info!("Temp file: {}", f.path().display());
    Ok("".into())
}
