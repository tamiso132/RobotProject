use std::{thread, time::Duration};

use robotproject::protocol::{ptp, queue, FloatCustom, GetPoseR};

const GRID: [(f32, f32, f32, f32); 24] = [
    (120.20642, -85.481865, -40.303055, -35.417606),
    (154.76396, -100.03103, -44.133507, -32.87643),
    (184.9903, -116.56075, -43.261543, -32.214664),
    (214.94167, -126.31306, -44.42035, -30.441135),
    (130.9671, -65.13826, -41.873077, -26.444077),
    (161.24797, -77.43004, -43.5082, -25.64996),
    (193.32906, -90.102325, -41.652092, -24.988194),
    (224.79176, -100.12502, -44.228027, -24.008781),
    (139.17743, -40.853374, -43.09153, -16.358782),
    (168.79364, -52.098347, -41.102554, -17.1529),
    (202.9993, -65.33675, -42.45054, -17.841135),
    (202.9993, -65.33675, -42.45054, -17.841135),
    (150.46698, -18.514116, -42.915535, -7.0146646),
    (184.4296, -28.599546, -45.358208, -8.814665),
    (218.01868, -38.25781, -45.29129, -9.9529),
    (248.21432, -49.015182, -43.225224, -11.1705475),
    (161.13551, 3.4995544, -45.47975, 1.2441589),
    (194.09102, -6.189176, -45.30606, -1.8264294),
    (227.43098, -16.6309, -43.19778, -4.1823115),
    (259.33014, -21.494867, -45.36095, -4.738194),
    (162.13339, 29.146744, -44.345734, 10.191217),
    (190.82622, 19.55182, -44.55007, 5.8500414),
    (224.13536, 13.373989, -44.890713, 3.414747),
    (258.1475, 3.5783308, -42.931282, 0.7941588),
];

fn get_cell_pos(x: u8, y: u8) -> (FloatCustom, FloatCustom, FloatCustom, FloatCustom) {
    let index = ((y - 1) * 4 + x) - 1;

    let cell = GRID[index as usize];

    (
        FloatCustom::new(cell.0),
        FloatCustom::new(cell.1),
        FloatCustom::new(cell.2),
        FloatCustom::new(cell.3),
    )
}

pub fn pick_up_from_conveyor(fd: i32, procentage: f32) {
    let pos = get_conveyor_y(procentage).unwrap();

    move_robot(fd, pos.0, pos.1, pos.2, pos.3);
}

pub fn move_to_pos_in_grid(fd: i32, x: u8, y: u8) {
    let cell = get_cell_pos(x, y);
    GetPoseR::send_immediate_command(fd);
    println!("stuff");
    queue::StopExec::send_immediate_command(fd);
    queue::ClearExec::send_immediate_command(fd);

    println!("Clear");
    let pos = GetPoseR::send_immediate_command(fd).unwrap();
    let mut curr = queue::CurrentIndex::send_get_command(fd)
        .unwrap()
        .current_index;

    println!("before start: {}", curr);

    ptp::Cmd::send_queue_command(fd, &ptp::PTPMode::MovlXYZ, &cell.0, &pos.y, &pos.z, &pos.r);

    ptp::Cmd::send_queue_command(fd, &ptp::PTPMode::MovlXYZ, &cell.0, &cell.1, &pos.z, &pos.r);

    ptp::Cmd::send_queue_command(
        fd,
        &ptp::PTPMode::MovlXYZ,
        &cell.0,
        &cell.1,
        &pos.z,
        &cell.3,
    );
    ptp::Cmd::send_queue_command(
        fd,
        &ptp::PTPMode::MovlXYZ,
        &cell.0,
        &cell.1,
        &cell.2,
        &pos.r,
    );

    let last_index = ptp::Cmd::send_queue_command(
        fd,
        &ptp::PTPMode::MovlXYZ,
        &cell.0,
        &cell.1,
        &cell.2,
        &cell.3,
    )
    .unwrap();

    queue::StartExec::send_immediate_command(fd);
    curr = queue::CurrentIndex::send_get_command(fd)
        .unwrap()
        .current_index;
    while last_index != curr {
        thread::sleep(Duration::from_millis(100));
        curr = queue::CurrentIndex::send_get_command(fd)
            .unwrap()
            .current_index;

        println!("last index: {}, current index: {}", last_index, curr);
    }
    queue::StopExec::send_immediate_command(fd);
}
// X:X: -3.374155, Y: -100.01023, Z: 21.952965, R: -91.93231
const RULLBAND_START: (f32, f32, f32, f32) = (-3.374155, -100.0, -22.0, -92.0);
const RULLBAND_END: (f32, f32, f32, f32) = (-3.374155, -180.0, -22.0, -92.0);

fn get_conveyor_y(procentage: f32) -> Option<(FloatCustom, FloatCustom, FloatCustom, FloatCustom)> {
    if procentage > 1.0 || procentage < 0.0 {
        return None;
    }

    let x = FloatCustom::new(RULLBAND_START.0);
    let y = FloatCustom::new((RULLBAND_END.1 - RULLBAND_START.1) * procentage);
    let z = FloatCustom::new(RULLBAND_START.2);
    let r = FloatCustom::new(RULLBAND_START.3);

    Some((x, y, z, r))
    // take picture
}

// walk to rullband // from lager
// first: X: 127.80598, Y: -99.61005, Z: 62.348213, R: -37.932312
// second //  X: 48.42568, Y: -167.83894, Z: 95.0106, R: -73.90584

fn move_robot(fd: i32, x: FloatCustom, y: FloatCustom, z: FloatCustom, r: FloatCustom) {
    let pos = GetPoseR::send_immediate_command(fd).unwrap();

    queue::StopExec::send_immediate_command(fd);
    queue::ClearExec::send_immediate_command(fd);

    ptp::Cmd::send_queue_command(fd, &ptp::PTPMode::MovlXYZ, &x, &y, &pos.z, &r);


    let last_index =
        ptp::Cmd::send_queue_command(fd, &ptp::PTPMode::MovlXYZ, &x, &y, &z, &r).unwrap();

    let mut curr = queue::CurrentIndex::send_get_command(fd)
        .unwrap()
        .current_index;
    queue::StartExec::send_immediate_command(fd);
    while last_index != curr {
        thread::sleep(Duration::from_millis(100));
        curr = queue::CurrentIndex::send_get_command(fd)
            .unwrap()
            .current_index;
    }
}
