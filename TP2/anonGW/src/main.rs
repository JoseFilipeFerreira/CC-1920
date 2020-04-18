use std::thread;
use structopt::StructOpt;

pub mod client;
fn main() -> std::io::Result<()> {
    let args = client::Args::from_args();
    let nargs = args.clone();
    let th = thread::spawn(move || {
        match client::pick_up_client(&nargs) {
            Ok(a) => a,
            Err(b) => println!("{}", b),
        };
    });
    th.join();

    println!("{:?}", &args);
    Ok(())
}
