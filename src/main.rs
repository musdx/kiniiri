use gio::prelude::*;
use gtk::prelude::*;
use gtk4 as gtk;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn activate(application: &gtk::Application) {
    // Create a normal GTK window however you like
    let window = gtk::Window::new();
    window.set_application(Some(application));

    // Before the window is first realized, set it up to be a layer surface
    window.init_layer_shell();

    // Display above normal windows
    window.set_layer(Layer::Overlay);
    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, false),
        (Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }
    let image = gtk::Image::from_file("cat.webp");
    image.set_size_request(200, 200);
    let boxs = gtk::Box::new(gtk::Orientation::Vertical, 10);
    boxs.append(&image);
    window.set_child(Some(&boxs));

    window.show()
}

fn main() {
    let application = gtk::Application::new(Some("blue.likely.kiniiri"), Default::default());

    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}
