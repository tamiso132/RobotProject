use std::process::Command;

use colors_transform::Color;
use image::{GenericImageView, Rgb, RgbImage};
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
        let is_x = x + threshold >= self.x_pos as i32
            && x - threshold <= self.x_pos as i32 + self.width as i32;
        let is_y = y + threshold >= self.y_pos as i32
            && y - threshold <= self.y_pos as i32 + self.height as i32;

        is_x && is_y
    }
    fn print_to_screen(&self, image: &mut RgbImage, rgb: Rgb<u8>) {
        for x in self.x_pos..self.width + self.x_pos {
            for y in self.y_pos..self.height + self.y_pos {
                if image.height() > y {
                    image.put_pixel(x, y, Rgb([255, 255, 255]));
                }
            }
        }
    }
}
// let check_yellow = is_color_equal(hsl, (30, 70), (20, 100), (0, 80));

// let check_green = is_color_equal(hsl, (90, 160), (30, 50), (0, 70));

// let check_blue = is_color_equal(hsl, (210, 230), (45, 100), (13, 100));

// let check_red = is_color_equal(hsl, (330, 359), (60, 100), (0, 50));
const RED_COLOR: [(u16, u16); 3] = [(330, 359), (60, 100), (0, 50)];
const YELLOW_COLOR: [(u16, u16); 3] = [(30, 70), (40, 100), (25, 80)];
const BLUE_COLOR: [(u16, u16); 3] = [(210, 230), (40, 100), (5, 100)];
const GREEN_COLOR: [(u16, u16); 3] = [(90, 160), (10, 100), (0, 70)];

