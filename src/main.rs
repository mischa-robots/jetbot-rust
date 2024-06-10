mod motor;
mod robot;

use actix_files::Files;
use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use serde::Deserialize;
use std::sync::Arc;
use crate::motor::MotorBoard;
use crate::robot::Robot;
use tokio::sync::Mutex;
use std::path::PathBuf;
use serde_json;
use actix::prelude::*;  // Corrected import

#[derive(Deserialize)]
struct DriveParams {
    left: f32,
    right: f32,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

async fn drive_robot(robot: web::Data<Arc<Robot>>, params: web::Query<DriveParams>) -> impl Responder {
    let left = params.left;
    let right = params.right;
    robot.drive(left, right).await;
    HttpResponse::Ok().body("Driving") 
}

async fn index() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

struct RobotControl {
    robot: Arc<Robot>,
}

impl RobotControl {
    fn new(robot: Arc<Robot>) -> Self {
        Self { robot }
    }
}

impl Actor for RobotControl {
    type Context = ws::WebsocketContext<Self>;

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let robot = self.robot.clone();
        actix_rt::spawn(async move {
            robot.stop().await;
        });
    }
}

#[derive(Deserialize)]
struct ControlMessage {
    left: f32,
    right: f32,
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RobotControl {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(control_msg) = serde_json::from_str::<ControlMessage>(&text) {
                    let robot = self.robot.clone();
                    actix_rt::spawn(async move {
                        robot.drive(control_msg.left, control_msg.right).await;
                    });
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_reason)) => {
                let robot = self.robot.clone();
                actix_rt::spawn(async move {
                    robot.stop().await;
                });
                ctx.stop(); // Close the WebSocket connection
            }
            _ => (),
        }
    }
}

async fn robot_ws(
    req: HttpRequest,
    stream: web::Payload,
    robot: web::Data<Arc<Robot>>,
) -> Result<HttpResponse, actix_web::Error> {
    let robot_control = RobotControl::new(robot.get_ref().clone());
    let res = ws::start(robot_control, &req, stream)?;
    Ok(res)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let motor_board = Arc::new(Mutex::new(MotorBoard::new("/dev/i2c-1", 0x60)));
    let robot = Robot::new(motor_board.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(robot.clone()))
            .route("/health", web::get().to(health_check))
            .route("/drive", web::get().to(drive_robot))
            .route("/", web::get().to(index))
            .route("/ws", web::get().to(robot_ws))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("0.0.0.0:8000")?  // Bind to all network interfaces
    .run()
    .await
}
