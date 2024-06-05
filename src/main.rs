mod motor;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use motor::{Motor, MotorBoard};
use tokio::time::{sleep, Duration};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let motor_board = Arc::new(MotorBoard::new("/dev/i2c-1", 0x60));

    let motor_board_clone = motor_board.clone();
    tokio::spawn(async move {
        // Run the motor for 3 seconds and then stop it
        motor_board_clone.set_motor_speed(Motor::Motor1, 2047, true);
        motor_board_clone.set_motor_speed(Motor::Motor2, 2047, true);
        sleep(Duration::from_secs(5)).await;
        motor_board_clone.stop_motor(Motor::Motor1);
        motor_board_clone.stop_motor(Motor::Motor2);
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(motor_board.clone()))
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
