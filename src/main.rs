use gtk::{prelude::*};

const ID_APP : &str = "io.github.singleslice.text-edit";

fn main() {
    let app = adw::Application::builder().application_id(ID_APP).build();

    app.connect_startup(|_| load_stylesheet());
    app.connect_activate(build_main_win);
    app.run();
}

fn load_stylesheet(){ // loads css
    let css_prov = gtk::CssProvider::new();
    css_prov.load_from_data(include_bytes!("style.css"));

    gtk::StyleContext::add_provider_for_display(&gdk4::Display::default().expect("Could not connect to a display."),
    &css_prov, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION );

}

fn build_main_win(app: &adw::Application) {
// ---------------- main bar setup ------------------------ -- //
    let main_header = adw::HeaderBar::builder()
    .title_widget(&gtk::Label::builder().label("sus").build())
    .build();

    let menu_button = gtk::ToggleButton::builder()
    .icon_name("open-menu-symbolic")
    .build();
    main_header.pack_end(&menu_button);
    
    let flap_button = gtk::Button::builder()
    .icon_name("sidebar-show-symbolic")
    .build();
    main_header.pack_start(&flap_button);

// ---------------- side bar setup ------------------------ -- //
    let side_header = adw::HeaderBar::builder()
    .show_end_title_buttons(false)
    .show_start_title_buttons(false)
    .build();

    let open_folder_button = gtk::Button::builder()
    .icon_name("folder-open-symbolic")
    .build();

    let new_page_button = gtk::Button::builder()
    .icon_name("document-new-symbolic")
    .build();

    side_header.pack_end(&open_folder_button);
    side_header.pack_start(&new_page_button);

// ---------------- flap setup ------------------------------ //
    let main_view = (adw::Flap::builder())
    .separator(&gtk::Separator::builder().build())
    .reveal_flap(true)
    .visible(true)
    .build();

    let side_flap = gtk::Box::builder()
    .orientation(gtk::Orientation::Vertical)
    .width_request(250)
    .build();
    main_view.set_flap(Some(&side_flap));
    side_flap.append(&side_header);

// ---------------- side flap ------------------------------ //
    let flap_list_box = gtk::ListBox::builder().build();
    let cloned_view = main_view.clone();

    side_flap.append(&flap_list_box);
    flap_button.connect_clicked(move |side_button| {
        open_close_flap(&side_button, &cloned_view);
    });

// ---------------- main win ------------------------------ //
    let clamp = adw::Clamp::builder()
    .maximum_size(800)
    .build();

    let boxy = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
    .width_request(400)
    .build();
    boxy.append(&main_header);
    main_view.set_content(Some(&boxy));

    let scroll_win = gtk::ScrolledWindow::builder()
    .vexpand(true)
    .build();
    scroll_win.set_child(Some(&clamp));
    boxy.append(&scroll_win);

// ---------------- text view -------------------------- //
    let main_text_view = gtk::TextView::builder()
    .wrap_mode(gtk::WrapMode::WordChar)
    .build();
    main_text_view.set_widget_name("main_text_view");

    clamp.set_child(Some(&main_text_view));

// ---------------- window ------------------------------- //
    let window = adw::ApplicationWindow::builder()
    .application(app)
    .content(&main_view)
    .title("among text")
    .default_width(800)
    .default_height(450)
    .build();

    window.present();
}

fn open_close_flap(_button : &gtk::Button, flap : &adw::Flap ){
    println!("flap button on/off");
    flap.set_reveal_flap(!flap.reveals_flap());
    
}