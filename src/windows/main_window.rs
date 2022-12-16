pub mod main_window {
    use gtk::prelude::*;

    pub fn build_main_win(app: &adw::Application) {

    // ---------------- main bar setup ------------------------ -- //
        let title_file_name = gtk::Label::builder().label("Sample name for a file").build();
        let main_header = adw::HeaderBar::builder()
        .title_widget(&title_file_name)
        .build();
    
        let menu_button = gtk::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .build();
        
        let flap_button = gtk::Button::builder()
        .icon_name("sidebar-show-symbolic")
        .build();

        
        main_header.pack_start(&flap_button);
        main_header.pack_end(&menu_button);

    // ---------------- popover menu setup ------------------------ -- //
        
        // TODO : menu
        //let pop_menu_model = gio::Menu::new();
        //pop_menu_model.append(Some("About"), Some(gock));
        //menu_button.set_menu_model(Some(&pop_menu_model));

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
        flap_button.connect_clicked(move |flap_button| {
            open_close_flap(&flap_button, &cloned_view);
        });
    
    // ---------------- main win ------------------------------ //
        let clamp = adw::Clamp::builder()
        .maximum_size(800)
        .build();
    
        let boxy = gtk::Box::builder().orientation(gtk::Orientation::Vertical)
        .width_request(300)
        .build();

    
        let scroll_win = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .build();


        main_view.set_content(Some(&boxy));
        boxy.append(&main_header); 
        boxy.append(&scroll_win);
        scroll_win.set_child(Some(&clamp));
        
    // ---------------- text view -------------------------- //
        let main_text_view = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::WordChar)
        .name("main_text_view")
        .build();

        let main_text_view_buffer = main_text_view.buffer();


        clamp.set_child(Some(&main_text_view));

    // ---------------- text char count ------------------------ //
        let bottom_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .halign(gtk::Align::End)
        .build();

        let char_count_label = gtk::Label::builder().label("Char count: ").name("char_count").build();
        let char_count = gtk::Label::builder().label("0").name("char_count").margin_end(10).build();
        let char_count_clone = char_count.clone();

        main_text_view_buffer.connect_changed(move | text_buffer | {on_text_buffer_change(text_buffer, &char_count_clone)});


        boxy.append(&bottom_box);
        bottom_box.append(&char_count_label);
        bottom_box.append(&char_count);

    // ---------------- window ------------------------------- //
        let window = adw::ApplicationWindow::builder()
        .application(app)
        .content(&main_view)
        .title("Cake Text")
        .default_width(800)
        .default_height(450)
        .height_request(200)
        .build();
    
        window.present();
    }
    
    fn open_close_flap(_button : &gtk::Button, flap : &adw::Flap ){
        flap.set_reveal_flap(!flap.reveals_flap());
    }

    fn on_text_buffer_change(buffer : &gtk::TextBuffer, char_label : &gtk::Label){
        char_label.set_label(&buffer.char_count().to_string());
    }
}