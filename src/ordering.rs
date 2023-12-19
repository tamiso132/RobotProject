use std::{
    io::{Read, Write},
    net::TcpStream,
    os::unix::thread,
    sync::{Arc, Mutex},
};

use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{image::Colory, Position, PositionWithColor};

#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub order_id: u64,
    pub positions: Vec<Position>,
}

struct Command {
    command: String,
    x: i32,
    y: i32,
}

pub fn read_ordering(
    stream: Arc<Mutex<std::net::TcpStream>>,
    order: Arc<Mutex<Option<Order>>>,
    sort_info: Arc<Mutex<Option<(usize, usize, u8)>>>,
    start: Arc<Mutex<bool>>,
) {
    loop {
        let mut buffer = String::new();
        stream.lock().unwrap().read_to_string(&mut buffer);
        if buffer.len() > 0 {
            let s: Value = serde_json::from_str(&buffer).unwrap();
            let command_type = s["command"].to_string();
            let command_type_s = command_type.as_str();
            println!("{}", command_type_s);
            if command_type_s.contains("order") {
                let order_id = s["order-id"].to_string().parse::<u64>().unwrap();
                let positions: Vec<Position> =
                    serde_json::from_str(&s["positions"].to_string()).unwrap();
                *order.lock().unwrap() = Some(Order {
                    order_id,
                    positions,
                });
            }
            if command_type_s.contains("sort_info") {
                println!("{}", s);
                let x = s["x"].to_string().parse::<usize>().unwrap();
                let y = s["y"].to_string().parse::<usize>().unwrap();
                let color = s["color"].to_string().parse::<usize>().unwrap();
                *sort_info.lock().unwrap() = Some((x as usize, y as usize, color as u8));
            }
            if command_type_s.contains("stop") {
                *start.lock().unwrap() = false;
            }
            if command_type_s.contains("start") {
                *start.lock().unwrap() = true;
            }
        }
    }
}

// read request
// change

pub fn send_sort_request(stream: Arc<Mutex<std::net::TcpStream>>, color: Colory) {
    let json = json!({"command": "sort_request","color": color as u8});
    stream
        .lock()
        .unwrap()
        .write(json.to_string().as_bytes())
        .unwrap();
    println!("send sort request");
}

pub fn send_order_finished(order_to_send: Order, stream: Arc<Mutex<TcpStream>>) {
    let json = json!({ "order_id": order_to_send.order_id, "positions": order_to_send.positions});
    stream
        .lock()
        .unwrap()
        .write(json.to_string().as_bytes())
        .unwrap();
}

pub fn send_sort_confirm(x: usize, y: usize, color: u8, stream: Arc<Mutex<TcpStream>>) {
    let json = json!({"command": "sort_confirm", "x": x, "y": y, "color": color});
    stream
        .lock()
        .unwrap()
        .write(json.to_string().as_bytes())
        .unwrap();
}
