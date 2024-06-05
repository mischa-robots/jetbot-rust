use std::sync::Arc;
use crate::motor::{Motor, MotorBoard};
use tokio::sync::Mutex;

pub struct Robot {
    motor_board: Arc<Mutex<MotorBoard>>,
}

impl Robot {
    pub fn new(motor_board: Arc<Mutex<MotorBoard>>) -> Self {
        Robot { motor_board }
    }

    pub async fn drive(&self, left: f32, right: f32) {  // Renamed from `move` to `drive`
        let motor_board = self.motor_board.lock().await;

        let left_speed = (left * 2047.0).round() as u16;
        let right_speed = (right * 2047.0).round() as u16;

        if left > 0.0 {
            motor_board.set_motor_speed(Motor::Motor1, left_speed, true);
        } else if left < 0.0 {
            motor_board.set_motor_speed(Motor::Motor1, left_speed, false);
        } else {
            motor_board.stop_motor(Motor::Motor1);
        }

        if right > 0.0 {
            motor_board.set_motor_speed(Motor::Motor2, right_speed, true);
        } else if right < 0.0 {
            motor_board.set_motor_speed(Motor::Motor2, right_speed, false);
        } else {
            motor_board.stop_motor(Motor::Motor2);
        }
    }

    pub async fn stop(&self) {
        let motor_board = self.motor_board.lock().await;
        motor_board.stop_motor(Motor::Motor1);
        motor_board.stop_motor(Motor::Motor2);
    }
}
