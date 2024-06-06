mod motor;
mod robot;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use crate::motor::{MotorBoard};
use crate::robot::Robot;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let motor_board = Arc::new(Mutex::new(MotorBoard::new("/dev/i2c-1", 0x60)));
    let robot = Robot::new(motor_board.clone());

    // Example usage: drive the robot for 5 seconds and then stop it
    let robot_clone = robot.clone();
    tokio::spawn(async move {
        robot_clone.drive(1.0, 1.0).await;
        sleep(Duration::from_secs(5)).await;
        robot_clone.drive(-1.0, -1.0).await;
        sleep(Duration::from_secs(5)).await;
        robot_clone.drive(1.0, -1.0).await;
        sleep(Duration::from_secs(5)).await;
        robot_clone.drive(-1.0, 1.0).await;
        sleep(Duration::from_secs(5)).await;
        robot_clone.stop().await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(robot.clone()))
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
