#[macro_use] extern crate log;
extern crate env_logger;
extern crate scaproust;

use std::io::*;
use std::time;
use std::thread;

use scaproust::{Session, SocketType, Socket};

const NODE0: &'static str = "node0";
const NODE1: &'static str = "node1";

fn duration_ms(ms: u64) -> time::Duration {
    time::Duration::from_millis(ms)
}

fn sleep_ms(ms: u64) {
    thread::sleep(duration_ms(ms));
}

fn recv_name(socket: &mut Socket, name: &str) {
    match socket.recv() {
        Ok(buffer) => {
            let msg = std::str::from_utf8(&buffer).expect("Failed to parse msg !");
            println!("{}: RECEIVED \"{}\"", name, msg);
        },
        _ => {}
    }
}

fn send_name(socket: &mut Socket, name: &str) {
    println!("{}: SENDING \"{}\"", name, name);
    let buffer = From::from(name.as_bytes());
    socket.send(buffer).expect("Send failed !");
}

fn send_recv(mut socket: Socket, name: &str) -> ! {
    socket.set_recv_timeout(duration_ms(100)).expect("Failed to set ercv timeout !");
    loop {
        recv_name(&mut socket, name);
        sleep_ms(1000);
        send_name(&mut socket, name);
    }
}

fn node0(url: &str) {
    let session = Session::new().expect("Failed to create session !");
    let mut socket = session.create_socket(SocketType::Pair).expect("Failed to create socket !");

    socket.bind(url).expect("Failed to bind socket !");
    send_recv(socket, NODE0);
}

fn node1(url: &str) {
    let session = Session::new().expect("Failed to create session !");
    let mut socket = session.create_socket(SocketType::Pair).expect("Failed to create socket !");

    socket.connect(url).expect("Failed to connect socket !");
    send_recv(socket, NODE1);
}

fn usage(program: &str) -> ! {
    let _ = writeln!(stderr(), "Usage: {} {}|{} <URL> ...", program, NODE0, NODE1);
    std::process::exit(1)
}

fn main() {
    env_logger::init().unwrap();

    let os_args: Vec<_> = std::env::args().collect();
    let args: Vec<&str> = os_args.iter().map(|x| x.as_ref()).collect();
    let program = args[0];

    if args.len() < 2 {
        usage(program);
    }

    match args[1] {
        NODE0 if args.len() == 3 => node0(args[2]),
        NODE1 if args.len() == 3 => node1(args[2]),
        _ => usage(program)
    }
}