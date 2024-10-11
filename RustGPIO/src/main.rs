use gpio::{GpioIn, GpioOut};
use std::{thread, time, sync::mpsc};
use ctrlc;

fn main() {
    let mut gpio35 = gpio::sysfs::SysFsGpioInput::open(35).expect("Failed to open GPIO23");
    let mut gpio24 = gpio::sysfs::SysFsGpioOutput::open(24).expect("Failed to open GPIO24");

    let (tx, rx) = mpsc::channel();

    // Handle Ctrl+C for graceful shutdown
    ctrlc::set_handler(move || {
        let _ = tx.send(());  // Send shutdown signal
    }).expect("Error setting Ctrl-C handler");

    // Toggle GPIO24 in a background thread
    thread::spawn(move || {
        let mut value = false;
        loop {
            gpio24.set_value(value).expect("Could not set gpio24");
            thread::sleep(time::Duration::from_secs(1));
            value = !value;
        }
    });

    // Main loop to read GPIO23
    loop {
        match gpio35.read_value() {
            Ok(value) => println!("GPIO35: {:?}", value),
            Err(e) => eprintln!("Error reading GPIO35: {}", e),
        }
        thread::sleep(time::Duration::from_millis(100));
        
        // Check for shutdown signal
        if let Ok(_) = rx.try_recv() {
            break;
        }
    }

    println!("Shutting down...");
}
