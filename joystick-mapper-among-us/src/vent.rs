#[path = "opencv.rs"] mod opencv;

use self::opencv::{
    bitwise_or, in_range, Mat, Point,
    contour_area, find_contours, 
    CV_8UC1, CV_8UC4, RETR_LIST, CHAIN_APPROX_SIMPLE
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum VentAction {
    VentUp,
    VentDown,
    VentRight,
    VentLeft,
}

pub fn select_vent(
    buffer: &[u8],
    width: i32,
    height: i32,
    max_vents: u8,
    action: &VentAction,
) -> Result<Option<(i32, i32)>, Box<dyn std::error::Error>> {
    let src = Mat::new_from_bytes(height, width, CV_8UC4, buffer);
    let mut mask: Mat = Mat::default();
    let mut mask_emergency: Mat = Mat::default();
    let mut mask_all = Mat::default();
    // venting arrows have two colors, this will give us, half an arrow
    let target = (218, 215, 209); // the dark part of the arrow is 184, 179, 169
    // during an emergency the screen flashes red changing the color of the arrow
    let target_emergency = (137, 135, 226); // the dark part of the arrow is 116, 112, 201
    let delta = 1;
    let lower_color_mat = Mat::new_from_bytes(1, 4, CV_8UC1, &[target.0 - delta, target.1 - delta, target.2 - delta, 255]);
    let upper_color_mat = Mat::new_from_bytes(1, 4, CV_8UC1, &[target.0 + delta, target.1 + delta, target.2 + delta, 255]);
    let lower_color_mat_emergency = Mat::new_from_bytes(1, 4, CV_8UC1, &[target_emergency.0 - delta, target_emergency.1 - delta, target_emergency.2 - delta, 255]);
    let upper_color_mat_emergency = Mat::new_from_bytes(1, 4, CV_8UC1, &[target_emergency.0 + delta, target_emergency.1 + delta, target_emergency.2 + delta, 255]);
    in_range(
        &src,
        &lower_color_mat,
        &upper_color_mat,
        &mut mask,
    );
    in_range(
        &src,
        &lower_color_mat_emergency,
        &upper_color_mat_emergency,
        &mut mask_emergency,
    );
    bitwise_or(&mask, &mask_emergency, &mut mask_all);

    let contours: Vec<Vec<Point>> = find_contours(
        &mask_all,
        RETR_LIST,
        CHAIN_APPROX_SIMPLE,
    );
    let mut vents = Vec::default();
    for mut contour in contours {
        let area = contour_area(&mut contour);
        let area_score = area / (width as f64);

        // half venting arrow's area at fullscreen is 0.25 of the screenshot (on mac at least)
        // it's 0.77 on windows
        // starting from 0.01 to handle playing in a window and removing one off pixels
        if area_score > 0.01 {
            let mut c_x = 0;
            let mut c_y = 0;
            let mut counter = 0;
            for point in contour {
                c_x = c_x + point.x;
                c_y = c_y + point.y;
                counter = counter + 1;
            }
            c_x = c_x / counter;
            c_y = c_y / counter;
            vents.push((area_score, c_x, c_y));
        }
    }
    if vents.len() == 0 {
        return Ok(None);
    }
    vents.sort_by(|a, b| b.0.partial_cmp(&a.0).expect("NaN sorting vents by area"));
    let max_area = vents[0].0;
    for (i, vent) in vents.iter().enumerate() {
        if vent.0 / max_area < 0.5 {
            vents.truncate(i);
            break;
        }
    }
    vents.truncate(max_vents as usize);
    let vent_coords: Vec<(i32, i32)> = vents.iter().map(|(_, b, c)| (*b, *c)).collect();
    let mut direction = vec![
        vent_coords.clone(),
        vent_coords.clone(),
        vent_coords.clone(),
        vent_coords.clone(),
    ];
    direction[0].sort_by(|a, b| b.1.cmp(&a.1));
    direction[1].sort_by(|a, b| a.1.cmp(&b.1));
    direction[2].sort_by(|a, b| b.0.cmp(&a.0));
    direction[3].sort_by(|a, b| a.0.cmp(&b.0));

    for vent in vent_coords {
        if direction.iter().any(|vent_by_dir| vent_by_dir[0] == vent) {
            continue;
        }
        let mut second_chances: Vec<(usize, &mut Vec<(i32, i32)>)> = direction
            .iter_mut()
            .filter(|vent_by_dir| vent_by_dir[1] == vent)
            .enumerate()
            .collect();
        second_chances.sort_by(|(direction_a, data_a), (direction_b, data_b)| {
            let delta_a = match direction_a {
                0 => data_a[0].1 - data_a[1].1,
                1 => data_a[1].1 - data_a[0].1,
                2 => data_a[0].0 - data_a[1].0,
                3 => data_a[1].0 - data_a[0].0,
                _ => 0,
            };
            let delta_b = match direction_b {
                0 => data_b[0].1 - data_b[1].1,
                1 => data_b[1].1 - data_b[0].1,
                2 => data_b[0].0 - data_b[1].0,
                3 => data_b[1].0 - data_b[0].0,
                _ => 0,
            };
            delta_a.cmp(&delta_b)
        });
        &mut second_chances[0].1.swap(0, 1);
    }

    let vent = match action {
        VentAction::VentDown => direction[0][0],
        VentAction::VentUp => direction[1][0],
        VentAction::VentRight => direction[2][0],
        VentAction::VentLeft => direction[3][0],
    };
    Ok(Some(vent))
}
