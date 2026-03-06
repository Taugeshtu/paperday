use gtk4::{self as gtk, Orientation};
use gio::prelude::*;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
// use gdk4::Display;
use jiff::{ToSpan, Unit, Zoned};

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn activate(application: &gtk::Application) {
    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);
    
    window.init_layer_shell();
    window.set_layer(Layer::Background);
    
    // Push other windows out of the way
    // window.auto_exclusive_zone_enable();
    
    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    window.set_margin(Edge::Left, 140);
    
    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (Edge::Left, true),
        (Edge::Right, false),
        (Edge::Top, false),
        (Edge::Bottom, false),
    ];
    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }
    
    
    if let Err(_) = fill_window(&window) {
        window.set_child(Some(&gtk::Label::new(Some("Time error"))));
    }
    
    window.show()
}

fn fill_window(window: &gtk::ApplicationWindow) -> Result<(), jiff::Error> {
    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_string("label#month-label { transform: rotate(-90deg); }");
    
    gtk::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    
    window.set_default_size(400, 900);
    
    let content = gtk::ListBox::new();
    
    let today = Zoned::now().round(Unit::Day)?;
    for monthIndex in -1..=1 {
        let shiftedToday = &today+monthIndex.months();
        
        let monthContainer = gtk::Box::new(Orientation::Horizontal, 0);
        
        let monthLabelArea = gtk::DrawingArea::new();
        monthLabelArea.set_size_request(30, -1);
        monthLabelArea.set_vexpand(true);
        
        let month_name = shiftedToday.strftime("%B").to_string();
        monthLabelArea.set_draw_func(move |_area, cr, width, height| {
            let font_desc = gtk4::pango::FontDescription::from_string("Sans 10");
            let pango_context = pangocairo::functions::create_context(cr);
            let layout = gtk4::pango::Layout::new(&pango_context);
            layout.set_font_description(Some(&font_desc));
            layout.set_text(&month_name);
            
            cr.translate(width as f64 / 2.0, height as f64 / 2.0);
            cr.rotate(-std::f64::consts::FRAC_PI_2);
            
            let (text_w, text_h) = layout.pixel_size();
            cr.move_to(
                -(text_w as f64) / 2.0,
                -(text_h as f64) / 2.0,
            );
            
            pangocairo::functions::show_layout(cr, &layout);
        });
        
        monthContainer.append(&monthLabelArea);
        
        let monthRowsContainer = gtk::ListBox::new();
        monthContainer.append(&monthRowsContainer);
        
        let monthRowsCount = if monthIndex == 0 {6} else {4}; /// this needs math
        
        for monthRowIndex in 0..monthRowsCount {
            let monthRow = gtk::Box::new(Orientation::Horizontal, 20);
            for day in 0..7 {
                let dayDisplay = gtk::Inscription::new(Some(&day.to_string()));
                monthRow.append( &dayDisplay );
            }
            monthRowsContainer.append(&monthRow);
        }
        
        content.append(&monthContainer);
    }
    
    window.set_child(Some(&content));
    
    Ok(())
}

fn main() {
    // probably a good idea to call gtk_layer_is_supported () ahead, to decide if we even wanna run
    
    let application = gtk::Application::new(Some("games.tau.paperday"), Default::default());
    
    application.connect_activate(|app| {
        activate(app);
    });
    
    application.run();
}