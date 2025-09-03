//! Global project configuration structures.
//!
//! This library defines the various configuration settings used throughout the project,
//! including model configurations, aircraft properties, coordinates, and communication settings.

use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;
use toml::to_string;

/// Configuration settings for the global project.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManagerConfig {
    pub test: bool,
    pub sauron_config: SauronConfig,
    pub aircraft_properties: AircraftProperties,
    pub coordinates: Coordinates,
    pub commconfig: CommConfig,
}

/// Configuration for the YOLO model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SauronConfig {
    pub model_path: String,
    pub input_size: i32,
    pub dataset_name: String,
    pub fov: (f64, f64),
    pub resolution: (i32, i32),
    pub untagged_image_folder: String,
    pub detection_image_folder: String,
    pub mapping_image_folder: String,
}

impl Default for SauronConfig {
    fn default() -> Self {
        SauronConfig {
            model_path: "./sauron/data/yolov8n.onnx".to_string(),
            input_size: 640,
            dataset_name: "COCO".to_string(),
            fov: (93.0, 81.0),
            resolution: (4096, 2160),
            untagged_image_folder: "/feonix-images/untagged".to_string(),
            detection_image_folder: "/feonix-images/detection".to_string(),
            mapping_image_folder: "/feonix-images/mapping".to_string(),
        }
    }
}

/// Physical properties of the aircraft.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AircraftProperties {
    pub turn_radius: f64,
    pub velocity: f64,
}

/// Represents various coordinate sets used in competition.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinates {
    pub waypoints: Vec<Point>,
    pub mapping_area: Vec<Point>,
    pub target_area: Vec<Point>,
    pub flying_threshold: f64,
    pub mapping_threshold: f64,
}

/// A point in 2D space.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Communication settings between all processes.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommConfig {
    pub dad_gnc_port: i32,
    pub gnc_dad_port: i32,
    pub dad_sauron_port: i32,
    pub sauron_dad_port: i32,
    pub groundstation_ip: String,
    pub flightcomputer_ip: String,
}

/// Create a ManagerConfig file given a filepath of a .toml to read from
pub fn read_config<P: AsRef<Path>>(path: P) -> Result<ManagerConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: ManagerConfig = toml::de::from_str(&content)?;
    Ok(config)
}

/// Write a ManagerConfig to a .toml file given the filepath
pub fn generate_config(
    file_name: String,
    config: ManagerConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(file_name, to_string(&config)?)?;
    println!("Configuration file generation: SUCCESS");
    Ok(())
}
