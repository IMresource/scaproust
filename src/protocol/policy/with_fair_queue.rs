// Copyright 2016 Benoît Labaere (benoit.labaere@gmail.com)
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use std::io;

use mio;

use protocol::policy::{ Timeout, clear_timeout };
use protocol::policy::priolist::*;
use transport::pipe::Pipe;
use event_loop_msg::{ SocketNotify };
use EventLoop;
use Message;
use super::WithPipes;

pub trait WithFairQueue : WithPipes {
    fn get_fair_queue(&self) -> &PrioList;
    fn get_fair_queue_mut(&mut self) -> &mut PrioList;

    fn insert_into_fair_queue(&mut self, tok: mio::Token, priority: u8) {
        self.get_fair_queue_mut().insert(tok, priority)
    }
    
    fn add_pipe(&mut self, tok: mio::Token, pipe: Pipe) -> io::Result<()> {
        let priority = pipe.get_recv_priority();

        self.insert_into_pipes(tok, pipe).map(|_| self.insert_into_fair_queue(tok, priority))
    }

    fn remove_pipe(&mut self, tok: mio::Token) -> Option<Pipe> {
        self.get_fair_queue_mut().remove(&tok);
        self.get_pipes_mut().remove(&tok)
    }

    fn open_pipe(&mut self, event_loop: &mut EventLoop, tok: mio::Token) {
        self.get_pipe_mut(&tok).map(|p| p.open(event_loop));
    }

    fn on_pipe_opened(&mut self, event_loop: &mut EventLoop, tok: mio::Token) {
        self.get_fair_queue_mut().show(tok);
        self.get_pipe_mut(&tok).map(|p| p.on_open_ack(event_loop));
    }

    fn get_active_pipe(&self) -> Option<&Pipe> {
        match self.get_fair_queue().get() {
            Some(tok) => self.get_pipe(&tok),
            None      => None
        }
    }

    fn get_active_pipe_mut(&mut self) -> Option<&mut Pipe> {
        match self.get_fair_queue().get() {
            Some(tok) => self.get_pipe_mut(&tok),
            None      => None
        }
    }

    fn ready(&mut self, event_loop: &mut EventLoop, tok: mio::Token, events: mio::EventSet) {
        if events.is_readable() {
            self.get_fair_queue_mut().activate(tok);
        } else {
            self.get_fair_queue_mut().deactivate(tok);
        }

        self.get_pipe_mut(&tok).map(|p| p.ready(event_loop, events));
    }

    fn recv(&mut self, event_loop: &mut EventLoop) -> bool {
        self.get_active_pipe_mut().map(|p| p.recv(event_loop)).is_some()
    }

    fn recv_from(&mut self, event_loop: &mut EventLoop) -> Option<mio::Token> {
        self.get_active_pipe_mut().map(|p| {
            p.recv(event_loop);
            p.token()
        })
    }

    fn on_recv_done(&mut self, event_loop: &mut EventLoop, msg: Message, timeout: Timeout) {
        self.send_notify(SocketNotify::MsgRecv(msg));
        self.get_active_pipe_mut().map(|p| p.resync_readiness(event_loop));
        self.get_fair_queue_mut().advance();

        clear_timeout(event_loop, timeout);
    }

    fn on_recv_done_late(&mut self, event_loop: &mut EventLoop, tok: mio::Token) {
        self.get_fair_queue_mut().show(tok);
        self.get_pipe_mut(&tok).map(|p| p.resync_readiness(event_loop));
    }

    fn on_recv_timeout(&mut self) {
        let err = io::Error::new(io::ErrorKind::TimedOut, "recv timeout reached");

        self.send_notify(SocketNotify::MsgNotRecv(err));
        self.get_fair_queue_mut().skip();
    }

    fn can_recv(&self) -> bool {
        match self.get_active_pipe() {
            Some(pipe) => pipe.can_recv(),
            None       => false,
        }
    }
}
