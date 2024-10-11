
use std::time::Duration;
use std::thread;



fn main() {
    // Setup wiringPi in GPIO mode
    let pi = wiringpi::setup_gpio();

    // Set pin 24 as input for the button
    let button = pi.input_pin(24);

    // Track the button state
    let mut last_state = button.digital_read();  // Get the initial state

    loop {
        // Read the current state of the button
        let current_state = button.digital_read();

        // Check if the button was pressed (changed from LOW to HIGH)
        if current_state == wiringpi::pin::Value::High && last_state == wiringpi::pin::Value::Low {
            // Button was pressed, toggle a print statement
            println!("Button pressed! Toggling action...");
        }

        // Update the last state to the current state
        last_state = current_state;

        // Small delay to debounce the button
        thread::sleep(Duration::from_millis(50));
    }
}
