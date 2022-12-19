mod windows {pub mod main_window;}

use gtk::prelude::*;
use windows::main_window::main_window::build_main_win;

const ID_APP : &str = "io.github.singleslice.text-edit";

fn main() {
    let app = adw::Application::builder().application_id(ID_APP).build();
    
    app.connect_startup(|_| create_config_dir());
    app.connect_startup(|_| load_stylesheet());

    app.connect_activate(build_main_win);
    app.run();
}

fn create_config_dir(){ // creates config directory
    let user_dirs = directories::ProjectDirs::from("io", "singleslice", "CakeMD");
    let config_dir_exist = user_dirs.as_ref().unwrap().config_dir().exists();

    if config_dir_exist == false {
        println!("config directory {} not found. creating.", directories::ProjectDirs::from("io", "singleslice", "CakeMD").unwrap().config_dir().display());
        std::fs::DirBuilder::new().create(user_dirs.unwrap().config_dir()).expect("fuck, shit");
    }
}

fn load_stylesheet(){ // loads css
    let css_prov = gtk::CssProvider::new();
    css_prov.load_from_data(include_bytes!("../data/style.css"));

    gtk::StyleContext::add_provider_for_display(&gdk4::Display::default()
    .expect("Could not connect to a display."),&css_prov, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION );
}