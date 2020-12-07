use opencv::{
    core::{bitwise_or, in_range, Point, Scalar, Vector},
    imgproc::{contour_area, find_contours, moments, CHAIN_APPROX_SIMPLE, RETR_LIST},
    prelude::*,
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
    let src = Mat::from_slice(buffer)?.reshape(4, height as i32)?;
    let mut mask: Mat = Mat::default()?;
    let mut mask_emergency: Mat = Mat::default()?;
    let mut mask_all = Mat::default()?;
    // venting arrows have two colors, this will give us, half an arrow
    let target = (184., 179., 169.);
    // during an emergency the screen flashes red changing the color of the arrow
    let target_emergency = (116., 112., 201.);
    let delta = 1.;
    in_range(
        &src,
        &Scalar::new(target.0 - delta, target.1 - delta, target.2 - delta, 255.),
        &Scalar::new(target.0 + delta, target.1 + delta, target.2 + delta, 255.),
        &mut mask,
    )?;
    in_range(
        &src,
        &Scalar::new(
            target_emergency.0 - delta,
            target_emergency.1 - delta,
            target_emergency.2 - delta,
            255.,
        ),
        &Scalar::new(
            target_emergency.0 + delta,
            target_emergency.1 + delta,
            target_emergency.2 + delta,
            255.,
        ),
        &mut mask_emergency,
    )?;
    bitwise_or(&mask, &mask_emergency, &mut mask_all, &Mat::default()?)?;
    let mut contours: Vector<Vector<Point>> = Vector::new();
    find_contours(
        &mask_all,
        &mut contours,
        RETR_LIST,
        CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )?;
    let mut vents = Vec::default();
    for contour in contours {
        let area = contour_area(&contour, false)?;
        let area_score = area / (width as f64);

        // half venting arrow's area at fullscreen is 0.25 of the screenshot (on mac at least)
        // it's 0.77 on windows
        // starting from 0.01 to handle playing in a window and removing one off pixels
        if area_score > 0.01 {
            let moments = moments(&contour, true)?;
            let c_x = (moments.m10 / moments.m00).round() as i32;
            let c_y = (moments.m01 / moments.m00).round() as i32;
            vents.push((area_score, c_x, c_y));
        }
    }
    if vents.len() == 0 {
        return Ok(None);
    }
    vents.sort_by(|a, b| b.0.partial_cmp(&a.0).expect("NaN sorting vents by area"));
    let max_area = vents[0].0;
    for (i, vent) in vents.iter().enumerate() {
        println!("{} {:?} ",max_area, vent);
        if vent.0 / max_area < 0.9 {
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
