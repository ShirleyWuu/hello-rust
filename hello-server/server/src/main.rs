use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn slepp() {
    thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut clients = vec![];
    let (sender, receiver) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let sender = sender.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || loop {
                let mut buf = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        println!("{}: {:?}", addr, msg);
                        sender.send(msg).expect("Failed to send data to receiver");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Connection with {} was severed", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        if let Ok(msg) = receiver.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buf = msg.clone().into_bytes();
                    buf.resize(MSG_SIZE, 0);
                    client.write_all(&buf).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }
        sleep();
    }
}