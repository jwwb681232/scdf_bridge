use std::time::Instant;
use actix::{Actor, AsyncContext, ActorContext, StreamHandler};
use actix_web::{web,Error,HttpRequest, HttpResponse};
use actix_web_actors::ws;
use crate::{HEARTBEAT_INTERVAL, CLIENT_TIMEOUT};
use crate::donning::server;


struct DonningWs {
    hb: Instant,
    addr: Addr<server::DonningServer>,
}

impl Actor for DonningWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb_start(ctx);

        let addr = ctx.address();

    }
}

impl DonningWs {
    fn hb_start(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
            } else {
                ctx.ping(b"");
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for DonningWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                ctx.text(text);
            }
            Ok(ws::Message::Ping(bytes)) => {
                self.hb = Instant::now();
                ctx.pong(&bytes);
            }
            Ok(ws::Message::Pong(_bytes)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bytes)) => {
                ctx.binary(bytes);
            }
            Ok(ws::Message::Close(close_reason)) => {
                ctx.close(close_reason);
                ctx.stop();
            }
            _ => {
                ctx.stop();
            }
        }
    }
}

pub async fn donning_start(req: HttpRequest, stream: web::Payload,srv: web::Data<Addr<server::ChatServer>>,) ->Result<HttpResponse,Error> {
    ws::start(DonningWs{
        hb: Instant::now(),
        addr: srv.get_ref().clone()
    }, &req, stream)
}
