use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

use std::collections::HashMap;

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

#[derive(Debug)]
pub struct DonningServer{
    sessions:HashMap<usize,Recipient<Message>>,
    rng:ThreadRng,
    trainees:Vec<String>,
}

impl DonningServer{
    pub fn new()->Self{
        Self{
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
            trainees:Vec::new(),
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

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) -> Self::Result {
        self.sessions.remove(&msg.id);

        self.send_message("Someone disconnected",msg.id);
    }
}

impl Handler<MessageData> for DonningServer{
    type Result = ();

    fn handle(&mut self, msg: MessageData, _ctx: &mut Self::Context) -> Self::Result {

        let data: serde_json::Value = serde_json::from_str(&msg.msg).unwrap();



        let mut has_trainees_ids:Vec<u64> = Vec::new();



        for trainee in &self.trainees {
            let has_trainee: serde_json::Value = serde_json::from_str(trainee.as_str()).unwrap();
            has_trainees_ids.push(has_trainee["id"].as_u64().unwrap());
        }


        if !has_trainees_ids.contains(&data["data"]["id"].as_u64().unwrap()) {
            self.trainees.push(data["data"].to_string())
        }





        println!("{:?}",self.trainees);

        self.send_message(json!(&self.trainees),msg.id);
    }
}
