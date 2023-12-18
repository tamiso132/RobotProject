use colors_transform::Color;
use image::{get_rectangle_pos_procentage, Rectangle};
use ordering::Order;
use position::{do_order, pick_up_from_conveyor_and_place};
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
mod ordering;
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

pub fn sort_objects(fd: i32, stream: Arc<Mutex<TcpStream>>) {
    EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
    let state = sensor::get_infrared_state(fd, Port::GP2 as u8);
    if state == 1 {
        EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(0));
        image::take_picture();
        let procentage = image::get_rectangle_pos_procentage();
        // TODO, get position from ordering
        ordering::send_sort_request(stream, procentage.1);
        loop {}
        position::pick_up_from_conveyor_and_place(fd, procentage, 0, 0);
        EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
    }
}

pub fn sort_all_objects(fd: i32, mut number: u8) {
    EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
    let state = sensor::get_infrared_state(fd, Port::GP2 as u8);
    if state == 1 {
        EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(0));
        image::take_picture();
        let x = number % 4;
        let y = number / 4;
        let procentage = image::get_rectangle_pos_procentage();
        position::pick_up_from_conveyor_and_place(fd, procentage, x, y);
        number += 1;
        EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
    }
    if number < 25 {
        sort_all_objects(fd, number);
    }
}

pub fn init(fd: i32) {
    cal(fd);
    sensor::set_infrared_immediate(fd, 1, sensor::Port::GP4);
}
#[derive(Serialize, Deserialize, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Serialize, Deserialize, Clone)]
struct PositionWithColor {
    x: usize,
    y: usize,
    color: u8,
}

pub fn read_request(ss: &str) {}

#[derive(Clone, Copy)]
enum RobotMode {
    SortMode,
    OrderMode,
}

fn robot_work(fd: i32, stream: Arc<Mutex<TcpStream>>, order: Arc<Mutex<Option<Order>>>) {
    if order.lock().unwrap().is_none() {
        sort_objects(fd, stream);
    } else {
        EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(0));
        let order_ = order.lock().unwrap().clone();
        *order.lock().unwrap() = None;
        let order_ = order_.unwrap();
        do_order(fd, order_.positions, order_.order_id as usize);
    }
}

const IP_ADRESS_ORDER: &str = "192.168.88.71";

fn main() {
    unsafe {
        let order: Arc<Mutex<Option<Order>>> = Arc::new(Mutex::new(None));
        let sort_position: Arc<Mutex<Option<Position>>> = Arc::new(Mutex::new(None));

        let mut stream = TcpStream::connect(IP_ADRESS_ORDER).unwrap();
        stream.set_nonblocking(true).unwrap();

        let stream: Arc<Mutex<std::net::TcpStream>> = Arc::new(Mutex::new(stream));

        robot_work(fd, stream, order);

        let fd = cbinding::serial_open();
        let sort_info: Arc<Mutex<Option<(u8, u8, u8)>>> = Arc::new(Mutex::new(None));
        //init(fd);
        let pos = vec![
            Position { x: 0, y: 2 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 2 },
            Position { x: 3, y: 2 },
        ];
        //init(fd);
        // robot_work(fd, order.clone());
        // ordering::read_ordering(stream, order.clone(), sort_position.clone());
        // // init(fd);
        // // image::take_picture();
        // // image::get_rectangle_pos_procentage();
        // sort_all_objects(fd, 0);

        let pos = GetPoseR::send_immediate_command(fd).unwrap();

        let x = pos.x.to_float();
        let y = pos.y.to_float();
        let z = pos.z.to_float();

        println!("X: {}, Y: {}, Z: {}", x, y, z);

        cbinding::close_port(fd);
    }
}
