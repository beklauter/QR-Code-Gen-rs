use std::error::Error;
use std::path::Path;

use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Entry, Image, Orientation};
use gtk::prelude::*;
use image::Luma;
use qrcode::QrCode;

fn main() -> Result<(), Box<dyn Error>> {
    let application = Application::builder()
        .application_id("de.beklauter.qrcodegenerator")
        .build();

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();

    Ok(())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("QR Code Generator")
        .default_width(300)
        .default_height(400)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 0);
    let entry = Entry::new();
    entry.set_placeholder_text(Some("Enter URL here..."));
    let path_entry = Entry::new();
    path_entry.set_placeholder_text(Some("Enter save path here..."));
    let button = Button::with_label("Generate QR Code");
    let image_widget = Image::new();

    vbox.append(&entry);
    vbox.append(&path_entry);
    vbox.append(&button);
    vbox.append(&image_widget);
    window.set_child(Some(&vbox));

    button.connect_clicked(move |_| {
        let url = entry.text().to_string();
        let save_path = path_entry.text().to_string();
        if let Ok(qr) = QrCode::new(url.as_bytes()) {
            let qr_image = qr.render::<Luma<u8>>().build();
            let file_name = format!("{}/QRCode_{}.png", save_path, chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S"));
            let path = Path::new(&file_name);
            qr_image.save(path).expect("Failed to save QR code");
            println!("QR code saved as {}", file_name);
        }
    });

    window.show();
}