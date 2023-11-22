use colors_transform::Color;
use image::{imageops, DynamicImage, GenericImageView, Pixel, Rgb, RgbImage, Rgba, RgbaImage};
use robotproject::{
    self,
    cbinding::{self, close_port, read, write},
    protocol::{self, queue, sensor, FloatCustom, IntCustom, SuctionCup},
};
use std::{
    ffi::CString,
    fs,
    process::Command,
    thread::{self, Thread},
    time::Duration,
};

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

fn check_color(
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

    for y in start_y..end_y {
        for x in start_x..end_x {
            let pixel = pixels[(x, y)];
            let hsl = rgb_to_hsl([pixel[0], pixel[1], pixel[2]]);
            let check_red2 = check_color(hsl, (330, 359), (20, 100), (0, 50));

            let check_yellow = check_color(hsl, (45, 65), (40, 100), (0, 60));

            let check_green = check_color(hsl, (90, 160), (15, 50), (0, 70));

            //let check_blue = check_color(hsl, (210, 230), (45, 100), (13, 100));

            if check_blue {
                output_img.put_pixel(x, y, pixel);
            }
        }
    }
    // Iterate through each pixel in the input image
    // for (x, y, pixel) in img.to_rgb8().enumerate_pixels()[0] {
    //     // Adjust brightness
    //     // let adjusted_pixel = adjust_brightness(pixel, brightness_factor);
    //     // Convert RGB to HSV

    //     let hsl = rgb_to_hsl([pixel[0], pixel[1], pixel[2]]);

    //     let check_yellow = check_color(hsl, (45, 65), (40, 100), (0, 60));

    //     let check_green = check_color(hsl, (90, 160), (30, 100), (0, 70));

    //     let check_blue = check_color(hsl, (200, 240), (40, 100), (10, 50));

    //     // let check_red1 = check_color(hsl, (0, 359), (0, 100), (0, 100));
    //     let check_red2 = check_color(hsl, (330, 359), (20, 100), (0, 50));

    //     // if check_green {
    //     //     output_img.put_pixel(x, y, *pixel);
    //     //     continue;
    //     // }
    //     // if check_yellow {
    //     //     output_img.put_pixel(x, y, *pixel);
    //     //     continue;
    //     // }

    //     // if check_red2 {
    //     //     output_img.put_pixel(x, y, *pixel);
    //     //     continue;
    //     // }

    //     // if check_blue {
    //     //     output_img.put_pixel(x, y, *pixel);
    //     //     continue;
    //     // }

    //     // if too_low_sat || unallowed_brightness {
    //     //     output_img.put_pixel(x, y, black);
    //     //     continue;
    //     // }
    //     // println!("Hue {}, satur {},  Brightness {},", hsv[0], hsv[1], hsv[2]);
    //     // Check for yellow
    //     // if curr_hue >= yellow_hue_range.0 && curr_hue <= yellow_hue_range.1 {
    //     //     output_img.put_pixel(x, y, *pixel);
    //     //     continue;
    //     // }

    //     output_img.put_pixel(x, y, *pixel);
    //     if y == 215 {
    //         output_img.put_pixel(x, y, white)
    //     }

    //     if y == 305 {
    //         output_img.put_pixel(x, y, white)
    //     }

    //     if x == 125 {
    //         output_img.put_pixel(x, y, white);
    //     }

    //     if x == img.width() - 100 {
    //         output_img.put_pixel(x, y, white);
    //     }
    //     // if hsv[0] >= green_hue_range.0 && hsv[0] <= green_hue_range.1 {
    //     //     output_img.put_pixel(x, y, Rgb([124, 254, 0]));
    //     //     all_green.push((x, y, Rgb([124, 254, 0])));
    //     //     continue;
    //     // }

    //     // if hsv[0] >= yellow_hue_range.0 && hsv[0] <= yellow_hue_range.1 {
    //     //     output_img.put_pixel(x, y, *pixel);
    //     //     continue;
    //     // }

    //     // output_img.put_pixel(x, y, black);

    //     // } else if (hsv[0] >= 150.0 && hsv[0] <= 210.0) && hsv[2] > 0.5 {
    //     //     output_img.put_pixel(x, y, adjusted_pixel);
    //     // } else if ((hsv[0] >= -30.0 && hsv[0] <= 30.0) || (hsv[0] >= 150.0 && hsv[0] <= 180.0))
    //     //     && hsv[2] > 0.5
    //     // {
    //     //     output_img.put_pixel(x, y, adjusted_pixel);
    //     // }
    // }

    // Save the output image
    output_img.save(output_path).expect("Failed to save image");
}

// 3280x2464 pixels
fn main() {
    unsafe {
        //  take_picture();
        extract_color_pixels("src/tyy.jpg", "yeppers.jpg", 1.5);
        // let s = String::from("HalloWelt!");
        // let cs = CString::new(s).unwrap();
        // let cv: Vec<u8> = cs.into_bytes_with_nul();
        // let mut tmp: Vec<i8> = cv.into_iter().map(|c| c as i8).collect::<_>(); // line 7
        // let _cptr: *mut i8 = tmp.as_mut_ptr();

        // cbinding::bindings::takee_pic(_cptr);

        // let fd = cbinding::serial_open();

        //  sensor::set_infrared_immediate(fd, 1, sensor::Port::GP4);

        //  protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));

        // // thread::sleep(Duration::from_millis(2000));
        //  protocol::EMotor::send_immediate_command(fd, &1, &1, &IntCustom::new(10000));
        // // sensor::get_infrared_state(fd, 0);
        //   loop {
        //        if sensor::get_infrared_state(fd, 0) == 1 {
        //         break;
        //        }
        //   }

        // protocol::EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(0));
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
    }
}
