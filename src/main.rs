use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Orientation, Align, Label};
use rand::Rng;
use gtk::glib::{self, MainContext};
use std::time::Duration;

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

    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .build();

    let label = Label::new(Some("Click me!"));
    label.set_markup("<b><span font-size='18000'>Click me!</span></b>");
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
        let _ = css_provider.load_from_data(format!("window {{ background-color: rgb({}, {}, {}); }}", (r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8).as_bytes());
        window_clone.style_context().add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    });

    let exit_label = Label::new(Some("Exit"));
    exit_label.set_markup("<b><span font-size='18000'>Exit</span></b>");
    let exit_button = gtk::Button::new();
    exit_button.add(&exit_label);
    exit_button.set_halign(Align::Center);
    exit_button.set_valign(Align::Center);
    let app_clone = app.clone();
    exit_button.connect_clicked(move |_| {
        app_clone.quit();
    });

    let countdown_label = Label::new(Some("Countdown"));
    countdown_label.set_markup("<b><span font-size='18000'>Countdown</span></b>");
    let countdown_button = gtk::Button::new();
    countdown_button.add(&countdown_label);
    countdown_button.set_halign(Align::Center);
    countdown_button.set_valign(Align::Center);
    let app_clone = app.clone();
    countdown_button.connect_clicked(move |_| {
        let app_clone = app_clone.clone();
        MainContext::default().spawn_local(async move {
            glib::timeout_future(Duration::from_secs(3)).await;
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
    vbox.pack_start(&hbox, true, true, 0);
    window.add(&vbox);
    window.show_all();
}
