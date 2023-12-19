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

use core::panic;
use std::{
    fs::File,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    process::Command,
    sync::{Arc, Mutex},
    thread::{self, Thread},
    time::Duration,
};

use crate::image::take_picture;

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

pub fn sort_objects(
    fd: i32,
    stream: Arc<Mutex<TcpStream>>,
    sort_info: Arc<Mutex<Option<(usize, usize, u8)>>>,
) {
    EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));
    let state = sensor::get_infrared_state(fd, Port::GP2 as u8);
    if state == 1 {
        EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(0));
        image::take_picture();
        let procentage = image::get_rectangle_pos_procentage();
        // TODO, get position from ordering
        ordering::send_sort_request(stream.clone(), procentage.1);
        let pos_color;
        loop {
            if sort_info.lock().unwrap().is_some() {
                pos_color = sort_info.lock().unwrap().clone().unwrap();
                *sort_info.lock().unwrap() = None;
                break;
            }
            println!("it is waiting for value");
            thread::sleep(Duration::from_millis(100));
        }
        position::pick_up_from_conveyor_and_place(fd, procentage.0, pos_color.0, pos_color.1);
        ordering::send_sort_confirm(pos_color.0, pos_color.1, pos_color.2, stream.clone());
        EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(10000));
    }
}

pub fn init(fd: i32) {
    cal(fd);
    sensor::set_infrared_immediate(fd, 1, sensor::Port::GP4);
}
#[derive(Serialize, Deserialize, Clone)]
struct Position {
    position_x: usize,
    position_y: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PositionWithColor {
    pub position_x: usize,
    pub position_y: usize,
    pub product_type_id: u8,
}

fn robot_work(
    fd: i32,
    stream: Arc<Mutex<TcpStream>>,
    order: Arc<Mutex<Option<Order>>>,
    sort_info: Arc<Mutex<Option<(usize, usize, u8)>>>,
    start: Arc<Mutex<bool>>,
) {
    loop {
        if *start.lock().unwrap() {
            if order.lock().unwrap().is_none() {
                sort_objects(fd, stream.clone(), sort_info.clone());
            } else {
                EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(0));
                let order_ = order.lock().unwrap().clone();
                *order.lock().unwrap() = None;
                let order_ = order_.unwrap();
                do_order(fd, order_.positions.clone(), order_.order_id as usize);
                ordering::send_order_finished(order_, stream.clone());
            }
        } else {
            EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(0));
            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn main() {
    unsafe {
        let stream = TcpListener::bind("192.168.88.222:12000").unwrap();
        let stream = stream.accept().unwrap().0;
        stream.set_nonblocking(true).unwrap();

        //Thread safe variables
        let order: Arc<Mutex<Option<Order>>> = Arc::new(Mutex::new(None));
        let sort_info: Arc<Mutex<Option<(usize, usize, u8)>>> = Arc::new(Mutex::new(None));
        let sort_position: Arc<Mutex<Option<PositionWithColor>>> = Arc::new(Mutex::new(None));
        let data = Arc::new(Mutex::new(cbinding::serial_open()));
        let stream: Arc<Mutex<std::net::TcpStream>> = Arc::new(Mutex::new(stream));
        let start = Arc::new(Mutex::new(true));

        // Cloned Thread Safe
        let cloned_data = Arc::clone(&data);
        let cloned_sort = Arc::clone(&sort_info);
        let cloned_sort_2 = Arc::clone(&sort_info);
        let cloned_stream = Arc::clone(&stream);
        let cloned_stream_2 = Arc::clone(&stream);
        let cloned_order = Arc::clone(&order);
        let cloned_start = Arc::clone(&start);

        std::thread::spawn(move || {
            ordering::read_ordering(cloned_stream_2, order, cloned_sort, cloned_start)
        });

        std::thread::spawn(move || {
            let fd = cloned_data.lock().unwrap().clone();
            init(fd);
            robot_work(
                fd.clone(),
                cloned_stream,
                cloned_order,
                cloned_sort_2,
                start,
            );
        });

        loop {}
        cbinding::close_port(*data.lock().unwrap());
    }
}
