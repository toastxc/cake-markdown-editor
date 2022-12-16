mod windows {pub mod main_window;}

use gtk::prelude::*;
use windows::main_window::main_window::build_main_win;

const ID_APP : &str = "io.github.singleslice.text-edit";

fn main() {
    let app = adw::Application::builder().application_id(ID_APP).build();
    app.connect_startup(|_| load_stylesheet());
    app.connect_activate(build_main_win);
    app.run();
}

fn load_stylesheet(){ // loads css
    let css_prov = gtk::CssProvider::new();
    css_prov.load_from_data(include_bytes!("../data/style.css"));

    gtk::StyleContext::add_provider_for_display(&gdk4::Display::default()
    .expect("Could not connect to a display."),&css_prov, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION );
}