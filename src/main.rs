use std::io;
use std::f64::consts::PI;

fn main() {
    let cam_px_h = get_num("Camera resolution in px (horizontal): ");
    let cam_px_v = get_num("Camera resolution in px (vertical): ");
    let fov_h = get_num("Camera FOV in deg (horizontal): ");
    let fov_v = get_num("Camera FOV in deg (vertical): ");
    let size_h = get_num("Size of object in m (horizontal): ");
    let size_v = get_num("Size of object in m (vertical): ");
    let r = get_num("Distance from the camera in m: ");

    let px_h = ((cam_px_h / fov_h) * (360.0 / PI) * (size_h/(2.0 * r)).atan()).round();
    let px_v = ((cam_px_v / fov_v) * (360.0 / PI) * (size_v/(2.0 * r)).atan()).round();
    println!("An object of size {}m x {}m will take up {} x {} pixels at a distance of {}m", size_h, size_v, px_h, px_v, r);
}


fn get_num(msg: &str) -> f64 {
    println!("{}", msg);
    let mut in_string = String::new();
    io::stdin().read_line(&mut in_string).expect("Failed to read line!");
    let num: f64 = in_string.trim().parse().expect("Please type a number!");
    return num;
}