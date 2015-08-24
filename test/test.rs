// Copyright 2015 Copyright (c) 2015 Benoît Labaere (benoit.labaere@gmail.com)
//
// Licensed under the MIT license LICENSE or <http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

#![feature(time)]
#![feature(duration)]

extern crate scaproust;

use std::io;
use std::time;

use scaproust::*;

#[test]
fn test_pipeline_connected_to_bound() {
	let session = Session::new().unwrap();
	let mut pull = session.create_socket(SocketType::Pull).unwrap();
	let mut push = session.create_socket(SocketType::Push).unwrap();

	pull.bind("tcp://127.0.0.1:5454").unwrap();
	push.connect("tcp://127.0.0.1:5454").unwrap();

	let sent = vec![65, 66, 67];
	push.send(sent).unwrap();
	let received = pull.recv().unwrap();

	assert_eq!(vec![65, 66, 67], received)
}

#[test]
fn test_pipeline_bound_to_connected() {
	let session = Session::new().unwrap();
	let mut pull = session.create_socket(SocketType::Pull).unwrap();
	let mut push = session.create_socket(SocketType::Push).unwrap();

	push.bind("tcp://127.0.0.1:5455").unwrap();
	pull.connect("tcp://127.0.0.1:5455").unwrap();

	let sent = vec![65, 66, 67];
	push.send(sent).unwrap();
	let received = pull.recv().unwrap();

	assert_eq!(vec![65, 66, 67], received)
}

#[test]
fn test_send_while_not_connected() {
	let session = Session::new().unwrap();
	let mut push = session.create_socket(SocketType::Push).unwrap();

	push.connect("tcp://127.0.0.1:5456").unwrap();

	let err = push.send(vec![65, 66, 67]).unwrap_err();

	assert_eq!(io::ErrorKind::NotConnected, err.kind());
}

#[test]
fn test_send_timeout() {
	let session = Session::new().unwrap();
	let mut pull = session.create_socket(SocketType::Pull).unwrap();
	let mut push = session.create_socket(SocketType::Push).unwrap();
	let timeout = time::Duration::from_millis(50);

	pull.bind("tcp://127.0.0.1:5457").unwrap();
	push.connect("tcp://127.0.0.1:5457").unwrap();
	push.set_send_timeout(timeout).unwrap();

	let err = push.send(vec![0; 5 * 1024 * 1024]).unwrap_err();

	assert_eq!(io::ErrorKind::TimedOut, err.kind());
}

#[test]
fn test_recv_while_not_connected() {
	let session = Session::new().unwrap();
	let mut pull = session.create_socket(SocketType::Pull).unwrap();

	pull.bind("tcp://127.0.0.1:5458").unwrap();

	let err = pull.recv().unwrap_err();

	assert_eq!(io::ErrorKind::NotConnected, err.kind());
}

#[test]
fn test_recv_timeout() {
	let session = Session::new().unwrap();
	let mut pull = session.create_socket(SocketType::Pull).unwrap();
	let mut push = session.create_socket(SocketType::Push).unwrap();
	let timeout = time::Duration::from_millis(50);

	pull.set_recv_timeout(timeout).unwrap();
	pull.bind("tcp://127.0.0.1:5459").unwrap();
	push.connect("tcp://127.0.0.1:5459").unwrap();

	let err = pull.recv().unwrap_err();

	assert_eq!(io::ErrorKind::TimedOut, err.kind());
}

#[test]
fn test_pair_connected_to_bound() {
	let session = Session::new().unwrap();
	let mut bound = session.create_socket(SocketType::Pair).unwrap();
	let mut connected = session.create_socket(SocketType::Pair).unwrap();

	bound.bind("tcp://127.0.0.1:5460").unwrap();
	connected.connect("tcp://127.0.0.1:5460").unwrap();

	let sent = vec![65, 66, 67];
	connected.send(sent).unwrap();
	let received = bound.recv().unwrap();

	assert_eq!(vec![65, 66, 67], received)
}

#[test]
fn test_pair_bound_to_connected() {
	let session = Session::new().unwrap();
	let mut bound = session.create_socket(SocketType::Pair).unwrap();
	let mut connected = session.create_socket(SocketType::Pair).unwrap();

	bound.bind("tcp://127.0.0.1:5461").unwrap();
	connected.connect("tcp://127.0.0.1:5461").unwrap();

	let sent = vec![65, 66, 67];
	bound.send(sent).unwrap();
	let received = connected.recv().unwrap();

	assert_eq!(vec![65, 66, 67], received)
}

