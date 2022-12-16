pub mod main_window {
    pub fn build_main_win(app: &adw::Application) {
    use gtk::prelude::*;


    // ---------------- main bar setup ------------------------ -- //
        let main_header = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::builder().label("sus").build())
        .build();
    
        let menu_button = gtk::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .build();
        main_header.pack_end(&menu_button);
        
        let flap_button = gtk::Button::builder()
        .icon_name("sidebar-show-symbolic")
        .build();
        main_header.pack_start(&flap_button);
    // ---------------- popover menu setup ------------------------ -- //
        let pop_menu_model = gio::Menu::new();
        pop_menu_model.append(Some("About"), Some("gock"));
    
        menu_button.set_menu_model(Some(&pop_menu_model));
    
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
        .name("sidebar")
        .width_request(250)
        .build();
        main_view.set_flap(Some(&side_flap));
        side_flap.append(&side_header);
    
    // ---------------- side flap ------------------------------ //
        let flap_list_box = gtk::ListBox::builder().build();
    
        //flap_list_box.append(&folder_treeview);
    
        side_flap.append(&flap_list_box);
    
        let cloned_view = main_view.clone();
        flap_button.connect_clicked(move |side_button| {
            open_close_flap(&side_button, &cloned_view);
        });
    
    // ---------------- main win ------------------------------ //
        let clamp = adw::Clamp::builder()
        .maximum_size(800)
        .build();
    
        let boxy = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
        .width_request(300)
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
}