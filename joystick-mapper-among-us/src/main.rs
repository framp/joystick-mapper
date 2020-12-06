//use image::{Bgra, ImageBuffer, Luma, imageops::FilterType, Rgb};
//use imageproc::{region_labelling::Connectivity, corners::corners_fast12, contours::find_contours, drawing::draw_polygon_mut, edges::canny, geometry::{approximate_polygon_dp, arc_length}, map::map_pixels, region_labelling::connected_components};
use joystick_mapper_lib::ActionClient;
use joystick_mapper_lib::{Action, InputState, JoystickClient, KeyMapping};
use joystick_mapper_lib::{Key, MouseAction};
use opencv::{core::Point, core::Scalar, core::Vector, imgproc::{CHAIN_APPROX_SIMPLE, RETR_LIST}, prelude::*};
use scrap::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::io::ErrorKind::WouldBlock;
use std::{env, fs, thread, time};
use enigo::*;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum VentAction {
    VentUp,
    VentDown,
    VentRight,
    VentLeft,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum KeyMouseVentAction {
    Key(Key),
    Layout(char),
    Mouse(MouseAction),
    Vent(VentAction),
}

impl Action for VentAction {
    fn perform_action(
        &self,
        client: &mut ActionClient,
        input_state: InputState,
        _amount: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if input_state == InputState::Up {
            return Ok(());
        }

        println!("venting");
        let display = Display::primary()?;
        let mut capturer = Capturer::new(display)?;
        let (w, h) = (capturer.width(), capturer.height());

        loop {
            // Wait until there's a frame.
            let buffer = match capturer.frame() {
                Ok(buffer) => buffer,
                Err(error) => {
                    if error.kind() == WouldBlock {
                        // Keep spinning.
                        thread::sleep(time::Duration::new(1, 0) / 60);
                        continue;
                    } else {
                        panic!("Error: {}", error);
                    }
                }
            };

            println!("got buffer {:?}", buffer.len());

            let src = Mat::from_slice(&buffer).unwrap();
            let src = src.reshape(4, h as i32).unwrap();
            let mut mask: Mat = Mat::default()?;
            let mut mask_emergency: Mat = Mat::default()?;
            let mut mask_all = Mat::default()?;
            // venting arrows have two colors, this will give us, half an arrow
            let target = (184.,179.,169.);
            let target_emergency = (116., 112., 201.);
            let delta = 1.;
            opencv::core::in_range(
                &src,
                &Scalar::new(target.0-delta, target.1-delta, target.2-delta, 255.),
                &Scalar::new(target.0+delta, target.1+delta, target.2+delta, 255.),
                &mut mask,
            )?;
            opencv::core::in_range(
                &src,
                &Scalar::new(target_emergency.0-delta, target_emergency.1-delta, target_emergency.2-delta, 255.),
                &Scalar::new(target_emergency.0+delta, target_emergency.1+delta, target_emergency.2+delta, 255.),
                &mut mask_emergency,
            )?;
            opencv::core::bitwise_or(&mask, &mask_emergency, &mut mask_all, &Mat::default()?)?;
            let mut contours: Vector<Vector<Point>> = Vector::new();
            opencv::imgproc::find_contours(
                &mask_all,
                &mut contours,
                RETR_LIST,
                CHAIN_APPROX_SIMPLE,
                opencv::core::Point_::new(0, 0),
            )?;
            let mut vents = Vec::default();
            for contour in contours {
                let area = opencv::imgproc::contour_area(&contour,false)?;
                let area_score = area / (w as f64);

                // half venting arrow's area at fullscreen is 0.25 of the screenshot
                // starting from 0.01 to handle playing in a window
                if area_score > 0.01 && area_score < 0.5 {
                    let moments = opencv::imgproc::moments(&contour, true)?;
                    let c_x = (moments.m10 / moments.m00).round() as i32;
	                let c_y = (moments.m01 / moments.m00).round() as i32;
                    vents.push((area_score, c_x, c_y));
                }
            }
            if vents.len() == 0 {
                break;
            }
            vents.sort_by(|a, b| b.0.partial_cmp(&a.0).expect("NaN sorting vents by area"));
            vents.truncate(3); // there are maximum 3 vent direction in game
            //println!("{:?} {:?}",  vents.len(), vents);
            let vent_coords : Vec<(i32, i32)> = vents.iter().map(|(_,b,c)| (*b,*c)).collect();
            let mut direction = vec![vent_coords.clone(),vent_coords.clone(),vent_coords.clone(),vent_coords.clone()];
            direction[0].sort_by(|a, b| b.1.cmp(&a.1));
            direction[1].sort_by(|a, b| a.1.cmp(&b.1));
            direction[2].sort_by(|a, b| b.0.cmp(&a.0));
            direction[3].sort_by(|a, b| a.0.cmp(&b.0));

            // println!("{:?}",  direction[0]);
            // println!("{:?}",  direction[1]);
            // println!("{:?}",  direction[2]);
            // println!("{:?}",  direction[3]);

            for vent in vent_coords {
                if direction.iter().any(|vent_by_dir| vent_by_dir[0] == vent) {
                    continue;
                }
                let mut second_chances : Vec<(usize, &mut Vec<(i32, i32)>)>= direction.iter_mut().filter(|vent_by_dir| vent_by_dir[1] == vent).enumerate().collect();
                //println!("{:?}", second_chances);
                second_chances.sort_by(|(direction_a, data_a), (direction_b, data_b)| {
                    let delta_a = match direction_a {
                        0 => data_a[0].1 - data_a[1].1,
                        1 => data_a[1].1 - data_a[0].1,
                        2 => data_a[0].0 - data_a[1].0,
                        3 => data_a[1].0 - data_a[0].0,
                        _ => 0
                    };
                    let delta_b = match direction_b {
                        0 => data_b[0].1 - data_b[1].1,
                        1 => data_b[1].1 - data_b[0].1,
                        2 => data_b[0].0 - data_b[1].0,
                        3 => data_b[1].0 - data_b[0].0,
                        _ => 0
                    };
                    delta_a.cmp(&delta_b)
                });
                &mut second_chances[0].1.swap(0, 1);
            }
            // println!("{:?}",  direction[0]);
            // println!("{:?}",  direction[1]);
            // println!("{:?}",  direction[2]);
            // println!("{:?}",  direction[3]);
            
            let vent = match self {
                VentAction::VentDown => direction[0][0],
                VentAction::VentUp => direction[1][0],
                VentAction::VentRight => direction[2][0],
                VentAction::VentLeft => direction[3][0],
            };
            println!("selected {:?}", vent);
            client.enigo.mouse_move_to(vent.0, vent.1);
            client.enigo.mouse_click(MouseButton::Left);
            println!("done");
            break;
        }
        Ok(())
    }
}

impl Action for KeyMouseVentAction {
    fn perform_action(
        &self,
        client: &mut ActionClient,
        input_state: InputState,
        amount: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            KeyMouseVentAction::Key(key) => key.perform_action(client, input_state, amount),
            KeyMouseVentAction::Layout(c) => {
                Key::Layout(*c).perform_action(client, input_state, amount)
            }
            KeyMouseVentAction::Mouse(mouse) => mouse.perform_action(client, input_state, amount),
            KeyMouseVentAction::Vent(vent) => vent.perform_action(client, input_state, amount),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let exec = env::args().nth(0).unwrap();
    let filename = env::args().nth(1);
    if filename.is_none() {
        println!("USAGE: {} <configuration-file>", exec);
        std::process::exit(0);
    }
    let conf_content = fs::read_to_string(filename.unwrap()).expect("Failed reading the file");
    let conf: KeyMapping<KeyMouseVentAction> = serde_yaml::from_str(&conf_content)?;

    let mut joystick_client: JoystickClient<KeyMouseVentAction> = JoystickClient::new(conf);

    let pause = time::Duration::from_millis(15);
    loop {
        joystick_client.exec_event_loop()?;
        thread::sleep(pause);
    }
}