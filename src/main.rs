extern crate gtk;
extern crate gio;
extern crate stopwatch;

use std::time::Duration;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use stopwatch::{Stopwatch};
use std::{time};


fn main() {
    // GTK+ Initialization (No need to reload)
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("Failed to initialize GTK application");

    
    

// Actual app
    application.connect_activate(|app| {
        
        let sw = Stopwatch::start_new();
        let clock_update = time::Duration::from_secs_f64(0.1);


        // Main app window
        let window = ApplicationWindow::new(app);
        // Window title
        window.set_title("Game Time");
        // Window size (px)
        window.set_default_size(300, 600);


        let button = Button::with_label("Start Stopwatch");
        
        button.connect_clicked( move |_| {
            
            loop {
                println!("Your time: {:?}ms", sw.elapsed());
                sleep(clock_update);

            }

        });

        // stop button go here?

        // Calls widgets
        window.add(&button);

        // Makes all widgets visible
        window.show_all();
    });
    
    application.run(&[]);
}

// Alt Functions

fn sleep(x: Duration) {
    std::thread::sleep(x);
}
        

        
