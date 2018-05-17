// (c) 2018 Joost Yervante Damad <joost@damad.be>

use std::process::Command;
use std::fs::File;

use tempfile::tempdir;
use error::MpError;
use kicad;
use DrawState;

// python check_kicad_mod.py --nocolor -v /home/joost/prj/madparts-rs/Texas_TSSOP-14_5x4.4x1.2mm_EP.kicad_mod 


pub fn run_klc(draw_state:&DrawState, klc_dir:&str) -> Result<String, MpError> {
    let dir = tempdir()?;
    let name = format!("{}.kicad_mod", draw_state.name());
    let file_path = dir.path().join(&name);
    let mut f = File::create(&file_path)?;
    kicad::save(&draw_state.elements, &mut f)?;
    info!("Temp file: {}", file_path.display());
    let output = Command::new("/usr/bin/python")
        .current_dir(klc_dir)
        .arg("check_kicad_mod.py")
        .arg("--nocolor")
        .arg("-v")
        .arg(&format!("{}", file_path.display()))
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
