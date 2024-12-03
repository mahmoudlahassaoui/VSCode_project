use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Orientation, Align, Label, MenuBar, MenuItem, Menu, AboutDialog, ColorButton, Dialog, DialogFlags, Adjustment, Scale};
use rand::Rng;
use gtk::glib::{self, MainContext};
use std::time::Duration;
use std::process::Command;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK");
        return;
    }

    let app = Application::builder()
        .application_id("com.example.GtkWindow")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1050)
        .default_height(700)
        .title("Simple GUI")
        .build();

    window.set_position(gtk::WindowPosition::Center);

    // Create a MenuBar
    let menubar = MenuBar::new();

    // Create a File menu
    let file_menu = Menu::new();
    let file_menu_item = MenuItem::with_label("File");
    file_menu_item.set_submenu(Some(&file_menu));
    menubar.append(&file_menu_item);

    // Add items to the File menu
    let exit_item = MenuItem::with_label("Exit");
    let app_clone = app.clone();
    exit_item.connect_activate(move |_| app_clone.quit());
    file_menu.append(&exit_item);

    // Create a Help menu
    let help_menu = Menu::new();
    let help_menu_item = MenuItem::with_label("Help");
    help_menu_item.set_submenu(Some(&help_menu));
    menubar.append(&help_menu_item);

    // Add items to the Help menu
    let about_item = MenuItem::with_label("About");
    about_item.connect_activate(|_| {
        let about_dialog = AboutDialog::new();
        about_dialog.set_program_name("Simple GTK App");
        about_dialog.set_version(Some("0.1"));
        about_dialog.set_comments(Some("A simple GTK application with a menu bar."));
        about_dialog.run();
        about_dialog.close();
    });
    help_menu.append(&about_item);

    // Create a Settings menu
    let settings_menu = Menu::new();
    let settings_menu_item = MenuItem::with_label("Settings");
    settings_menu_item.set_submenu(Some(&settings_menu));
    menubar.append(&settings_menu_item);

    // Add items to the Settings menu
    let label = Label::new(Some("Click me!"));
    label.set_markup("<b><span font_size='18000'>Click me!</span></b>");
    let button = gtk::Button::new();
    button.add(&label);
    button.set_halign(Align::Center);
    button.set_valign(Align::Center);

    let preferences_item = MenuItem::with_label("Preferences");
    let window_clone = window.clone();
    let button_clone = button.clone();
    preferences_item.connect_activate(move |_| {
        let dialog = Dialog::with_buttons::<ApplicationWindow>(Some("Preferences"), None, DialogFlags::empty(), &[("OK", gtk::ResponseType::Ok.into()), ("Cancel", gtk::ResponseType::Cancel.into())]);
        let content_area = dialog.content_area();
        let color_label = Label::new(Some("Background Color:"));
        content_area.pack_start(&color_label, true, true, 0);
        let color_button = ColorButton::new();
        content_area.pack_start(&color_button, true, true, 0);

        let font_size_label = Label::new(Some("Font Size:"));
        content_area.pack_start(&font_size_label, true, true, 0);
        let adjustment = Adjustment::new(18.0, 6.0, 72.0, 1.0, 1.0, 1.0);
        let font_size_scale = Scale::new(Orientation::Horizontal, Some(&adjustment));
        content_area.pack_start(&font_size_scale, true, true, 0);


        let response = dialog.run();
        if response == gtk::ResponseType::Ok.into() {
            let color = color_button.rgba();
            let css_provider = gtk::CssProvider::new();
            let color_css = format!(
                "window {{ background-color: rgb({}, {}, {}); }}",
                (color.red() * 255.0) as u8,
                (color.green() * 255.0) as u8,
                (color.blue() * 255.0) as u8
            );
            let _ = css_provider.load_from_data(color_css.as_bytes());
            window_clone.style_context().add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

            let font_size = font_size_scale.value() as i32;
            let label_text = format!("<b><span font_size='{}'>Click me!</span></b>", font_size);

            // Update the existing label
            if let Some(child) = button_clone.child() { 
                if let Ok(label) = child.downcast::<Label>() {
                    label.set_markup(&label_text);
                    button_clone.queue_draw(); 
                }
            }
        }
        dialog.close();
    });
    settings_menu.append(&preferences_item);



    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .build();

    let label = Label::new(Some("Click me!"));
    label.set_markup("<b><span font_size='18000'>Click me!</span></b>");
    let button = gtk::Button::new();
    button.add(&label);
    button.set_halign(Align::Center);
    button.set_valign(Align::Center);
    let window_clone = window.clone();
    button.connect_clicked(move |_| {
        let mut rng = rand::thread_rng();
        let r: f64 = rng.gen_range(0.0..1.0);
        let g: f64 = rng.gen_range(0.0..1.0);
        let b: f64 = rng.gen_range(0.0..1.0);
        let css_provider = gtk::CssProvider::new();
        let color_css = format!("window {{ background-color: rgb({}, {}, {}); transition: background-color 1s; }}", (r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8);
        let _ = css_provider.load_from_data(color_css.as_bytes());
        window_clone.style_context().add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
        if let Err(e) = Command::new("aplay").arg("/home/toor/Desktop/VSCode_project/click.wav").stdout(std::process::Stdio::null()).spawn() {
            println!("Error playing sound: {:?}", e);
        }
    });

    let exit_label = Label::new(Some("Exit"));
    exit_label.set_markup("<b><span font_size='18000'>Exit</span></b>");
    let exit_button = gtk::Button::new();
    exit_button.add(&exit_label);
    exit_button.set_halign(Align::Center);
    exit_button.set_valign(Align::Center);
    let app_clone = app.clone();
    exit_button.connect_clicked(move |_| {
        if let Err(e) = Command::new("aplay").arg("/home/toor/Desktop/VSCode_project/click.wav").stdout(std::process::Stdio::null()).spawn() {
            println!("Error playing sound: {:?}", e);
        }
        app_clone.quit();
    });

    let countdown_button = gtk::Button::new();
    let countdown_label = Label::new(Some("Countdown"));
    countdown_label.set_markup("<b><span font_size='18000'>Countdown</span></b>");
    countdown_button.add(&countdown_label);
    countdown_button.set_halign(Align::Center);
    countdown_button.set_valign(Align::Center);
    let app_clone = app.clone();
    let countdown_label_clone = countdown_label.clone();
    countdown_button.connect_clicked(move |_| {
        if let Err(e) = Command::new("aplay").arg("/home/toor/Desktop/VSCode_project/click.wav").stdout(std::process::Stdio::null()).spawn() {
            println!("Error playing sound: {:?}", e);
        }
        let app_clone = app_clone.clone();
        let countdown_label_clone = countdown_label_clone.clone();
        MainContext::default().spawn_local(async move {
            countdown_label_clone.set_markup("<b><span font_size='36000'>3</span></b>");
            glib::timeout_future(Duration::from_secs(1)).await;
            countdown_label_clone.set_markup("<b><span font_size='36000'>2</span></b>");
            glib::timeout_future(Duration::from_secs(1)).await;
            countdown_label_clone.set_markup("<b><span font_size='36000'>1</span></b>");
            glib::timeout_future(Duration::from_secs(1)).await;
            app_clone.quit();
        });
    });

    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();
    hbox.pack_start(&button, true, true, 0);
    hbox.pack_start(&countdown_button, true, true, 0);
    hbox.pack_start(&exit_button, true, true, 0);
    vbox.pack_start(&menubar, false, false, 0);
    vbox.pack_start(&hbox, true, true, 0);
    window.add(&vbox);
    window.show_all();
}
