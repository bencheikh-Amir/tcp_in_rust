use std::fmt;
use std::io::{self, Write};
use std::net::TcpListener;
use std::result;

const SAFE_MODE: bool = true;
type Result<T> = result::Result<T, io::Error>;

struct Sensitive<T> {
    inner: T,
}

impl<T: fmt::Display> fmt::Display for Sensitive<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if SAFE_MODE {
            writeln!(f, "[REDACTED]")
        } else {
            writeln!(f, "{}", self.inner)
        }
    }
}

fn main() -> Result<()> {
    let address = "127.0.0.1:6969";
    let listener = TcpListener::bind(address).map_err(|err| {
        eprint!("ERROR: Could not bind {}: {}\n", address, err);
        err
    })?;
    println!("Listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let message = Sensitive {
                    inner: "Hallo Mein Führer ;)",
                };

                let _ = writeln!(stream, "{}", message).map_err(|err| {
                    eprintln!("ERROR: Could not write message: {}", err);
                    err
                });
            }
            Err(err) => {
                eprintln!("ERROR: Could not accept connection: {}", err);
            }
        }
    }
    Ok(())
}
