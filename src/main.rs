extern crate futures;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Entry, Label, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("faild to initialize GTK");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("mango");
    window.set_default_size(800, 800);

    let label = Label::new(Some("HTML"));
    let entry = Entry::new();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    // child: &P, expand: bool, fill: bool, padding: u32
    vbox.pack_start(&entry, false, true, 2);
    vbox.pack_start(&label, true, true, 3);
    window.add(&vbox);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    entry.connect_activate(move |x| {
        let url: String = x.get_text().unwrap();
        label.set_text(&(url).to_string());
    });
    gtk::main();
}
