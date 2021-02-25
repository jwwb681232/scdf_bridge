use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use std::collections::{HashMap, HashSet};

#[derive(Message)]
#[rtype(result="()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect{
    pub addr:Recipient<Message>,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct Disconnect{
    pub id:usize,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct MessageData{
    pub id:usize,
    pub msg:String,
}

pub struct DonningServer{
    sessions:HashMap<usize,Recipient<Message>>,
    rng:ThreadRng,
}

impl DonningServer{
    pub fn new()->Self{
        Self{
            sessions: HashMap::new(),
            rng: rand::thread_rng()
        }
    }

    fn send_message(&self,message:&str,skip_id:usize){
        for (id,addr) in self.sessions {
            if *id != skip_id {
                addr.do_send(Message(message.to_owned()));
            }
        }
    }
}

impl Actor for DonningServer{
    type Context = Context<Self>;
}

impl Handler<Connect> for DonningServer{
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
        self.send_message("Someone connected",0);

        let id = self.rng.gen::<usize>();

        self.sessions.insert(id,msg.addr);

        id
    }
}

impl Handler<Disconnect> for DonningServer{
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Context<Self>) -> Self::Result {
        self.sessions.remove(&msg.id);

        self.send_message("Someone disconnected",0);
    }
}
