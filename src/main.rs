use std::{
    ffi::CString,
    fs,
    process::Command,
    thread::{self, Thread},
    time::Duration,
};

use image::{Pixel, Rgba, RgbaImage};
use robotproject::{
    self,
    cbinding::{self, close_port, read, write},
    protocol::{self, queue, sensor, FloatCustom, IntCustom, SuctionCup},
};

pub fn take_picture() {
    let output = Command::new("libcamera-jpeg")
        .arg("-o")
        .arg("/home/tom/projects/RobotProject/src/ty.jpg")
        .output()
        .expect("Failed to execute libcamera-still command");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn rgb_to_hsv(rgb: [u8; 3]) -> [f32; 3] {
    let r = rgb[0] as f32 / 255.0;
    let g = rgb[1] as f32 / 255.0;
    let b = rgb[2] as f32 / 255.0;

    let cmax = r.max(g).max(b);
    let cmin = r.min(g).min(b);
    let delta = cmax - cmin;

    let hue = if delta == 0.0 {
        0.0
    } else if cmax == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if cmax == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let saturation = if cmax == 0.0 { 0.0 } else { delta / cmax };
    let value = cmax;

    [hue, saturation, value]
}

fn adjust_brightness(pixel: &Rgba<u8>, factor: f32) -> Rgba<u8> {
    let adjusted_pixel = [
        (pixel[0] as f32 * factor).min(255.0) as u8,
        (pixel[1] as f32 * factor).min(255.0) as u8,
        (pixel[2] as f32 * factor).min(255.0) as u8,
        pixel[3],
    ];

    Rgba(adjusted_pixel)
}

fn extract_color_pixels(input_path: &str, output_path: &str, brightness_factor: f32) {
    // Load the image
    let img = image::open(input_path).expect("Failed to open image");

    // Create an output image with the same dimensions
    let mut output_img = RgbaImage::new(img.width(), img.height());

    // Define the updated HSV range for green
    // Define the updated HSV range for green
    let yellow_hue_range = (53.0, 60.0); // Adjusted hue range

    let green_hue_range = (70.0, 140.0); // Adjusted hue range

    let saturation_threshold = 0.4;
    let brightness_threshold = (0.3, 0.6);

    let black = Rgba([255 as u8, 255 as u8, 255 as u8, 255 as u8]);
    // Iterate through each pixel in the input image
    for (x, y, pixel) in img.to_rgba8().enumerate_pixels() {
        // Adjust brightness
        // let adjusted_pixel = adjust_brightness(pixel, brightness_factor);
        // Convert RGB to HSV
        let hsv = rgb_to_hsv([pixel[0], pixel[1], pixel[2]]);
        if (pixel[1] as f64) < saturation_threshold
            || (pixel[2] as f64) < brightness_threshold.0
                && (pixel[2] as f64) > brightness_threshold.1
        {
            output_img.put_pixel(x, y, black);
            continue;
        }
        // Check for yellow
        // if (hsv[0] >= 50.0 && hsv[0] <= 70.0) && hsv[2] > 0.5 {
        //     output_img.put_pixel(x, y, *pixel);
        // }
        if hsv[0] >= green_hue_range.0 && hsv[0] <= green_hue_range.1 {
            output_img.put_pixel(x, y, *pixel);
            continue;
        }

        if hsv[0] >= yellow_hue_range.0 && hsv[0] <= yellow_hue_range.1 {
            output_img.put_pixel(x, y, *pixel);
            continue;
        }

        output_img.put_pixel(x, y, black);

        // } else if (hsv[0] >= 150.0 && hsv[0] <= 210.0) && hsv[2] > 0.5 {
        //     output_img.put_pixel(x, y, adjusted_pixel);
        // } else if ((hsv[0] >= -30.0 && hsv[0] <= 30.0) || (hsv[0] >= 150.0 && hsv[0] <= 180.0))
        //     && hsv[2] > 0.5
        // {
        //     output_img.put_pixel(x, y, adjusted_pixel);
        // }
    }

    // Save the output image
    output_img.save(output_path).expect("Failed to save image");
}

// 3280x2464 pixels
fn main() {
    unsafe {
        extract_color_pixels("src/ty.jpg", "yeppers.jpg", 1.5);
        // let s = String::from("HalloWelt!");
        // let cs = CString::new(s).unwrap();
        // let cv: Vec<u8> = cs.into_bytes_with_nul();
        // let mut tmp: Vec<i8> = cv.into_iter().map(|c| c as i8).collect::<_>(); // line 7
        // let _cptr: *mut i8 = tmp.as_mut_ptr();

        // cbinding::bindings::takee_pic(_cptr);

        // let fd = cbinding::serial_open();

        // sensor::set_infrared_immediate(fd, 1, sensor::Port::GP4);

        // protocol::EMotor::send_immediate_command(fd, &0, &1, &IntCustom::new(10000));

        // thread::sleep(Duration::from_millis(2000));
        // protocol::EMotor::send_immediate_command(fd, &0, &0, &IntCustom::new(10000));
        // sensor::get_infrared_state(fd, 0);
        //  loop {
        //       println!("State: {}", sensor::get_infrared_state(fd, 0) as u8);
        //       thread::sleep(Duration::from_millis(1000));
        //  }

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
