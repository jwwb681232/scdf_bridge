use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

use std::collections::HashMap;


#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageData {
    pub id: usize,
    pub msg: String,
}

#[derive(Debug)]
pub struct FitnessServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rng: ThreadRng,
}


impl FitnessServer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    fn send_message(&self, message: &str, skip_id: usize) {
        for (id, addr) in &self.sessions {
            if *id != skip_id {
                let _ = addr.do_send(Message(message.to_string()));
            }
        }
    }
}

impl Actor for FitnessServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for FitnessServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);
        id
    }
}

impl Handler<Disconnect> for FitnessServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<MessageData> for FitnessServer {
    type Result = ();

    fn handle(&mut self, msg: MessageData, _ctx: &mut Context<Self>) -> Self::Result {
        self.send_message(msg.msg.as_str(), msg.id);
    }
}























