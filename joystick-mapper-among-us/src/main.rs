//use image::{Bgra, ImageBuffer, Luma, imageops::FilterType, Rgb};
//use imageproc::{region_labelling::Connectivity, corners::corners_fast12, contours::find_contours, drawing::draw_polygon_mut, edges::canny, geometry::{approximate_polygon_dp, arc_length}, map::map_pixels, region_labelling::connected_components};
use joystick_mapper_lib::ActionClient;
use joystick_mapper_lib::{Action, InputState, JoystickClient, KeyMapping};
use joystick_mapper_lib::{Key, MouseAction};
use opencv::{core::Point, core::Scalar, core, core::{CV_8UC4, Vector}, imgcodecs::{IMREAD_COLOR, IMREAD_UNCHANGED}, imgproc::{CHAIN_APPROX_SIMPLE, RETR_LIST}, prelude::*};
use scrap::*;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{convert::TryInto, io::ErrorKind::WouldBlock};
use std::{env, fs, thread, time};
use std::ops::Deref;
use std::ffi::c_void;

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
        amount: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

        /*
        let image: ImageBuffer<Bgra<_>, _> = ImageBuffer::from_raw(
            w.try_into().unwrap(),
            h.try_into().unwrap(),
            buffer.to_owned(),
        )
        .unwrap();
        //let resized = resize(&image, (w/2).try_into().unwrap(), (h/2).try_into().unwrap(), FilterType::Nearest);
        let mask = map_pixels(&image, |_x, _y, p| {
            if p[0] == 184 && p[1] == 179 && p[2] == 169 {
                //println!("{:?}", p);
                Luma([255])
            } else {
                Luma([0])
            }
        });
        let mut output = map_pixels(&mask, |x, y, p| {
            Rgb([p[0], p[0], p[0]])
        });
        //let connected = connected_components(&mask, Connectivity::Eight, Luma([0]));
        //println!("{:?}", connected);

        let corners = corners_fast12(&mask, 128);
        println!("{:?}", corners);
        let points = corners
            .iter()
            .map(|imageproc::corners::Corner {x, y, ..}| imageproc::point::Point { x: *x as i32, y: *y as i32})
            .collect::<Vec<imageproc::point::Point<i32>>>();
        draw_polygon_mut(&mut output, &points[..], Rgb([255,0,0]));           
        /*
        let contours = find_contours::<i32>(&mask);
        for contour in contours {
            let poly_arc_length = arc_length(&contour.points, true);
            if poly_arc_length == 0_f64 {
                continue;
            }
            let polygon = approximate_polygon_dp(&contour.points, 0.01 * poly_arc_length, true);
            if polygon.len() == 3 {
                println!("{:?}", polygon);
                draw_polygon_mut(&mut output, &polygon, Rgb([255,0,0]));
            }
            
        }*/
        output.save("result.png")?;
