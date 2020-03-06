extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::{ActionMapExt, ApplicationExt, MenuExt, SimpleActionExt};
use gtk::{ContainerExt, GtkApplicationExt, TextBufferExt, TextViewExt, WidgetExt, WindowExt};

fn main() {
    match gtk::Application::new(
        "com.github.koji-m.vanilla_text",
        gio::APPLICATION_HANDLES_OPEN,
    ) {
        Ok(app) => {
            app.connect_activate(|app| {
                let new_window_action = gio::SimpleAction::new("new_window", None);
                {
                    let app = app.clone();
                    new_window_action.connect_activate(move |_, _| {
                        create_window(&app);
                    });
                }

                let quit_action = gio::SimpleAction::new("quit", None);
                {
                    let app = app.clone();
                    quit_action.connect_activate(move |_, _| {
                        app.quit();
                    });
                }

                app.add_action(&new_window_action);
                app.add_action(&quit_action);

                {
                    use gio::{Menu, MenuItem};

                    let menubar = Menu::new();

                    let submenu_file = Menu::new();
                    let newwindow = MenuItem::new("New Window", "app.new_window");
                    let quit = MenuItem::new("Quit", "app.quit");

                    submenu_file.append_item(&newwindow);
                    submenu_file.append_item(&quit);

                    let submenu_edit = Menu::new();
                    let copy = MenuItem::new("Copy", "win.copy");
                    let paste = MenuItem::new("Paste", "win.paste");

                    submenu_edit.append_item(&copy);
                    submenu_edit.append_item(&paste);

                    menubar.append_submenu("File", &submenu_file);
                    menubar.append_submenu("Edit", &submenu_edit);

                    app.set_menubar(&menubar);
                }

                create_window(&app);
            });

            app.run(&[""]);
        }
        Err(_) => {
            println!("Application start up error");
        }
    };
}

fn create_window(app: &gtk::Application) -> gtk::ApplicationWindow {
    let win = gtk::ApplicationWindow::new(app);
    win.set_default_size(800, 600);
    win.set_title("Vanilla Text");
    let scr_win = gtk::ScrolledWindow::new(None, None);
    let txt_view = gtk::TextView::new();
    scr_win.add(&txt_view);
    win.add(&scr_win);

    let copy_action = gio::SimpleAction::new("copy", None);
    {
        let txt_view = txt_view.clone();
        copy_action.connect_activate(move |_, _| {
            let clipboard = txt_view.get_clipboard(&gdk::SELECTION_CLIPBOARD);
            txt_view.get_buffer().unwrap().copy_clipboard(&clipboard);
        });
    }

    let paste_action = gio::SimpleAction::new("paste", None);
    {
        let txt_view = txt_view.clone();
        paste_action.connect_activate(move |_, _| {
            let clipboard = txt_view.get_clipboard(&gdk::SELECTION_CLIPBOARD);
            let buf = txt_view.get_buffer().unwrap();
            buf.paste_clipboard(&clipboard, None, txt_view.get_editable());
        });
    }

    win.add_action(&copy_action);
    win.add_action(&paste_action);

    win.show_all();

    win
}
