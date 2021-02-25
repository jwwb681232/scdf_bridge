use std::time::Instant;
use actix::{Actor, AsyncContext, ActorContext, StreamHandler};
use actix_web::{web,Error,HttpRequest, HttpResponse};
use actix_web_actors::ws;
use crate::{HEARTBEAT_INTERVAL, CLIENT_TIMEOUT};
use actix_web_actors::ws::{Message, ProtocolError};

struct MazeWs{
    hb:Instant,
}

impl MazeWs{
    fn new()->Self{
        Self{
            hb: Instant::now(),
        }
    }

    fn hb_start(&self,ctx:&mut <Self as Actor>::Context){
        ctx.run_interval(HEARTBEAT_INTERVAL,|act,ctx|{
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
            }else{
                ctx.ping(b"");
            }
        });
    }
}

impl Actor for MazeWs{
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb_start(ctx);
    }
}

impl StreamHandler<Result<ws::Message,ws::ProtocolError>> for MazeWs{
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text))=>{
                ctx.text(text);
            },
            Ok(ws::Message::Binary(bytes))=>{
                ctx.binary(bytes);
            },
            Ok(ws::Message::Ping(bytes))=>{
                self.hb = Instant::now();
                ctx.pong(&bytes);
            },
            Ok(ws::Message::Pong(_bytes))=>{
                self.hb = Instant::now();
            },
            Ok(ws::Message::Close(close_reason))=>{
                ctx.close(close_reason);
                ctx.stop();
            },
            _=>{
                ctx.stop();
            },
        }
    }
}

pub async fn fitness_start(req:HttpRequest,stream:web::Payload)->Result<HttpResponse,Error>{
    ws::start(MazeWs::new(),&req,stream)
}
