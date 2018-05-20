// (c) 2018 Joost Yervante Damad <joost@damad.be>

use cairo;
use gdk_pixbuf::Pixbuf;
use gio::MemoryInputStream;
use glib::Bytes;
use gtk;
use gtk::prelude::*;
use gtk::{AboutDialog, DrawingArea, Menu, MenuBar, MenuItem, Statusbar};
use gtk::{Label, Notebook, ScrolledWindow, TextBuffer, TextView, Window};

use gtk::{FileChooserAction, FileChooserDialog, FileChooserExt, ResponseType};

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use VERSION;

use DrawState;

use element::DrawElement;
use layers::{Layer, LAYER, LAYER_Z};

const ICON: &'static str = include_str!("../media/icon.svg");

fn draw_fn(draw_state: Arc<Mutex<DrawState>>, area: &DrawingArea, cr: &cairo::Context) -> Inhibit {
    let w: f64 = area.get_allocated_width().into();
    let h: f64 = area.get_allocated_height().into();
    info!("w: {}, h: {}", w, h);

    // set background
    LAYER[&Layer::Background].color.set_source(cr);
    cr.rectangle(0.0, 0.0, w, h);
    cr.fill();

    // scale x and y
    let draw_state = draw_state.lock().unwrap();
    if draw_state.elements.is_empty() {
        return Inhibit(false);
    }
    let dw = draw_state.bound.max_x - draw_state.bound.min_x;
    let dh = draw_state.bound.max_y - draw_state.bound.min_y;
    info!("dw: {}, dh: {}", dw, dh);
    let t = w / h;
    let dt = dw / dh;
    info!("t: {}, dt: {}", t, dt);
    let (sw, sh) = if t > dt {
        // dh is limiting factor
        cr.scale(w / (w * dh / h), h / dh);
        info!("scaling to: {}, {}", w * dh / h, dh);
        (w * dh / h, dh)
    } else {
        // dw is limiting factor
        cr.scale(w / dw, h / (h * dw / w));
        info!("scaling to: {}, {}", dw, h * dw / w);
        (dw, h * dw / w)
    };

    // translate origin
    cr.translate(
        -draw_state.bound.min_x + (sw - dw) / 2.0,
        -draw_state.bound.min_y + (sh - dh) / 2.0,
    );

    // draw axes
    LAYER[&Layer::Axes].color.set_source(cr);
    cr.move_to(-dw * 2.0, 0.0);
    cr.set_line_width(0.01);
    cr.line_to(dw * 2.0, 0.0);
    cr.stroke();
    cr.move_to(0.0, -dh * 2.0);
    cr.line_to(0.0, dh * 2.0);
    cr.stroke();

    // draw unit dots
    LAYER[&Layer::Grid].color.set_source(cr);
    cr.set_line_cap(cairo::enums::LineCap::Round);
    cr.set_line_width(0.01);
    for ix in -((dw * 2.0) as i32)..((dw * 2.0) as i32) {
        for iy in -((dh * 2.0) as i32)..((dh * 2.0) as i32) {
            cr.move_to(ix as f64, iy as f64);
            cr.close_path();
        }
    }
    cr.stroke();

    // draw elements, layer by layer
    for (_z, layer) in LAYER_Z.iter() {
        for e in &draw_state.elements {
            e.draw_element(cr, *layer);
        }
    }

    Inhibit(false)
}

pub struct GuiData {
    window: Window,
    statusbar: Statusbar,
    input_buffer: TextBuffer,
    klc_buffer: TextBuffer,
    notebook: Notebook,
    exit: Arc<AtomicBool>,
    save: Arc<AtomicBool>,
}

impl GuiData {
    pub fn show_all(&self) {
        self.window.show_all();
    }

