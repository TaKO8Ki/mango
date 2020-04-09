extern crate gtk;
extern crate reqwest;

use gtk::prelude::*;

fn get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = reqwest::Url::parse(url)?;
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}

fn main() {
    if gtk::init().is_err() {
        println!("faild to initialize GTK");
        return;
    }

    let win = gtk::Window::new(gtk::WindowType::Toplevel);
    win.set_title("mango");
    win.set_default_size(800, 800);

    let scr_win = gtk::ScrolledWindow::new(None, None);

    let label = gtk::Label::new(Some("HTML"));
    let entry = gtk::Entry::new();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    vbox.pack_start(&entry, false, true, 2);
    vbox.pack_start(&label, true, true, 3);
    scr_win.add(&vbox);
    win.add(&scr_win);

    win.show_all();

    let html: &str = "
                    <body>
                    <div>
                        <h1>Example Domain</h1>
                        <p>This domain is for use in illustrative examples in documents. You may use this
                        domain in literature without prior coordination or asking for permission.</p>
                        <p><a href=\"https://www.iana.org/domains/example\">More information...</a></p>
                    </div>
                    </body>
                    ";

    let css: &str = "a { display: block; }";

    entry.connect_activate(move |x| {
        let url: String = x.get_text().unwrap();
        match get(&(url)) {
            Ok(_) => {
                let root_node =
                    mango::interfaces::controllers::parser::html::parse(html.to_string());
                let stylesheet =
                    mango::interfaces::controllers::parser::css::parse(css.to_string());
                let style_root =
                    mango::interfaces::controllers::style::style_tree(&root_node, &stylesheet);
                println!("{:?}", style_root);
                label.set_text(&(html).to_string());
            }
            Err(err) => println!("Error: {:?}", err),
        }
    });

    win.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
