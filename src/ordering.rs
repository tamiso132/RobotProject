use std::{
    io::{Read, Write},
    sync::{Arc, Mutex},
};

use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{image::Colory, Position, PositionWithColor};

#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub order_id: u64,
    pub positions: Vec<PositionWithColor>,
}

pub fn read_ordering(
    stream: Arc<Mutex<std::net::TcpStream>>,
    order: Arc<Mutex<Option<Order>>>,
    sort_info: Arc<Mutex<Option<(u8, u8, u8)>>>,
) {
    let mut buffer = String::new();

    stream.lock().unwrap().read_to_string(&mut buffer);

    if !buffer.is_empty() {
        let s: Value = serde_json::from_str(&buffer).unwrap();
        let command_type = s["command"].to_string();
        let command_type_s = command_type.as_str();

        match command_type_s {
            "order" => {
                let order_id = s["order-id"].as_u64().unwrap();
                let positions: Vec<Position> =
                    serde_json::from_str(&s["positions"].to_string()).unwrap();
                *order.lock().unwrap() = Some(Order {
                    order_id,
                    positions,
                });
            }
            "sort_info" => {
                let x = s["x"].as_u64().unwrap();
                let y = s["y"].as_u64().unwrap();
                let color = s["color"].as_u64().unwrap();
                *sort_info.lock().unwrap() = Some((x as u8, y as u8, color as u8));
            }
            _ => {
                panic!("should not come here");
            }
        }
    }

    // read request
    // change
}

pub fn send_sort_request(stream: Arc<Mutex<std::net::TcpStream>>, color: Colory) {
    let json = json!({"command": "sort_request","color": color as u8});
    stream.lock().unwrap().write(json.to_string().as_bytes());
}

pub fn send_order_finished(order_to_send: Order) {}

pub fn send_sort_confirm() {}
