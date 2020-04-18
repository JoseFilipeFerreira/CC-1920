use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(rename_all = "kebab-case")]
pub struct Args {
    #[structopt(short, long)]
    pub target_server: String,
    #[structopt(short, long)]
    pub overlay_peers: Option<Vec<String>>,
    #[structopt(short = "hl", long, default_value = "3")]
    pub hop_limit: u32,
}

pub fn pick_up_client(args: &Args) -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:80")?;
    for stream in listener.incoming() {
        println!("new client");
        let stram = stream?;
        let cargs = args.clone();
        std::thread::spawn(move || handle_client(stram, &cargs))
            .join()
            .expect("Couldn't talk to client");
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, args: &Args) {
    let mut destiny_stream = TcpStream::connect(&args.target_server).expect("Can't Connect");
    let mut r_buffer = [0; 4096];
    let mut w_buffer = [0; 4096];
    stream
        .set_nonblocking(true)
        .expect("Couldn't set nonblocking");
    loop {
        match stream.read(&mut r_buffer[..]) {
            Ok(a) => {
                if a == 0 {
                    break;
                };
                destiny_stream
                    .write_all(&r_buffer[..a])
                    .expect("Couldn't write to destiny");
            }
            Err(_) => break,
            _ => (),
        };
    }
    stream
        .set_nonblocking(false)
        .expect("Couldn't set nonblocking");
    loop {
        match destiny_stream.read(&mut w_buffer[..]) {
            Ok(a) => {
                if a == 0 {
                    break;
                };
                println!("{}", a);
                stream
                    .write_all(&w_buffer[..a])
                    .expect("Couldn't write to client");
            }
            Err(a) => {
                println!("{}", a);
                break;
            }
        };
    }
    stream
        .shutdown(Shutdown::Both)
        .expect("Couldn't close client connection");
    destiny_stream
        .shutdown(Shutdown::Both)
        .expect("Couldn't close server connection");
}
