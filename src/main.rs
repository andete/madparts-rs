// (c) 2016-2018 Joost Yervante Damad <joost@damad.be>
#![feature(proc_macro, specialization, const_fn)]

extern crate cairo;
extern crate clap;
extern crate env_logger;
extern crate pyo3;

extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;

extern crate inotify;
#[macro_use]
extern crate log;
extern crate range;


use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use clap::{Arg, App};

use inotify::{WatchMask, Inotify};

use pyo3::{Python, ObjectProtocol, PyList, PyDict};

use gtk::{WidgetExt, StatusbarExt, TextBufferExt};

use error::MpError;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const PRELUDEPY:&'static str = include_str!("prelude.py");

#[derive(Debug,Default)]
pub struct DrawState {
    pub bound:element::Bound,
    pub elements:Vec<element::Element>,
}

fn run() -> Result<(), MpError> {
    std::env::set_var("RUST_LOG","debug");
    env_logger::init();
    let matches = App::new("madparts")
        .version(VERSION)
        .author("Joost Yervante Damad <joost@damad.be>")
        .about("a functional footprint editor")
        .arg(Arg::with_name("INPUT")
             .help("Sets the python file to use")
             .required(true)
             .index(1))
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    let filepath:PathBuf = Path::new(&filename).canonicalize().unwrap();
    info!("Filename: {}", filepath.display());
    
    let filedir:PathBuf = filepath.parent().unwrap().into();
    info!("Dir: {}", filedir.display());

    if let Err(err) = gtk::init() {
        error!("Failed to initialize GTK.");
        return Err(err.into())
    }

    let mut ino = match Inotify::init() {
        Ok(ino) => ino,
        Err(err) => {
            error!("Failed to initialize INotify");
            return Err(err.into())
        },
    };

    // close_write,moved_to,create indicate the file was possibly messed with
    let _file_watch = ino.add_watch(&filedir, WatchMask::CREATE | WatchMask::MOVED_TO | WatchMask::CLOSE_WRITE).unwrap();

    let draw_state = Arc::new(Mutex::new(DrawState::default()));
    
    let (window, statusbar, input_buffer, exit) = gui::make_gui(&filename, draw_state.clone());
    
    let update_input = Arc::new(AtomicBool::new(true));
    let update_input_timeout_loop = update_input.clone();
    gtk::timeout_add(250, move || {
        let mut buffer = [0; 1024];
        for event in ino.read_events(&mut buffer).unwrap() {
            let mut modified = false;
            if event.name == filepath.file_name() {
                debug!("modified!");
                modified = true;
            }
            if modified {
                update_input_timeout_loop.store(true, Ordering::SeqCst);
                break;
            }
        }
        glib::Continue(true)
    });
    
    window.show_all();

    let gil = Python::acquire_gil();
    let py = gil.python();

    let sys = py.import("sys")?;
    let version: String = sys.get( "version")?.extract()?;
    
    info!("using python: {}", version);

    py.run(PRELUDEPY,None,None)?;
    // info!("Using prelude: {}", PRELUDEPY);
    
    loop {
        {
            if exit.load(Ordering::SeqCst) {
                break;
            }
        }
        gtk::main_iteration();
        if update_input.compare_and_swap(true, false, Ordering::SeqCst) {
            statusbar.push(1, "Updating...");
            let data = util::read_file(&filename).unwrap();
            input_buffer.set_text(&data);
            statusbar.pop(1);
            trace!("updated");
            py.run(&data,None,None)?;
            let res = py.eval("flatten(footprint())", None,None)?;
            info!("res: {:?}", res);
            let resl:&PyList = res.extract()?;
            let mut draw_state = draw_state.lock().unwrap();
            draw_state.elements.clear();
            for i in 0..resl.len() {
                let item = resl.get_item(i as isize);
                let gen = item.call_method0("generate")?;
                //info!("gen: {:?}", gen);
                let genl:&PyList = gen.extract()?;
                for j in 0..genl.len() {
                    let item = genl.get_item(j as isize);
                    info!("item: {:?}", item);
                    let idict:&PyDict = item.extract()?;
                    let x = element::Element::try_from(idict)?;
                    info!("x: '{:?}'", x);
                    draw_state.elements.push(x);
                }
            }
            draw_state.bound = element::bound(&draw_state.elements);
            info!("Bound: {:?}", draw_state.bound);
        }
    }
    Ok(())
}

fn main() {
    util::main_run(run);
}

mod error;
mod gui;
mod util;
mod element;
