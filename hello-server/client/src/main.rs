use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use sd::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let (sender, receiver) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buf = vec![0; MSG_SIZE];
        match client.read_exact(&mut buf) {
            Ok(_) => {
                let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                println!("Message received: {}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server was severed");
                break;
            }
        }

        match receicer.try_recv() {
            Ok(msg) => {
                let mut buf = msg.clone().into_bytes();
                buf.resize(MSG_SIZE, 0);
                client.write_all(&buf).expect("Write failed");
                println!("Message sent: {}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));

    });

    println!("Write a message: ");
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Read failed");
        let msg = buf.trim().to_string();
        if msg == ":quit" || sender.send(msg).is_err() {
            break;
        }
    }
    println!("Bye!");
}
