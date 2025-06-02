use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;

fn main() {
    // Create a flag for graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set up ctrl-c handler
    ctrlc::set_handler(move || {
        println!("\nShutting down gracefully...");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // Create two channels for bidirectional communication
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let running1 = running.clone();
    let running2 = running.clone();

    // Spawn the first thread
    let handle1 = thread::spawn(move || {
        let mut message_count = 0;
        while running1.load(Ordering::SeqCst) {
            // Send message to thread 2
            message_count += 1;
            if tx1.send(format!("Hello from Thread 1! (message {})", message_count)).is_err() {
                break;
            }
            
            // Wait for response from thread 2
            match rx2.recv_timeout(Duration::from_secs(1)) {
                Ok(received) => println!("Thread 1 received: {}", received),
                Err(_) => {
                    if !running1.load(Ordering::SeqCst) {
                        break;
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(500));
        }
        println!("Thread 1 shutting down...");
    });

    // Spawn the second thread
    let handle2 = thread::spawn(move || {
        let mut message_count = 0;
        while running2.load(Ordering::SeqCst) {
            // Wait for message from thread 1
            match rx1.recv_timeout(Duration::from_secs(1)) {
                Ok(received) => {
                    println!("Thread 2 received: {}", received);
                    
                    // Send response to thread 1
                    message_count += 1;
                    if tx2.send(format!("Hello from Thread 2! (message {})", message_count)).is_err() {
                        break;
                    }
                }
                Err(_) => {
                    if !running2.load(Ordering::SeqCst) {
                        break;
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(500));
        }
        println!("Thread 2 shutting down...");
    });

    // Wait for both threads to complete
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("Program terminated successfully!");
} 