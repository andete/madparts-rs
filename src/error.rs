// (c) 2018 Joost Yervante Damad <joost@damad.be>

use std::io;

use glib;
use pyo3;

// TODO: use failure crate

#[derive(Debug)]
pub enum MpError {
    GuiError(String),
    IOError(String),
    Python(String),
    Other(String),
}

impl From<glib::BoolError> for MpError {
    fn from(e: glib::BoolError) -> MpError {
        MpError::GuiError(format!("{:?}", e))
    }
}

impl From<io::Error> for MpError {
    fn from(e: io::Error) -> MpError {
        MpError::IOError(format!("{:?}", e))
    }
}

impl From<pyo3::PyErr> for MpError {
    fn from(e: pyo3::PyErr) -> MpError {
        MpError::Python(format!("{:?}", e))
    }
}

impl From<pyo3::PyDowncastError> for MpError {
    fn from(e: pyo3::PyDowncastError) -> MpError {
        MpError::Python(format!("{:?}", e))
    }
}
