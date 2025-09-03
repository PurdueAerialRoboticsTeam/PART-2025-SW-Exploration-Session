//! This program runs a terminal interface which allows the user to generate a global config file
//! used by the rest of the program.

use std::any::type_name;
use std::io::{self, Write};
use std::net::IpAddr;
use std::str::FromStr;

use configuranator_demo::*;

/// Prompts the user for a string input.
pub fn prompt_str_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Prompts the user for an input of type T.
pub fn prompt<T>(prompt: &str) -> T
where
    T: FromStr,
{
    loop {
        let input = prompt_str_input(prompt);
        match input.parse::<T>() {
            Ok(value) => return value,
            Err(_) => eprintln!("Invalid input. Please enter a valid {}.", type_name::<T>()),
        }
    }
}

/// Prompts the user for an IP address input.
pub fn prompt_ip(prompt_str: &str) -> IpAddr {
    prompt::<IpAddr>(prompt_str)
}

pub fn prompt_tuple<T>(prompt_str: &str) -> (T, T)
where
    T: FromStr,
{
    loop {
        let input = prompt_str_input(prompt_str);
        match parse_tuple::<T>(&input) {
            Ok(val) => return val,
            Err(e) => println!("Error: {}", e),
        }
    }
}

/// Prompts the user to enter a coordinate point (x, y).
pub fn prompt_point() -> Point {
    let x: f64 = prompt("Enter the x-coordinate: ");
    let y: f64 = prompt("Enter the y-coordinate: ");
    Point { x, y }
}

/// Prompts the user to define an area by entering multiple coordinate points.
/// The process continues until the user decides to stop adding points.
pub fn prompt_area(name: &str) -> Vec<Point> {
    println!("---ENTER {} AREA---", name);
    let mut area = Vec::new();
    loop {
        area.push(prompt_point());
        if prompt_str_input("Enter another point? (yes/no): ") != "yes" {
            break;
        }
    }
    area
}

pub fn parse_tuple<T>(s: &str) -> Result<(T, T), String>
where
    T: FromStr,
{
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid tuple: expected two comma-separated values, got '{}'",
            s
        ));
    }
    let first = parts[0]
        .trim()
        .parse::<T>()
        .map_err(|_| "Error parsing first value")?;
    let second = parts[1]
        .trim()
        .parse::<T>()
        .map_err(|_| "Error parsing second value")?;
    Ok((first, second))
}

/// Runs the program to get parameters and generate a configuration file.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_name: String =
        prompt("Enter the configuration file name (e.g., test_config.toml): ");
    while !file_name.ends_with(".toml") {
        println!("Error: The file name must end with '.toml'.");
        file_name = prompt("Enter the configuration file name (e.g., test_config.toml): ");
    }

    let test = prompt::<bool>("Is this a test configuration? (true/false): ");
    let turn_radius = prompt::<f64>("Enter the turn radius (meters): ");
    let velocity = prompt::<f64>("Enter the velocity (meters/second): ");

    let waypoints = prompt_area("WAYPOINTS");
    let mapping_area = prompt_area("MAPPING");
    let target_area = prompt_area("TARGET");
    let flying_threshold = prompt::<f64>("Enter flying altitude threshold: ");
    let mapping_threshold = prompt::<f64>("Enter mapping altitude threshold: ");

    let dad_gnc_port = prompt("Enter the Dad to GNC port number: ");
    let gnc_dad_port = prompt("Enter the GNC to Dad port number: ");
    let dad_sauron_port = prompt("Enter the Dad to Sauron port number: ");
    let sauron_dad_port = prompt("Enter the Sauron to Dad port number: ");

    let groundstation_ip = prompt_ip("Enter the ground station IP: ");
    let flightcomputer_ip = prompt_ip("Enter the flight computer IP: ");

    let model_path = prompt("Enter file path to sauron model: ");
    let input_size = prompt("Enter model image input size: ");
    let untagged_image_folder = prompt(
        "Enter the folder path for images before bounds check [Leave empty for default - /feonix-images/untagged]: ",
    );
    let detection_image_folder = prompt(
        "Enter the folder path for Sauron detection images [Leave empty for default - /feonix-images/detection]: ",
    );
    let mapping_image_folder = prompt(
        "Enter the folder path for Sauron mapping images [Leave empty for default - /feonix-images/mapping]: ",
    );

    let fov = prompt_tuple("Enter the FOV of the camera in the format f64, f64: ");

    let resolution = prompt_tuple("Enter the resolution of the camera in the format i32, i32: ");

    let dataset_name = loop {
        let input = prompt("Enter name of dataset to be used (i.e: COCO): ");
        if input != "COCO" {
            continue;
        }
        break input;
    };

    let config = ManagerConfig {
        test,
        sauron_config: SauronConfig {
            model_path,
            input_size,
            dataset_name,
            fov,
            resolution,
            untagged_image_folder,
            detection_image_folder,
            mapping_image_folder,
        },
        aircraft_properties: AircraftProperties {
            turn_radius,
            velocity,
        },
        coordinates: Coordinates {
            waypoints,
            mapping_area,
            target_area,
            flying_threshold,
            mapping_threshold,
        },
        commconfig: CommConfig {
            dad_gnc_port,
            gnc_dad_port,
            dad_sauron_port,
            sauron_dad_port,
            groundstation_ip: groundstation_ip.to_string(),
            flightcomputer_ip: flightcomputer_ip.to_string(),
        },
    };

    generate_config(file_name, config)
}