*/
            let mut bytes = (&buffer).to_vec();
            let src = unsafe { Mat::new_rows_cols_with_data(h as _, w as _, CV_8UC4, bytes.as_mut_ptr() as *mut c_void, core::Mat_AUTO_STEP) }?;
            //let mut image: Mat = opencv::imgcodecs::imdecode(&Mat::from_slice::<u8>(pixels)?, IMREAD_UNCHANGED).unwrap();
            let mut mask: Mat = Mat::default()?;
            // venting arrows have two colors, this will give us, half an arrow
            let target = (184.,179.,169.);
            let delta = 1.;
            opencv::core::in_range(
                &src,
                &Scalar::new(target.0-delta, target.1-delta, target.2-delta, 255.),
                &Scalar::new(target.0+delta, target.1+delta, target.2+delta, 255.),
                &mut mask,
            )?;
            opencv::imgcodecs::imwrite("result.png", &mask, &Vector::new())?;
            let mut contours: Vector<Vector<Point>> = Vector::new();
            opencv::imgproc::find_contours(
                &mask,
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
                if area_score > 0.01 && area_score < 1. {
                    let moments = opencv::imgproc::moments(&contour, true)?;
                    let c_x = moments.m10 / moments.m00;
	                let c_y = moments.m01 / moments.m00;
                    vents.push((c_x, c_y));
                }
            }
            println!("{:?} {:?}",  vents.len(), vents);
            
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
    VentAction::VentUp.perform_action(&mut joystick_client.action_client, InputState::Up, None)?; //TODO remove
    std::process::exit(0); //TODO remove

    let pause = time::Duration::from_millis(15);
    loop {
        joystick_client.exec_event_loop()?;
        thread::sleep(pause);
    }
}
/*
pub fn is_convex(vertices: &Vector<Point>) -> bool {
    if vertices.len() < 4 {
        return true;
    }

    let mut sign: bool = false;
    let n = vertices.len();
    {
        let mut i = 0;
        while i < n {
            {
                let dx1 =
                    vertices.get((i + 2) % n).unwrap().x - vertices.get((i + 1) % n).unwrap().x;
                let dy1 =
                    vertices.get((i + 2) % n).unwrap().y - vertices.get((i + 1) % n).unwrap().y;
                let dx2 = vertices.get(i).unwrap().x - vertices.get((i + 1) % n).unwrap().x;
                let dy2 = vertices.get(i).unwrap().y - vertices.get((i + 1) % n).unwrap().y;
                let zcrossproduct = dx1 * dy2 - dy1 * dx2;
                if i == 0 {
                    sign = zcrossproduct > 0;
                } else if sign != (zcrossproduct > 0) {
                    return false;
                }
            }
            i += 1;
        }
    }
    true
}
*/
/*
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // let display = Display::primary()?;
    // let mut capturer = Capturer::new(display)?;
    // let (w, h) = (capturer.width(), capturer.height());

    // loop {
    //     // Wait until there's a frame.
    //     let mut buffer = match capturer.frame() {
    //         Ok(buffer) => buffer,
    //         Err(error) => {
    //             if error.kind() == WouldBlock {
    //                 // Keep spinning.
    //                 thread::sleep(Duration::new(1, 0) / 60);
    //                 continue;
    //             } else {
    //                 panic!("Error: {}", error);
    //             }
    //         }
    //     };

    const buffer: &[u8] = include_bytes!("../input.png");


        let image: Mat = opencv::imgcodecs::imdecode(&Mat::from_slice(&buffer)?, IMREAD_UNCHANGED).unwrap();
        let mut mask: Mat = image.clone();
        opencv::core::in_range(&image, &Scalar::new(184.,179.,169., 255.), &Scalar::new(184.,179.,169., 255.), &mut mask)?;
        let mut contours: Vector<Vector<Point>> = Vector::new();
        opencv::imgproc::find_contours(&mask, &mut contours, RETR_LIST, CHAIN_APPROX_SIMPLE, opencv::core::Point_::new(0,0))?;
        for contour in contours {
            let arc_length = opencv::imgproc::arc_length(&contour, true)?;
            let mut poly: Vector<Point> = Vector::new();
            opencv::imgproc::approx_poly_dp(&contour, &mut poly, 0.01 * arc_length, true)?;
            if poly.len() == 3 {
                println!("{:?}", poly);
            }
        }
        opencv::imgcodecs::imwrite("result.png", &mask, &Vector::new())?;


        // let image: ImageBuffer<Bgra<_>, _> = ImageBuffer::from_raw(
        //     w.try_into().unwrap(),
        //     h.try_into().unwrap(),
        //     buffer.to_owned(),
        // )
        // .unwrap();
        // let resized = resize(&image, (w/2).try_into().unwrap(), (h/2).try_into().unwrap(), FilterType::Nearest);
        // let mask = map_pixels(&resized, |x, y, p| {
        //     if p[0] > 180 && p[0] < 220 && p[1] > 170 && p[1] < 220 && p[2] > 160 && p[2] < 210 {
        //         //println!("{:?}", p);
        //         Luma([255])
        //     } else {
        //         Luma([0])
        //     }
        // });
        // let mut output = map_pixels(&mask, |x, y, p| {
        //     Rgb([p[0], p[0], p[0]])
        // });
        // let contours = find_contours::<i32>(&mask);
        // for contour in contours {
        //     let poly_arc_length = arc_length(&contour.points, true);
        //     if poly_arc_length == 0_f64 {
        //         continue;
        //     }
        //     let polygon = approximate_polygon_dp(&contour.points, 0.01 * poly_arc_length, true);
        //     if polygon.len() == 4 {
        //         println!("{:?}", polygon);
        //         draw_polygon_mut(&mut output, &contour.points, Rgb([255,0,0]));
        //     }

        // }
        // output.save("result.png")?;


        //find_contours
        //approximate_polygon_dp

        //scrap shit
    //     break;
    // }

    Ok(())
}
*/
