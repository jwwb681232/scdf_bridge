use std::time::Instant;
use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use crate::{HEARTBEAT_INTERVAL, CLIENT_TIMEOUT};
use crate::fitness::server;

struct FitnessWs {
    id: usize,
    hb: Instant,
    addr: Addr<server::FitnessServer>,
}


impl Actor for FitnessWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb_start(ctx);

        let addr = ctx.address();

        self.addr
            .send(server::Connect { addr: addr.recipient() })
            .into_actor(self).then(|res, act, ctx| {
            match res {
                Ok(res) => act.id = res,
                _ => ctx.stop(),
            }
            fut::ready(())
        })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl FitnessWs {
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

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for FitnessWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut <Self as Actor>::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_err) => {
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Text(text) => {
                self.addr.do_send(server::MessageData { id: self.id, msg: text });
            }
            ws::Message::Ping(bytes) => {
                self.hb = Instant::now();
                ctx.pong(&bytes);
            }
            ws::Message::Pong(_bytes) => {
                self.hb = Instant::now();
            }
            ws::Message::Binary(bytes) => {
                ctx.binary(bytes);
            }
            ws::Message::Close(close_reason) => {
                ctx.close(close_reason);
                ctx.stop();
            }
            _ => {
                ctx.stop();
            }
        }
    }
}

impl Handler<server::Message> for FitnessWs {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}


pub async fn fitness_start(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::FitnessServer>>
) -> Result<HttpResponse, Error> {
    ws::start(FitnessWs {
        id: 0,
        hb: Instant::now(),
        addr: srv.get_ref().clone(),
    }, &req, stream)
}
