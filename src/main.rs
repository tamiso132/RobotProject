use colors_transform::Color;
use image::{get_rectangle_pos_procentage, Rectangle};
use position::pick_up_from_conveyor_and_place;
use robotproject::{
    self,
    cbinding::{self, close_port, read, write},
    protocol::{
        self, homing, ptp, queue,
        sensor::{self, Port},
        EMotor, FloatCustom, GetPoseR, IntCustom, SuctionCup,
    },
};

use serde_derive::{Deserialize, Serialize};
use serde_json::json;

use std::{
    fs::File,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    process::Command,
    sync::{Arc, Mutex},
    thread::{self, Thread},
    time::Duration,
};

mod image;
mod position;

pub fn cal(fd: i32) {
    queue::StopExec::send_immediate_command(fd);
    queue::ClearExec::send_immediate_command(fd);

    homing::Param::send_queue_command(
        fd,
        &FloatCustom::new(100.0),
        &FloatCustom::new(0.0),
        &FloatCustom::new(20.0),
        &FloatCustom::new(0.0),
    );
    let last_index = homing::Cmd::send_queue_command(fd, &0).unwrap();
    let mut curr = queue::CurrentIndex::send_get_command(fd)
        .unwrap()
        .current_index;
    queue::StartExec::send_immediate_command(fd);
    while last_index != curr {
        curr = queue::CurrentIndex::send_get_command(fd)
            .unwrap()
            .current_index;
    }
    println!("done");
}

pub fn sort_objects(fd: i32) {
    EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
    loop {
        thread::sleep(Duration::from_millis(100));
        let state = sensor::get_infrared_state(fd, Port::GP2 as u8);
        if state == 1 {
            EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(0));
            image::take_picture();
            let procentage = image::get_rectangle_pos_procentage();
            // TODO, get position from ordering
            position::pick_up_from_conveyor_and_place(fd, procentage, 0, 0);
            EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
            continue;
        }
    }
}

// pub fn sort_all_objects(fd: i32, mut number: u8) {
//     EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
//     let state = sensor::get_infrared_state(fd, Port::GP2 as u8);
//     if state == 1 {
//         EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(0));
//         image::take_picture();
//         let x = number % 4;
//         let y = number / 4;
//         let procentage = image::get_rectangle_pos_procentage();
//         position::pick_up_from_conveyor_and_place(fd, procentage, x, y);
//         number += 1;
//         EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
//     }
//     if number < 25 {
//         sort_all_objects(fd, number);
//     }
// }

pub fn init(fd: i32) {
    cal(fd);
    sensor::set_infrared_immediate(fd, 1, sensor::Port::GP4);
}
#[derive(Serialize, Deserialize)]
struct Position {
    x: usize,
    y: usize,
}
#[derive(Serialize, Deserialize)]
struct CommandZero {
    command: u8,
    order_id: u16,
    positions: Vec<Position>,
}

pub fn read_request(ss: &str) {}

enum RobotMode {
    SortMode,
    OrderMode,
}

// 3280x2464 pixels
fn main() {
    unsafe {
        let robot_mode = Arc::new(Mutex::new(RobotMode::SortMode));
        let order: Arc<Mutex<Option<CommandZero>>> = Arc::new(Mutex::new(None));

        let fd = cbinding::serial_open();

        sort_objects(fd);

        image::get_rectangle_pos_procentage();

        //cbinding::close_port(fd);
    }
}