#[test]
fn test_req_rep() {
	let session = Session::new().unwrap();
	let mut server = session.create_socket(SocketType::Rep).unwrap();
	let mut client = session.create_socket(SocketType::Req).unwrap();

	server.bind("tcp://127.0.0.1:5462").unwrap();
	client.connect("tcp://127.0.0.1:5462").unwrap();

	let client_request = vec!(65, 66, 67);
	client.send(client_request).unwrap();

	let server_request = server.recv().unwrap();
	assert_eq!(vec!(65, 66, 67), server_request);

	let server_reply = vec!(67, 66, 65);
	server.send(server_reply).unwrap();

	let client_reply = client.recv().unwrap();

	assert_eq!(vec!(67, 66, 65), client_reply);
}

#[test]
fn test_pub_sub() {
	let session = Session::new().unwrap();
	let mut server = session.create_socket(SocketType::Pub).unwrap();
	let mut client = session.create_socket(SocketType::Sub).unwrap();

	server.bind("tcp://127.0.0.1:5463").unwrap();
	client.connect("tcp://127.0.0.1:5463").unwrap();

	let sent = vec![65, 66, 67];
	server.send(sent).unwrap();
	let received = client.recv().unwrap();

	assert_eq!(vec![65, 66, 67], received);
}

#[test]
fn test_bus() {
	let session = Session::new().unwrap();
	let mut server = session.create_socket(SocketType::Bus).unwrap();
	let mut client1 = session.create_socket(SocketType::Bus).unwrap();
	let mut client2 = session.create_socket(SocketType::Bus).unwrap();

	server.bind("tcp://127.0.0.1:5464").unwrap();
	client1.connect("tcp://127.0.0.1:5464").unwrap();
	client2.connect("tcp://127.0.0.1:5464").unwrap();

	let sent = vec![65, 66, 67];
	server.send(sent).unwrap();
	let received1 = client1.recv().unwrap();
	assert_eq!(vec![65, 66, 67], received1);
	let received2 = client2.recv().unwrap();
	assert_eq!(vec![65, 66, 67], received2);
}

#[test]
fn test_survey() {
	let session = Session::new().unwrap();
	let mut server = session.create_socket(SocketType::Surveyor).unwrap();
	let mut client1 = session.create_socket(SocketType::Respondent).unwrap();
	let mut client2 = session.create_socket(SocketType::Respondent).unwrap();

	server.bind("tcp://127.0.0.1:5465").unwrap();
	client1.connect("tcp://127.0.0.1:5465").unwrap();
	client2.connect("tcp://127.0.0.1:5465").unwrap();

	let server_survey = vec!(65, 66, 67);
	server.send(server_survey).unwrap();

	let client1_survey = client1.recv().unwrap();
	assert_eq!(vec!(65, 66, 67), client1_survey);

	let client2_survey = client2.recv().unwrap();
	assert_eq!(vec!(65, 66, 67), client2_survey);

	client1.send(vec!(65, 66, 65)).unwrap();
	let server_resp1 = server.recv().unwrap();
	assert_eq!(vec!(65, 66, 65), server_resp1);

	client2.send(vec!(67, 66, 67)).unwrap();
	let server_resp2 = server.recv().unwrap();
	assert_eq!(vec!(67, 66, 67), server_resp2);
}

#[test]
fn test_send_reply_before_send_request() {
	let session = Session::new().unwrap();
	let mut server = session.create_socket(SocketType::Rep).unwrap();

	server.bind("tcp://127.0.0.1:5466").unwrap();
	server.send(vec!(67, 66, 65)).unwrap_err();
}

#[test]
fn test_recv_reply_before_send_request() {
	let session = Session::new().unwrap();
	let mut server = session.create_socket(SocketType::Rep).unwrap();
	let mut client = session.create_socket(SocketType::Req).unwrap();

	server.bind("tcp://127.0.0.1:5467").unwrap();
	client.connect("tcp://127.0.0.1:5467").unwrap();

	let err = client.recv().unwrap_err();
	assert_eq!(io::ErrorKind::Other, err.kind());
}

#[test]
fn test_ipc() {
	let session = Session::new().unwrap();
	let mut bound = session.create_socket(SocketType::Pair).unwrap();
	let mut connected = session.create_socket(SocketType::Pair).unwrap();

	bound.bind("ipc:///tmp/test_ipc.ipc").unwrap();
	connected.connect("ipc:///tmp/test_ipc.ipc").unwrap();

	connected.send(vec!(65, 66, 67)).unwrap();
	let received = bound.recv().unwrap();
	assert_eq!(vec![65, 66, 67], received);

	bound.send(vec!(67, 66, 65)).unwrap();
	let received = connected.recv().unwrap();
	assert_eq!(vec!(67, 66, 65), received);
}