use colors_transform::Color;
use image::{
    imageops, math::Rect, DynamicImage, GenericImageView, Pixel, Rgb, RgbImage, Rgba, RgbaImage,
};
use robotproject::{
    self,
    cbinding::{self, close_port, read, write},
    protocol::{self, homing, ptp, queue, sensor, FloatCustom, GetPoseR, IntCustom, SuctionCup},
};
use std::{
    cmp,
    ffi::CString,
    fs,
    process::Command,
    thread::{self, Thread},
    time::Duration,
};

type XStart = u32;
type YStart = u32;
type Width = u32;

pub struct Rectangle {
    pub x_pos: u32,
    pub y_pos: u32,
    pub width: u32,
    pub height: u32,
    pub color: Colory,
}

impl Rectangle {
    fn in_bound(&self, x: u32, y: u32, threshold: u32) -> bool {
        let is_x = x + threshold >= self.x_pos && x - threshold <= self.x_pos + self.width;
        let is_y = y + threshold >= self.y_pos && y - threshold <= self.y_pos + self.height;

        is_x && is_y
    }
    fn print_to_screen(&self, image: &mut RgbImage, rgb: Rgb<u8>) {
        for x in self.x_pos..self.width + self.x_pos {
            for y in self.y_pos..self.height + self.y_pos {
                image.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }
}

pub fn take_picture() {
    let output = Command::new("libcamera-jpeg")
        .arg("-o")
        .arg("/home/tom/projects/RobotProject/src/ty.jpg")
        .arg("--width 450")
        .arg("--height 150")
        .arg("--brightness 0.3")
        .output()
        .expect("Failed to execute libcamera-still command");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn is_color_equal(
    curr_hsl: [f32; 3],
    hue_range: (u16, u16),
    sat_range: (u8, u8),
    light_range: (u8, u8),
) -> bool {
    let curr_hue = curr_hsl[0] as u16;
    let curr_sat = curr_hsl[1] as u8;
    let curr_light = curr_hsl[2] as u8;

    let is_hue_range = curr_hue >= hue_range.0 && curr_hue <= hue_range.1;
    let is_sat_range = curr_sat >= sat_range.0 && curr_sat <= sat_range.1;
    let is_light_range = curr_light >= light_range.0 && curr_light <= light_range.1;

    is_hue_range && is_light_range && is_sat_range
}

fn rgb_to_hsl(rgb: [u8; 3]) -> [f32; 3] {
    // let r = rgb[0] as f32 / 255.0;
    // let g = rgb[1] as f32 / 255.0;
    // let b = rgb[2] as f32 / 255.0;

    // let cmax = r.max(g).max(b);
    // let cmin = r.min(g).min(b);
    // let delta = cmax - cmin;

    // let hue = if delta == 0.0 {
    //     0.0
    // } else if cmax == r {
    //     60.0 * (((g - b) / delta) % 6.0)
    // } else if cmax == g {
    //     60.0 * (((b - r) / delta) + 2.0)
    // } else {
    //     60.0 * (((r - g) / delta) + 4.0)
    // };

    // let saturation = if cmax == 0.0 { 0.0 } else { delta / cmax };
    // let value = cmax;
    let hsl = colors_transform::Rgb::from(rgb[0] as f32, rgb[1] as f32, rgb[2] as f32).to_hsl();

    [hsl.get_hue(), hsl.get_saturation(), hsl.get_lightness()]
}

fn extract_color_pixels(input_path: &str, output_path: &str, brightness_factor: f32) {
    // Load the image
    let img = image::open(input_path).expect("Failed to open image");

    //img = img.resize(1000, , imageops::FilterType::Nearest);

    // Create an output image with the same dimensions
    let mut output_img = RgbImage::new(img.width(), img.height());

    // Define the updated HSV range for green
    // Define the updated HSV range for green

    let black = Rgb([0 as u8, 0 as u8, 0 as u8]);
    let white = Rgb([255 as u8, 255 as u8, 255 as u8]);

    let width = img.width();
    let height = img.height();

    let start_x = 125;
    let end_x = img.width() - 100;

    let start_y = 215;
    let end_y = 305;

    let pixels = img.to_rgb8();

    let mut rectangles: Vec<Rectangle> = vec![];

    for y in start_y..end_y {
        for x in start_x..end_x {
            let mut in_bound = false;
            for r in &rectangles {
                if r.in_bound(x, y, 20) {
                    in_bound = true;
                    break;
                }
            }

            if in_bound == true {
                continue;
            }

            let pixel = pixels[(x, y)];
            let hsl = rgb_to_hsl([pixel[0], pixel[1], pixel[2]]);

            let check_yellow = is_color_equal(hsl, (30, 70), (20, 100), (0, 80));

            let check_green = is_color_equal(hsl, (90, 160), (30, 50), (0, 70));

            let check_blue = is_color_equal(hsl, (210, 230), (45, 100), (13, 100));

            let check_red = is_color_equal(hsl, (330, 359), (20, 100), (0, 50));

            if check_red {
                // output_img.put_pixel(x, y, pixel);
                let rectangle = get_object(&pixels, x, y, Colory::Red, &mut output_img);
                if rectangle.is_some() {
                    println!("{},{}", x, y);
                    rectangles.push(rectangle.unwrap());
                    continue;
                }
            }
            // } else if check_blue {
            //     let rectangle = get_object(&pixels, x, y, Colory::Blue);
            //     // if rectangle.is_some() {
            //     //     rectangles.push(rectangle.unwrap());
            //     // }
            // }
            if check_yellow {
                //  output_img.put_pixel(x, y, pixel);
                let rectangle = get_object(&pixels, x, y, Colory::Yellow, &mut output_img);
                if rectangle.is_some() {
                    rectangles.push(rectangle.unwrap());
                    continue;
                }
                //  output_img.put_pixel(x, y, Rgb([255, 0, 0]));
            }
            // } else if check_blue {
            //     // let rectangle = get_object(&pixels, x, y, Colory::Blue);
            //     // if rectangle.is_some() {
            //     //     rectangles.push(rectangle.unwrap());
            //     // }
            // } else if check_green {
            //     // let rectangle = get_object(&pixels, x, y, Colory::Green);
            //     // if rectangle.is_some() {
            //     //     rectangles.push(rectangle.unwrap());
            //     // }
            // }
        }
    }

    for r in &rectangles {
        println!(
            "X: {}, Y: {}, Width: {}, Height {}",
            r.x_pos, r.y_pos, r.width, r.height
        );
        r.print_to_screen(&mut output_img, white)
    }
    // rectangles[1].print_to_screen(&mut output_img, Rgb([0, 255, 0]));
    // rectangles[1].print_to_screen(&mut output_img, white);
    // rectangles[1].print_to_screen(&mut output_img, Rgb([50, 50, 50]));

    // Save the output image
    output_img.save("hello.jpg").expect("Failed to save image");
}

fn check_color(rgb: [u8; 3], hue: (u16, u16), sat: (u8, u8), light: (u8, u8)) -> bool {
    let hsl = rgb_to_hsl(rgb);

    is_color_equal(hsl, hue, sat, light)
}

pub enum Colory {
    Red,
    Yellow,
    Blue,
    Green,
}

fn get_object(
    pixels: &RgbImage,
    start_x: u32,
    start_y: u32,
    color: Colory,
    output: &mut RgbImage,
) -> Option<Rectangle> {
    let mut curr_width_forward = 0;
    let mut curr_width_backward = 0;

    // let mut big_f = 0;
    // let mut big_b = 0;
    let hue_range;
    let sat_range;
    let light_range;

    //     let check_yellow = check_color(hsl, (45, 65), (40, 100), (0, 60));

    //     let check_green = check_color(hsl, (90, 160), (30, 100), (0, 70));

    //     let check_blue = check_color(hsl, (200, 240), (40, 100), (10, 50));

    match color {
        Colory::Red => {
            hue_range = (330, 359);
            sat_range = (20, 100);
            light_range = (0, 50);
        }
        Colory::Yellow => {
            hue_range = (45, 65);
            sat_range = (40, 100);
            light_range = (0, 60);
        }
        Colory::Blue => {
            hue_range = (200, 240);
            sat_range = (40, 100);
            light_range = (10, 50);
        }
        Colory::Green => {
            hue_range = (90, 160);
            sat_range = (30, 100);
            light_range = (0, 70);
        }
    }

    let mut y_test = start_y;
    loop {
        let pixel_forward = pixels[(curr_width_forward + start_x, y_test)];
        let pixel_back = pixels[(start_x - curr_width_backward, y_test)];

        let check_forward = check_color(pixel_forward.0, hue_range, sat_range, light_range);
        let check_backward = check_color(pixel_back.0, hue_range, sat_range, light_range);

        if check_forward {
            curr_width_forward += 1;
        }
        if check_backward {
            curr_width_backward += 1;
        }

        if !check_forward && !check_backward {
            y_test += 1;
            let mut move_down = false;
            for i in 0..5 {
                let pixel_now = pixels[(start_x, y_test + i)];
                let hsl_yep = rgb_to_hsl([pixel_now[0], pixel_now[1], pixel_now[2]]);
                let m = is_color_equal(hsl_yep, hue_range, sat_range, light_range);
                if m == true {
                    move_down = true;
                    break;
                }
            }
            if move_down {
            } else {
                break;
            }
        }
    }

    if curr_width_backward + curr_width_forward < 20 {
        return None;
    }
    let x_pos = start_x - curr_width_backward;
    let the_width = curr_width_forward + curr_width_backward;

    let mut height_down = 0;
    let mut height_up = 0;

    let free_down = 15;

    let mut x_test = 0;
    loop {
        let pixel_down_right = pixels[(start_x + x_test, start_y + height_down + free_down)];
        let pixel_up_right = pixels[(start_x + x_test, start_y - height_up)];

        let pixel_up_left = pixels[(start_x - x_test, start_y - height_up)];
        let pixel_down_left = pixels[(start_x - x_test, start_y + height_down + free_down)];

        let is_down_right = check_color(pixel_down_right.0, hue_range, sat_range, light_range);
        let is_up_right = check_color(pixel_up_right.0, hue_range, sat_range, light_range);

        let is_up_left = check_color(pixel_up_left.0, hue_range, sat_range, light_range);
        let is_down_left = check_color(pixel_down_left.0, hue_range, sat_range, light_range);

        if is_up_left || is_up_right {
            height_up += 5;
        }

        if is_down_left || is_down_right {
            height_down += 5;
        }
        // let hsl_forward = rgb_to_hsl([pixel_forward[0], pixel_forward[1], pixel_forward[2]]);
        // let hsl_backward = rgb_to_hsl([pixel_back[0], pixel_back[1], pixel_back[2]]);

        // let check_f = check_color(hsl_forward, hue_range, sat_range, light_range);
        // let check_b = check_color(hsl_backward, hue_range, sat_range, light_range);

        // if check_f {
        //     curr_width_forward += 1;
        // }
        // if check_b {
        //     curr_width_backward += 1;
        // }

        if !is_up_left || !is_up_right || !is_down_left || !is_down_right {
            x_test += 1;
            let mut move_left = false;
            for i in 0..5 {
                let pixel_right = pixels[(start_x + x_test + i, start_y + free_down)];
                let pixel_left = pixels[(start_x - x_test - i, start_y + free_down)];

                let m = check_color(pixel_right.0, hue_range, sat_range, light_range)
                    || check_color(pixel_left.0, hue_range, sat_range, light_range);

                if m == true {
                    move_left = true;
                    break;
                }
            }
            if move_left {
                continue;
            } else {
                break;
            }
        }
    }
    let y_pos = start_y - height_up;

    if height_down < 20 {
        return None;
    }

    let the_height = height_up + height_down + free_down;
    // output.put_pixel(x_pos, start_y, Rgb([0, 255, 0]));
    //    output.save("yepperssss.jpg").unwrap();

    // for i in 0..the_height { // maybe needed, we will see. if my width get wierd.
    //     loop {
    //         let pixel_back = pixels[(start_x - curr_width_backward, y_pos + i)];
    //         let pixel_forward = pixels[(start_x + curr_width_forward, y_pos + i)];
    //         let hsl_b = rgb_to_hsl([pixel_back[0], pixel_back[1], pixel_back[2]]);
    //         let hsl_f = rgb_to_hsl([pixel_forward[0], pixel_forward[1], pixel_forward[2]]);
    //         let check_bb = check_color(hsl_b, hue_range, sat_range, light_range);
    //         let check_ff = check_color(hsl_f, hue_range, sat_range, light_range);

    //         if check_bb {
    //             curr_width_backward += 1;
    //         }
    //         if check_ff {
    //             curr_width_forward += 1;
    //         }

    //         if !check_bb && !check_bb {
    //             break;
    //         }
    //     }
    //  }

    Some(Rectangle {
        x_pos,
        y_pos,
        width: the_width,
        height: the_height,
        color,
    })
}

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

pub fn get_cell_pos(x: u8, y: u8) -> (FloatCustom, FloatCustom, FloatCustom, FloatCustom) {
    let index = ((y - 1) * 4 + x) - 1;

    let cell = GRID[index as usize];

    (
        FloatCustom::new(cell.0),
        FloatCustom::new(cell.1),
        FloatCustom::new(cell.2),
        FloatCustom::new(cell.3),
    )
}

pub fn move_to_pos_in_grid(fd: i32, x: u8, y: u8) {
    let cell = get_cell_pos(x, y);
    queue::StopExec::send_immediate_command(fd);
    queue::ClearExec::send_immediate_command(fd);

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
    println!("last index: {}", last_index);
    println!("Current index: {}", curr);
    while last_index != curr {
        thread::sleep(Duration::from_millis(100));
        curr = queue::CurrentIndex::send_get_command(fd)
            .unwrap()
            .current_index;

        println!("last index: {}, current index: {}", last_index, curr);
    }
    queue::StopExec::send_immediate_command(fd);
}

// 3280x2464 pixels
fn main() {
    unsafe {
        //  take_picture();
        // extract_color_pixels("src/tyy.jpg", "yeppers.jpg", 1.5);
        // let s = String::from("HalloWelt!");
        // let cs = CString::new(s).unwrap();
        // let cv: Vec<u8> = cs.into_bytes_with_nul();
        // let mut tmp: Vec<i8> = cv.into_iter().map(|c| c as i8).collect::<_>(); // line 7
        // let _cptr: *mut i8 = tmp.as_mut_ptr();

        // cbinding::bindings::takee_pic(_cptr);

        let fd = cbinding::serial_open();

        move_to_pos_in_grid(fd, 3, 4);

        // homing::Param::send_immediate_command(
        //     fd,
        //     &FloatCustom::new(100.0),
        //     &FloatCustom::new(0.0),
        //     &FloatCustom::new(0.0),
        //     &FloatCustom::new(0.0),
        // );
        // let pos = GetPoseR::send_immediate_command(fd).unwrap();
        // let x = pos.x.to_float();
        // let y = pos.y.to_float();

        // // for e in &pos.y.hex_float {
        // //     println!("hex: Y: {:#02x}", e);
        // // }
        // //  homing::Cmd::send_immediate_command(fd, &0);
        // println!("({},{},{}, {})", x, y, pos.z.to_float(), pos.r.to_float());
        // // 120, -85, -30, 0, first row
        // //x, 120 -> 215
        // //y, -85-> -125,

        // // let new_x = 120.0 + (((215.0 - 120.0) / 4.0) * 1.0);
        // // let new_y = -85.0 + (((-125.0 + 85.0) / 4.0) * 1.0);
        // // homing::Cmd::send_immediate_command(fd, &0);
        // thread::sleep(Duration::from_secs(1));
        // homing::Cmd::send_immediate_command(fd, &0);
        //  thread::sleep(Duration::from_secs(1));
        // ptp::Cmd::send_immediate_command(
        //     fd,
        //     &ptp::PTPMode::MovlXYZ,
        //     &FloatCustom::new(210.0),
        //     &FloatCustom::new(-125.0),
        //     &FloatCustom::new(-30.0),
        //     &FloatCustom::new(0.0),
        // );
        // protocol::ptp::Cmd::send_immediate_command(fd, ptp_mode, x, y, z, r)

        //  sensor::set_infrared_immediate(fd, 1, sensor::Port::GP4);

        //  protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));

        // // thread::sleep(Duration::from_millis(2000));
        protocol::EMotor::send_immediate_command(fd, &1, &1, &IntCustom::new(10000));
        // sensor::get_infrared_state(fd, 0);
        loop {
            if sensor::get_infrared_state(fd, 0) == 1 {
                break;
            }
        }

        protocol::EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(0));
        // protocol::SuctionCup::send_immediate_command(fd, &1, &1);

        // thread::sleep(Duration::from_millis(2000));
        // protocol::SuctionCup::send_immediate_command(fd, &0, &0);

        // protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));

        // thread::sleep(Duration::from_millis(3000));
        // protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(0));

        // queue::StopExec::send_immediate_command(fd);
        // queue::ClearExec::send_immediate_command(fd);
        // let mut last_index = 0;
        // let mut f = FloatCustom::new(0.0);
        // let mut y_add = 10.0;

        // for i in 0..5 {
        //     y_add += 20.0;
        //     last_index = protocol::ptp::Cmd::send_queue_command(
        //         fd,
        //         &protocol::ptp::PTPMode::MovlXYZ,
        //         &FloatCustom::new(175.0),
        //         &FloatCustom::new(0.0),
        //         &FloatCustom::new(0.0),
        //         &pos.r,
        //     )
        //     .unwrap();
        // }
        // let mut pos = protocol::GetPoseR::send_immediate_command(fd).unwrap();

        // queue::ClearExec::send_immediate_command(fd);
        // queue::StopExec::send_immediate_command(fd);

        // protocol::homing::Cmd::send_queue_command(fd, &0);
        // println!("x: {}", &pos.x.to_float());
        //  println!("y: {}", &pos.y.to_float());
        //   println!("z: {}", &pos.z.to_float());
        // protocol::ptp::Cmd::send_queue_command(
        //     fd,
        //     &protocol::ptp::PTPMode::MovlXYZ,
        //     &FloatCustom::new(100.0),
        //     &FloatCustom::new(0.0),
        //     &FloatCustom::new(0.0),
        //     &pos.r,
        // );

        // queue::CurrentIndex::send_immediate_command(fd, current_index)
        // protocol::queue::StartExec::send_immediate_command(fd);

        // queue::StartExec::send_immediate_command(fd);

        // while protocol::queue::CurrentIndex::send_get_command(fd)
        //     .unwrap()
        //     .current_index
        //     >= last_index
        // {}
        cbinding::close_port(fd);
    }
}