    pub fn draw(&self) {
        self.window.queue_draw();
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    pub fn want_exit(&self) -> bool {
        self.exit.load(Ordering::SeqCst)
    }

    pub fn want_save(&self) -> bool {
        self.save.compare_and_swap(true, false, Ordering::SeqCst)
    }

    pub fn show_drawing_page(&self) {
        self.notebook.set_current_page(Some(0));
    }

    pub fn show_text_page(&self) {
        self.notebook.set_current_page(Some(1));
    }
    pub fn set_status(&self, status: &str) {
        self.statusbar.pop(0);
        self.statusbar.push(0, status);
    }

    pub fn set_input_text(&self, txt: &str) {
        self.input_buffer.set_text(txt);
    }

    pub fn set_klc_text(&self, txt: &str) {
        self.klc_buffer.set_text(txt);
    }
}

pub fn make_gui(filename: &str, draw_state: Arc<Mutex<DrawState>>) -> GuiData {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title(&format!("madparts (rustic edition) {}", VERSION));
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let menu_bar = MenuBar::new();

    let menu = Menu::new();
    let export = MenuItem::new_with_label("Export");
    menu.append(&export);
    let quit = MenuItem::new_with_label("Quit");
    menu.append(&quit);
    let file = MenuItem::new_with_label("File");
    file.set_submenu(Some(&menu));

    let menu = Menu::new();
    let about_me = MenuItem::new_with_label("About");
    menu.append(&about_me);
    let help = MenuItem::new_with_label("Help");
    help.set_submenu(Some(&menu));

    menu_bar.append(&file);
    menu_bar.append(&help);

    let about = {
        let about = AboutDialog::new();
        about.set_transient_for(Some(&window));
        about.add_credit_section("Credits", &["Joost Yervante Damad <joost@damad.be>"]);
        about.set_copyright(Some("MIT/Apache-2.0"));
        about.set_program_name("madparts");
        about.set_version(Some(VERSION));
        about.set_website(Some("http://madparts.org/"));
        about.set_website_label(Some("madparts"));
        let stream = MemoryInputStream::new_from_bytes(&Bytes::from_static(ICON.as_bytes()));
        let logo = Pixbuf::new_from_stream_at_scale(&stream, 64, 64, true, None).unwrap();
        about.set_logo(Some(&logo));
        about
    };

    about_me.connect_activate(move |_| {
        about.show();
        about.run();
        about.hide();
    });

    v_box.pack_start(&menu_bar, false, false, 0);

    let notebook = Notebook::new();

    v_box.pack_start(&notebook, true, true, 0);

    let input_buffer = TextBuffer::new(None);
    let data = fs::read_to_string(&filename).unwrap();
    input_buffer.set_text(&data);
    let input = TextView::new_with_buffer(&input_buffer);
    input.set_editable(false);
    let scrolled_input = ScrolledWindow::new(None, None);
    scrolled_input.add(&input);
    notebook.append_page(&scrolled_input, Some(&Label::new(Some("input"))));

    let view = DrawingArea::new();
    notebook.append_page(&view, Some(&Label::new(Some("view"))));

    view.connect_draw(move |a, c| draw_fn(draw_state.clone(), a, c));

    let klc_buffer = TextBuffer::new(None);
    let klc = TextView::new_with_buffer(&klc_buffer);
    klc.set_editable(false);
    let scrolled_klc = ScrolledWindow::new(None, None);
    scrolled_klc.add(&klc);
    notebook.append_page(&scrolled_klc, Some(&Label::new(Some("KLC"))));

    let statusbar = Statusbar::new();
    statusbar.push(0, "Ready.");
    v_box.pack_start(&statusbar, false, false, 0);

    window.add(&v_box);

    // exit handling
    let exit = Arc::new(AtomicBool::new(false));
    let exit2 = exit.clone();
    window.connect_delete_event(move |_, _| {
        exit2.store(true, Ordering::SeqCst);
        Inhibit(false)
    });
    let exit2 = exit.clone();
    quit.connect_activate(move |_| {
        exit2.store(true, Ordering::SeqCst);
    });

    // save handling
    let save = Arc::new(AtomicBool::new(false));
    let save2 = save.clone();
    export.connect_activate(move |_| {
        save2.store(true, Ordering::SeqCst);
    });

    GuiData {
        window,
        statusbar,
        input_buffer,
        klc_buffer,
        notebook,
        exit,
        save,
    }
}

pub fn get_export_filename(ui: &GuiData, filename: String) -> Option<PathBuf> {
    let d = FileChooserDialog::with_buttons(
        Some("Export kicad file"),
        Some(&ui.window),
        FileChooserAction::Save,
        &[
            ("_Cancel", ResponseType::Cancel),
            ("_Export", ResponseType::Accept),
        ],
    );
    d.set_current_name(filename);
    let res: ResponseType = d.run().into();
    if res == ResponseType::Accept {
        let filename = d.get_filename().unwrap();
        d.destroy();
        Some(filename)
    } else {
        d.destroy();
        None
    }
}
