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

#[tokio::main]  // Changed to use `tokio::main` macro
async fn main() -> std::io::Result<()> {
    let motor_board = Arc::new(Mutex::new(MotorBoard::new("/dev/i2c-1", 0x60)));
    let robot = Arc::new(Robot::new(motor_board.clone()));

    let robot_clone = robot.clone();
    tokio::spawn(async move {
        // Example usage: drive the robot for 1 seconds and then stop it
        robot_clone.drive(1.0, 1.0).await;
        sleep(Duration::from_secs(1)).await;
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
