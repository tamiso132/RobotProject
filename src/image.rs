use std::process::Command;

use colors_transform::Color;
use image::{Rgb, RgbImage};
use robotproject::protocol::FloatCustom;

pub struct Rectangle {
    pub x_pos: u32,
    pub y_pos: u32,
    pub width: u32,
    pub height: u32,
    pub color: Colory,
}

impl Rectangle {
    fn in_bound(&self, x: i32, y: i32, threshold: i32) -> bool {
        let is_x = x + threshold >= self.x_pos as i32 && x - threshold <= self.x_pos as i32 + self.width as i32;
        let is_y = y + threshold >= self.y_pos as i32 && y - threshold <= self.y_pos as i32 + self.height as i32;

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

pub fn get_rectangle_pos_procentage() -> f32 {
    // Load the image
    let img = image::open("src/tyy.jpg").expect("Failed to open image");

    //img = img.resize(1000, , imageops::FilterType::Nearest);

    // Create an output image with the same dimensions
   

    // Define the updated HSV range for green
    // Define the updated HSV range for green

    let black = Rgb([0 as u8, 0 as u8, 0 as u8]);
    let white = Rgb([255 as u8, 255 as u8, 255 as u8]);

    let width = img.width();
    let height = img.height();

    let start_x = 130;
    let end_x = img.width() - 100;

    let start_y = 215;
    let end_y = 300;

    let mut output_img = RgbImage::new(end_x - start_x, end_y - start_y);

    let orginal = img.to_rgb8();


    for x in start_x..end_x {
        for y in start_y..end_y {
            output_img.put_pixel(x-start_x, y-start_y, orginal[(x, y)]);
        }
    }
    output_img.save("output.jpg");

    let pixels = output_img;
    let mut rectangles: Vec<Rectangle> = vec![];

    for y in 0..pixels.height() {
        for x in 0..pixels.width() {
            let mut in_bound = false;
            for r in &rectangles {
                if r.in_bound(x as i32, y as i32, 20 as i32) {
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
                let rectangle = get_object(&pixels, x, y, Colory::Red);
                if rectangle.is_some() {
                    println!("{},{}", x, y);
                    rectangles.push(rectangle.unwrap());
                    continue;
                }
            }
           
            if check_yellow {
                //  output_img.put_pixel(x, y, pixel);
                let rectangle = get_object(&pixels, x, y, Colory::Yellow);
                if rectangle.is_some() {
                    println!("{},{}", x, y);
                    rectangles.push(rectangle.unwrap());
                    continue;
                }
            }
            if check_blue {
                let rectangle = get_object(&pixels, x, y, Colory::Blue);
                if rectangle.is_some() {
                    println!("{},{}", x, y);
                    rectangles.push(rectangle.unwrap());
                    continue;
                }
            }

            if check_green {
                let rectangle = get_object(&pixels, x, y, Colory::Green);
                if rectangle.is_some() {
                    println!("{},{}", x, y);
                    rectangles.push(rectangle.unwrap());
                    continue;
                }
            }
        }
    }
    let mut rec_check = Rectangle {
        x_pos: 0,
        y_pos: 0,
        width: 0,
        height: 0,
        color: Colory::Blue,
    };
    let mut biggest_y = 0;
    for r in rectangles {
        println!(
            "X: {}, Y: {}, Width: {}, Height {}",
            r.x_pos, r.y_pos, r.width, r.height
        );

        if biggest_y < r.y_pos + r.height {
            rec_check = r;
            biggest_y = rec_check.y_pos + rec_check.height;
        }
    }
    let img_width = pixels.width();
    // println!("procentage: {}, Color: ", procentage_x);
    let täljare = (rec_check.x_pos as f32) + (rec_check.width as f32/2.0) as f32;
    let procentage_x: f32 = 1.0 - (täljare as f32 / img_width as f32) as f32;
    if procentage_x > 1.0 || procentage_x < 0.0{
        panic!("{}, {}", täljare, procentage_x);
    }
    //pixels.save("hello.jpg");
    procentage_x
    // Save the output image
}

fn check_color(rgb: [u8; 3], hue: (u16, u16), sat: (u8, u8), light: (u8, u8)) -> bool {
    let hsl = rgb_to_hsl(rgb);

    is_color_equal(hsl, hue, sat, light)
}
#[derive(Debug, Clone, Copy)]
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

        let mut check_forward = false;
        let mut check_backward = false;

        let forward_index = curr_width_forward + start_x;
        let backward_index = start_x as i32 - curr_width_backward as i32;

        if forward_index < pixels.width(){
            let pixel_forward = pixels[(curr_width_forward + start_x, y_test)];
            check_forward = check_color(pixel_forward.0, hue_range, sat_range, light_range);
        }

        if backward_index >= 0{
            let pixel_back = pixels[(start_x - curr_width_backward, y_test)];
            check_backward = check_color(pixel_back.0, hue_range, sat_range, light_range);
        }

        

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
                let mut m = false;
                if y_test + i < pixels.height(){
                    let pixel_now = pixels[(start_x, y_test + i)];
                    let hsl_yep = rgb_to_hsl([pixel_now[0], pixel_now[1], pixel_now[2]]);
                    let m = is_color_equal(hsl_yep, hue_range, sat_range, light_range);    
                }
                else {
                    break;
                }
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
    let the_width = curr_width_forward  + curr_width_backward ;

    let mut height_down:u32 = 0;
    let mut height_up:u32 = 0;

    let mut step_up:u32 = 5;
    let mut step_down:u32 = 5;

    let free_down:u32 = 15;

    let mut x_test:u32 = 0;
    loop {
        let mut is_down_right = false;
        let mut is_up_right = false;
        let mut is_down_left = false;
        let mut is_up_left = false;
        if (start_x+ x_test) < pixels.width() {

            if (start_y + height_down + step_down + free_down) < pixels.height(){
                let pixel_down_right = pixels[(start_x + x_test, start_y + height_down + step_down + free_down)];
                is_down_right = check_color(pixel_down_right.0, hue_range, sat_range, light_range);
            }
            if start_y as i32  - height_up as i32 - step_up as i32  >= 0{
                let pixel_up_right = pixels[(start_x + x_test, start_y - height_up)];
                is_up_right = check_color(pixel_up_right.0, hue_range, sat_range, light_range);
            }
        }

        if (start_x as i32 - x_test as i32) >= 0{
            if (start_y + height_down + step_down+ free_down) < pixels.height(){
                let pixel_down_left = pixels[(start_x - x_test, start_y + height_down + step_down + free_down)];
                is_down_left = check_color(pixel_down_left.0, hue_range, sat_range, light_range);
            }
            if start_y as i32 - (height_up as i32 - step_up as i32) as i32 >= 0{
                let pixel_up_left = pixels[(start_x - x_test, start_y - height_up - step_up)];
                is_up_left = check_color(pixel_up_left.0, hue_range, sat_range, light_range);
            }
        }

      

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
                let mut left = false;
                let mut right = false;

                let mut left_index_x:i32 = (start_x as i32 - x_test as i32 - i as i32);
                let mut right_index_x:i32 = (start_x + x_test + i) as i32;
                if  start_y + free_down > pixels.height() {
                    break;
                }
                if left_index_x >= 0{
                    let pixel_left = pixels[(start_x - x_test - i, start_y + free_down)];
                    left = check_color(pixel_left.0, hue_range, sat_range, light_range);
                }

                if right_index_x < pixels.width() as i32{
                    let pixel_right = pixels[(start_x + x_test + i, start_y + free_down)];
                    right = check_color(pixel_right.0, hue_range, sat_range, light_range)
                }


                let m = left
                    || right;

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
    let y_pos = start_y + 5 - height_up;

    if height_down < 20 {
        return None;
    }

    let the_height:u32 = height_up + height_down + free_down;
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
        width: the_width as u32,
        height: the_height as u32,
        color,
    })
}
pub fn take_picture() {
    let output = Command::new("libcamera-jpeg")
    .arg("-o")
    .arg("/home/tom/projects/RobotProject/src/tyy.jpg")
    .arg("--width")
    .arg("500")
    .arg("--height")
    .arg("500")
    .arg("-t")
    .arg("50")
    .arg("-n")
        .output()
        .expect("Failed to execute libcamera-still command");

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
