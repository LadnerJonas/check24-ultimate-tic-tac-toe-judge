use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

// --- Server Configuration ---
const SERVER_HOST: &str = "feuermagier.com";
const SERVER_PORT: u16 = 7838;

// --- Data Structures for JSON Messages ---
// Use Option<T> to represent nullable fields like in Python's None
#[derive(Serialize, Debug)] // Serialize: Convert Rust struct to JSON
struct ClientMessage {
    last_move: Option<(usize, usize)>, // Tuple (usize, usize) for [u32, u32] array
}

// We don't know the exact structure the server sends back,
// so we'll deserialize into a generic JSON Value, similar to Python's behavior.
// If you knew the response structure, you'd define a struct here
// with #[derive(Deserialize)].
// type ServerResponse = serde_json::Value; // Alias for clarity

// --- Function for a single client's actions ---
fn client_connection(client_id: u32, message_to_send: ClientMessage) {
    let thread_name = thread::current()
        .name()
        .unwrap_or("unknown-thread")
        .to_string();
    let server_addr = format!("{}:{}", SERVER_HOST, SERVER_PORT);

    println!(
        "[{}] Client {}: Attempting to connect to {}...",
        thread_name, client_id, server_addr
    );

    // Result type handles potential errors during connection or I/O
    let connection_result: Result<(), Box<dyn std::error::Error>> = (|| {
        // Create a TCP/IP stream
        // The '?' operator propagates errors early
        let mut stream = TcpStream::connect(&server_addr)?;
        println!("[{}] Client {}: Connected.", thread_name, client_id);

        // Prepare the message (serialize struct to JSON string, then to bytes)
        let json_string = serde_json::to_string(&message_to_send)?;
        let message_bytes = json_string.as_bytes(); // UTF-8 encoding is default

        // Send the message
        println!(
            "[{}] Client {}: Sending: {}",
            thread_name, client_id, json_string
        );
        stream.write_all(message_bytes)?;
        // Ensure data is sent immediately (optional, often handled by OS buffering)
        // stream.flush()?;

        // Wait for and receive a response
        println!(
            "[{}] Client {}: Waiting for server response...",
            thread_name, client_id
        );

        let mut buffer = [0u8; 1024]; // Buffer to read data into
        let bytes_read = stream.read(&mut buffer)?; // Read data from the stream

        if bytes_read > 0 {
            // Attempt to decode bytes as UTF-8 string and then parse JSON
            match std::str::from_utf8(&buffer[..bytes_read]) {
                Ok(response_str) => {
                    match serde_json::from_str::<serde_json::Value>(response_str) {
                        Ok(response_json) => {
                            println!(
                                "[{}] Client {}: Received: {}",
                                thread_name,
                                client_id,
                                // Pretty print the JSON value
                                serde_json::to_string_pretty(&response_json)
                                    .unwrap_or_else(|_| response_str.to_string())
                            );
                        }
                        Err(e) => {
                            // Received valid UTF-8, but it wasn't valid JSON
                            println!(
                                "[{}] Client {}: Received non-JSON response: '{}' (Parse Error: {})",
                                thread_name, client_id, response_str, e
                            );
                        }
                    }
                }
                Err(_) => {
                    // Received data that couldn't be decoded as UTF-8
                    println!(
                        "[{}] Client {}: Received data that couldn't be decoded as UTF-8.",
                        thread_name, client_id
                    );
                }
            }
        } else {
            println!(
                "[{}] Client {}: Received empty response (server closed connection).",
                thread_name, client_id
            );
        }

        // Connection automatically closed when 'stream' goes out of scope (RAII)
        Ok(()) // Indicate success for this block
    })(); // Immediately invoke the closure

    // Handle any error that occurred within the closure
    if let Err(e) = connection_result {
        // Check for specific error kinds if needed
        if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
            match io_err.kind() {
                std::io::ErrorKind::ConnectionRefused => {
                    println!(
                        "[{}] Client {}: Connection refused. Is the server running on {}?",
                        thread_name, client_id, server_addr
                    );
                }
                std::io::ErrorKind::TimedOut => {
                    println!(
                        "[{}] Client {}: Connection timed out.",
                        thread_name, client_id
                    );
                }
                _ => {
                    println!(
                        "[{}] Client {}: An I/O error occurred: {}",
                        thread_name, client_id, io_err
                    );
                }
            }
        } else {
            println!(
                "[{}] Client {}: An unexpected error occurred: {}",
                thread_name, client_id, e
            );
        }
    }

    println!("[{}] Client {}: Connection closed.", thread_name, client_id);
}

// --- Main part of the script ---
pub fn test_clients() {
    println!("--- Starting Test Clients ---");

    // --- Messages to send ---
    let message_client_1 = ClientMessage { last_move: None };
    let message_client_2 = ClientMessage {
        last_move: Some((0, 0)),
    }; // Use Some() for non-null Option

    // Create handles for the threads
    let mut handles = vec![];

    // Spawn thread for client 1
    let handle1 = thread::Builder::new()
        .name("Thread-Client1".to_string())
        .spawn(move || {
            client_connection(1, message_client_1);
        })
        .expect("Failed to spawn client 1 thread");
    handles.push(handle1);

    // Small delay to slightly stagger connections
    thread::sleep(Duration::from_millis(100));

    // Spawn thread for client 2
    let handle2 = thread::Builder::new()
        .name("Thread-Client2".to_string())
        .spawn(move || {
            client_connection(2, message_client_2);
        })
        .expect("Failed to spawn client 2 thread");
    handles.push(handle2);

    // Wait for both threads to complete
    for handle in handles {
        handle.join().expect("Client thread panicked");
    }

    println!("--- Test Clients Finished ---");
}
