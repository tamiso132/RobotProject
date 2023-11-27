use std::{thread, time::Duration};

use robotproject::protocol::{ptp, queue, FloatCustom, GetPoseR, SuctionCup};

const GRID: [(f32, f32); 24] = [
    (132.8116,-42.881817),
    (178.25412,-47.52761),
    (218.6569,-49.830936),
    (258.44962,-50.9123),
    (131.18314,-9.775582),
    (177.40495,-11.078962),
    (216.07726,-13.393839),
(257.4657,-12.618484),
(133.80498,19.611336),
(180.43544,20.08916),
(215.41925,19.359575),
(259.64697,18.50474),
(140.71704,46.94687),
(180.9229,48.866703),
(218.42476,50.309402),
(250.42827,54.279274),
(130.1948,73.38755),
(168.18134,79.2352),
(208.03143,82.36565),
(247.15314,85.76941),
(116.08774,105.63186),
(153.79672,108.835075),
(194.92029,116.61683),
(234.20761,117.83956)];
const lager_z:f32 =  -43.0;
const BASE_LAGER_POS:(f32, f32, f32) = (200.0, 0.0, 30.0);
// ];

fn get_cell_pos(x: u8, y: u8) -> (FloatCustom, FloatCustom) {
    let index = (y * 4 + x);

    let cell = GRID[index as usize];

    (
        FloatCustom::new(cell.0),
        FloatCustom::new(cell.1),
    )
}

pub fn pick_up_from_conveyor_and_place(fd: i32, procentage: f32) {
    let pos = get_conveyor_y(procentage).unwrap();

    move_robot(fd, pos.0, pos.1, pos.2);

    SuctionCup::send_immediate_command(fd, &1, &1);
    println!("Coming to lagret now");
    move_to_pos_in_grid(fd, 0,0);
    println!("done with lagret");

    SuctionCup::send_immediate_command(fd, &0, &0);
    go_default_lager_pos(fd);
    return;
}
fn go_default_lager_pos(fd:i32){
    move_robot(fd, FloatCustom::new(BASE_LAGER_POS.0),  FloatCustom::new(BASE_LAGER_POS.1),  FloatCustom::new(BASE_LAGER_POS.2))
    
}
pub fn move_to_pos_in_grid(fd: i32, x: u8, y: u8) {
    let cell = get_cell_pos(x, y);
    //go_default_lager_pos(fd);
    move_robot(fd, cell.0, cell.1, FloatCustom::new(lager_z));
    //go_default_lager_pos(fd);
}
//X: -4.9768615, Y: -107.649376, Z: 22.339325, R: -92.64702

const RULLBAND_START: (f32, f32, f32, f32) = (-8.235003,-97.1714, 17.1, 0.0);
const RULLBAND_END: (f32, f32, f32, f32) = (-6.80, -196.0, 17.1, 0.0);

//80

fn get_conveyor_y(procentage: f32) -> Option<(FloatCustom, FloatCustom, FloatCustom, FloatCustom)> {
    if procentage > 1.0 || procentage < 0.0 {
        return None;
    }

    let x = FloatCustom::new(RULLBAND_START.0 - (RULLBAND_START.0 - RULLBAND_END.0) * procentage);
    let y = FloatCustom::new(RULLBAND_START.1 - (RULLBAND_START.1 - RULLBAND_END.1) * procentage);
    let z = FloatCustom::new(RULLBAND_START.2);
    let r = FloatCustom::new(0.0);

    println!("Y: {}", y.to_float());

    Some((x, y, z, r))
    // take picture
}

// walk to rullband // from lager
// first: X: 127.80598, Y: -99.61005, Z: 62.348213, R: -37.932312
// second //  X: 48.42568, Y: -167.83894, Z: 95.0106, R: -73.90584

fn move_robot(fd: i32, x: FloatCustom, y: FloatCustom, z: FloatCustom) {
    let pos = GetPoseR::send_immediate_command(fd).unwrap();

    let step = 10.0;
    queue::StopExec::send_immediate_command(fd);
    queue::ClearExec::send_immediate_command(fd);
    let diff_x_step = (pos.x.to_float() - x.to_float()) / (step as f32);
    let diff_y_step = (pos.y.to_float() - y.to_float()) / (step as f32);
    let diff_r_step = pos.r.to_float()/ step as f32;

    //  println("x: {} - {}, {}", pos.x.to_float(), x.to_float());
    // println("x: {} - {},", pos.y.to_float(), y.to_float());
    for i in 1..(step+1.0) as u32  {

       
        ptp::Cmd::send_queue_command(
            fd,
            &ptp::PTPMode::MovlXYZ,
            &FloatCustom::new(pos.x.to_float() - i as f32 * diff_x_step as f32),
            &FloatCustom::new(pos.y.to_float() - i as f32 * diff_y_step as f32),
            &FloatCustom::new(35.0),
            &FloatCustom::new(0.0),
        );
    }

    let last_index =
        ptp::Cmd::send_queue_command(fd, &ptp::PTPMode::MovlXYZ, &x, &y, &z, &FloatCustom::new(0.0)).unwrap();

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
    queue::StopExec::send_immediate_command(fd);
    queue::ClearExec::send_immediate_command(fd);
}
