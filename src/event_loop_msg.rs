use std::sync::mpsc;

use global::SocketType as SocketType;

pub enum EventLoopCmd {
	Ping,
	CreateSocket(SocketType),
	PingSocket(usize),
	Shutdown
}

pub enum SessionEvt {
	Pong,
	SocketCreated(usize, mpsc::Receiver<SocketEvt>),
	SocketNotCreated
}

pub enum SocketEvt {
    Pong
}