pub fn get_rectangle_pos_procentage() -> (f32, Colory) {
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

    let start_x = 5;
    let end_x = img.width();

    let start_y = 130;
    let end_y = img.height() - 200;

    let mut output_img = RgbImage::new(end_x - start_x, end_y - start_y);

    let orginal = img.to_rgb8();

    for x in start_x..end_x {
        for y in start_y..end_y {
            output_img.put_pixel(x - start_x, y - start_y, orginal[(x, y)]);
        }
    }

    output_img.save("output.jpg").unwrap();
    let mut pixels = output_img;
    let mut rectangles: Vec<Rectangle> = vec![];

    for y in 0..pixels.height() {
        for x in 0..pixels.width() {
            let mut in_bound = false;
            for r in &rectangles {
                if r.in_bound(x as i32, y as i32, 10 as i32) {
                    in_bound = true;
                    break;
                }
            }

            if in_bound == true {
                continue;
            }

            let pixel = pixels[(x, y)];
            let hsl = rgb_to_hsl([pixel[0], pixel[1], pixel[2]]);

            let check_yellow =
                is_color_equal(hsl, YELLOW_COLOR[0], YELLOW_COLOR[1], YELLOW_COLOR[2]);

            let check_green = is_color_equal(hsl, GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2]);

            let check_blue = is_color_equal(hsl, BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2]);

            let check_red = is_color_equal(hsl, RED_COLOR[0], RED_COLOR[1], RED_COLOR[2]);

            if check_red {
                let rectangle = get_object(&pixels, x, y, Colory::Red);
                if rectangle.is_some() {
                    rectangles.push(rectangle.unwrap());
                    continue;
                }
            } else if check_yellow {
                let rectangle = get_object(&pixels, x, y, Colory::Yellow);
                if rectangle.is_some() {
                    rectangles.push(rectangle.unwrap());
                    continue;
                }
            } else if check_blue {
                let rectangle = get_object(&pixels, x, y, Colory::Blue);
                if rectangle.is_some() {
                    rectangles.push(rectangle.unwrap());
                    continue;
                }
            } else if check_green {
                let rectangle = get_object(&pixels, x, y, Colory::Green);
                if rectangle.is_some() {
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
    println!("Len: {}", &rectangles.len());
    for r in rectangles {
        let current_height_rect = r.y_pos + r.height;
        r.print_to_screen(&mut pixels, white);
        println!(
            "X: {}, Y: {}, Width: {}, Height {},  Color, {:?}",
            r.x_pos, r.y_pos, r.height, r.width, r.color
        );
        if biggest_y < current_height_rect {
            rec_check = r;
            biggest_y = rec_check.y_pos + rec_check.height;
        }
    }

    pixels.save("path.jpg");

    let img_width = end_x - start_x;
    let täljare = (rec_check.x_pos) as f32 + (rec_check.width as f32 / 2.0) as f32;
    let procentage_x: f32 = 1.0 - (täljare as f32 / img_width as f32) as f32;
    if procentage_x > 1.0 || procentage_x < 0.0 {
        panic!("{}, {}, {}", täljare, procentage_x, img_width);
    }
    //pixels.save("hello.jpg");
    (procentage_x, rec_check.color)
    // Save the output image
}

fn check_color(rgb: [u8; 3], hue: (u16, u16), sat: (u16, u16), light: (u16, u16)) -> bool {
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

fn get_object(pixels: &RgbImage, start_x: u32, start_y: u32, color: Colory) -> Option<Rectangle> {
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
            hue_range = RED_COLOR[0];
            sat_range = RED_COLOR[1];
            light_range = RED_COLOR[2];
        }
        Colory::Yellow => {
            hue_range = YELLOW_COLOR[0];
            sat_range = YELLOW_COLOR[1];
            light_range = YELLOW_COLOR[2];
        }
        Colory::Blue => {
            hue_range = BLUE_COLOR[0];
            sat_range = BLUE_COLOR[1];
            light_range = BLUE_COLOR[2];
        }
        Colory::Green => {
            hue_range = GREEN_COLOR[0];
            sat_range = GREEN_COLOR[1];
            light_range = GREEN_COLOR[2];
        }
    }

    let max_width = pixels.width();
    let max_height = pixels.height();
    let mut progress_left = 0;
    let mut progress_right = 0;
    let mut current_y = start_y;
    loop {
        let mut color_right = false;
        let mut color_left = false;

        let x_to_right = progress_right + start_x;
        let x_to_left = (start_x as i32) - progress_left;
        if x_to_right < max_width {
            let pixel_forward = pixels[(x_to_right, current_y)].0;
            color_right = check_color(pixel_forward, hue_range, sat_range, light_range);
            if color_right {
                progress_right += 1;
            }
        }

        if x_to_left > 0 {
            let pixel_backward = pixels[(x_to_left as u32, current_y)].0;
            color_left = check_color(pixel_backward, hue_range, sat_range, light_range);
            if color_left {
                progress_left += 1;
            }
        }

        if !color_left && !color_right {
            let test = max_height - current_y + 5;
            let y_test_step;
            if 5 + current_y >= pixels.height() {
                y_test_step = 2;
            } else {
                y_test_step = 5;
            }

            let mut move_down = false;
            for y_step in 1..y_test_step {
                let color_check = check_color(
                    pixels[(start_x, y_step + current_y)].0,
                    hue_range,
                    sat_range,
                    light_range,
                );

                if color_check {
                    current_y += 1;
                    move_down = true;
                    break;
                }
            }
            if !move_down {
                break;
            }
        }
    }

    let mut height = if current_y - start_y == 0 {
        0
    } else {
        current_y - start_y - 1
    };
    let width = progress_left as u32 + progress_right;
    let x_start = start_x - progress_left as u32;
    let y_start = start_y;

    for w in 1..width {
        if height + 1 >= pixels.height() {
            break;
        } else {
            loop {
                if check_color(
                    pixels[(w + x_start, height)].0,
                    hue_range,
                    sat_range,
                    light_range,
                ) {
                    height += 1;
                    if height >= pixels.height() {
                        println!("here?");
                        height -= 2;
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    if height < 20 || width < 20 {
        return None;
    }

    Some(Rectangle {
        x_pos: x_start,
        y_pos: y_start,
        width: width,
        height: height,
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
    sat_range: (u16, u16),
    light_range: (u16, u16),
) -> bool {
    let curr_hue = curr_hsl[0] as u16;
    let curr_sat = curr_hsl[1] as u16;
    let curr_light = curr_hsl[2] as u16;

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
