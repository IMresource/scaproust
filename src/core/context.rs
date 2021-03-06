// Copyright 2016 Benoît Labaere (benoit.labaere@gmail.com)
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use std::fmt;
use std::io::Result;
use std::time::Duration;

use core::EndpointSpec;
use core::network::Network;

pub trait Context : Network + Scheduler + fmt::Debug {
    fn raise(&mut self, evt: Event);
}

pub trait Scheduler {
    fn schedule(&mut self, schedulable: Schedulable, delay: Duration) -> Result<Scheduled>;
    fn cancel(&mut self, scheduled: Scheduled);
}

pub enum Event {
    CanSend,
    CanRecv,
    Closed
}

pub enum Schedulable {
    Reconnect(EndpointSpec),
    Rebind(EndpointSpec),
    SendTimeout,
    RecvTimeout,
    ReqResend,
    SurveyCancel
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Scheduled(usize);

impl fmt::Debug for Scheduled {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for Scheduled {
    fn from(value: usize) -> Scheduled {
        Scheduled(value)
    }
}

impl Into<usize> for Scheduled {
    fn into(self) -> usize {
        self.0
    }
}

impl<'x> Into<usize> for &'x Scheduled {
    fn into(self) -> usize {
        self.0
    }
}
