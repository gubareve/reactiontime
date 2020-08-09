use termion;
use termion::input::TermRead;
use std::io;
use termion::raw::IntoRawMode;
use std::time::Instant;
use std::thread;
use std::time;

fn main() {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    println!("Get ready!\r");
    thread::sleep(time::Duration::from_millis(3000));
    println!(" --- PRESS NOW ---\r");

    let start = Instant::now();

    stdin.next();

    loop {
        // Read input (if any)
        let input = stdin.next();


        // If a key was pressed
        if let Some(Ok(key)) = input {
            break;    
        }
    }

    let elapsed = start.elapsed();
    println!("It took you {:?}\r", elapsed);
}

