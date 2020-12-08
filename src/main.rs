use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};

fn read_input_data(filename: &str) -> HashMap<String, Vec<(i32, String)>> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut connections:HashMap<String, Vec<(i32, String)>> = HashMap::new();

    for line in f.lines() {
        let mut bag_connection:Vec<(i32, String)> = Vec::new();
        let result_line = line.unwrap();
        let key_values_split = result_line.split("contain").collect::<Vec<&str>>();
        let key = key_values_split[0].replace("bags", "").trim().to_owned();
        let values = key_values_split[1];
        debug!("Keys: {}, Values: {}", key, values);
        let values_split = values.split(",");
        for value in values_split {
            if !value.contains("no") {
                let clean_value = value.replace("bags", "")
                    .replace("bag", "").replace(".", "");
                let trimmed_value = clean_value.trim();
                let count = &trimmed_value[0..1].parse::<i32>().unwrap();
                let color = trimmed_value[1..].trim();
                bag_connection.push((count.clone(), String::from(color)));
                debug!("Count: {}, Color: {}", count, color);
            }
        }
        connections.insert(key, bag_connection);
    }
    return connections;
}

fn invert_dict(connections: HashMap<String, Vec<(i32, String)>>) -> HashMap<String, Vec<String>> {
    let mut inv_connections:HashMap<String, Vec<String>> = HashMap::new();
    for connection in connections {
        debug!("Key: {}, Values: {:?}", connection.0, connection.1);
        let color = connection.0;
        for color_key in connection.1 {
            debug!("Color Key: {:?}", color_key);
            if inv_connections.contains_key(&color_key.1) {
                let colors = inv_connections.get_mut(&color_key.1).unwrap();
                debug!("Colors: {:?}", colors);
                colors.push(color.clone());
            } else {
                inv_connections.insert(color_key.1, vec![color.clone()]);
            }
        }
    }
    return inv_connections
}

fn solution_part_1(file_name: &str) -> i32 {
    let connections = invert_dict(read_input_data(file_name));
    debug!("Connections: {:?}", connections);
    let mut backlog: Vec<String> = Vec::new();
    let colors = connections.get("shiny gold").unwrap();
    backlog.extend_from_slice(&colors[..]);
    let mut visited:HashSet<String> = HashSet::new();
    visited.insert(String::from("shiny gold"));
    while backlog.len() != 0 {
        let color = backlog.remove(0);
        debug!("Color: {}", color);
        if !visited.contains(&color) {
            if connections.contains_key(&color) {
                let colors = connections.get(&color).unwrap();
                backlog.extend_from_slice(&colors[..]);
            }
            visited.insert(color);
        }
    }
    debug!("Len: {}",visited.len());
    return (visited.len()-1) as i32;
}

fn solution_part_2(file_name: &str) -> i32 {
    let connections = read_input_data(file_name);
    let mut backlog: Vec<(i32, String)> = Vec::new();
    let bags = connections.get("shiny gold").unwrap();
    backlog.extend_from_slice(&bags[..]);
    let mut count:i32 = 0;
    while backlog.len() != 0 {
        let current_element = backlog.remove(0);
        count += current_element.0;
        let bags = connections.get(&current_element.1).unwrap();
        for (number, color) in bags {
            backlog.push((number*current_element.0,color.clone()))
        }
    }
    return count
}

fn main() {
    env_logger::init();
    info!("Count: {}", solution_part_1("testData.txt"));
    info!("Count: {}", solution_part_2("testData.txt"));
    info!("Count: {}", solution_part_2("testData_2.txt"));
}
