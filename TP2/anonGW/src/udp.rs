use std::{
    io::{Read, Write},
    net::{Shutdown, UdpSocket},
};
use client::Args;
use packet::Packet;

pub fn pick_up_package(args: &Args) -> std::io::Result<()> {
    let listener = UdpSocket::bind("0.0.0.0:6666");
    Ok(())
}

fn handle_client(mut stream: TcpStream, args: &Args) {
    let mut destiny_stream = TcpStream::connect(&args.target_server).expect("Can't Connect");
    let mut r_buffer = [0; 4096];
    let mut w_buffer = [0; 4096];
    stream.set_nonblocking(true).expect("Couldn't set nonblocking");
    loop {
        match stream.read(&mut r_buffer[..]) {
            Ok(a) => {
                if a == 0 {
                    break;
                };
                destiny_stream.write_all(&r_buffer[..a]).expect("Couldn't write to destiny");
            }
            Err(_) => break,
            _ => (),
        };
    };
    stream.set_nonblocking(false).expect("Couldn't set nonblocking");
    loop {
        match destiny_stream.read(&mut w_buffer[..]) {
            Ok(a) => {
                if a == 0 {
                    break;
                };
                println!("{}", a);
                stream.write_all(&w_buffer[..a]).expect("Couldn't write to client");
            },
            Err(a) => { println!("{}", a); break; },
        };
    }
    stream.shutdown(Shutdown::Both).expect("Couldn't close client connection");
    destiny_stream.shutdown(Shutdown::Both).expect("Couldn't close server connection");
}
