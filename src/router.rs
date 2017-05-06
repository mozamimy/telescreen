use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

use yaml_rust::YamlLoader;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Rule {
    pub regex: Regex,
    pub destination: String,
}

#[derive(Debug)]
pub struct Router {
    pub rules: Vec<Rule>,
}

impl Router {
    pub fn new(config_path_string: &String) -> Router {
        let config_path = Path::new(config_path_string);
        let display = config_path.display();
        let mut file = match File::open(&config_path) {
            Err(why) => panic!("Clould not open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };

        let mut yaml_string = String::new();
        match file.read_to_string(&mut yaml_string) {
            Err(why) => panic!("Clould not read {}: {}", display, Error::description(&why)),
            Ok(_) => { /* noop */ },
        }

        let yamls = match YamlLoader::load_from_str(&yaml_string) {
            Err(e) => panic!("Failed to load yaml: {:?}", e),
            Ok(c) => c,
        };

        let mut rules = vec![];
        for yaml in yamls {
            for rule in yaml {
                let match_regex_string = match rule["match"].as_str() {
                    None => panic!("No `match` term in {:?}", rule),
                    Some(m) => m,
                };
                let match_regex = match Regex::new(match_regex_string) {
                    Err(e) => panic!("Failed to parse regex: {:?}", e),
                    Ok(m) => m,
                };
                let destinations = match rule["destinations"].as_vec() {
                    None => panic!("No `destinations` term in {:?}", rule),
                    Some(d) => d,
                };

                for destination in destinations {
                    let destination_string = match destination.as_str() {
                        None => panic!("No `destinations` term in {:?}", destination),
                        Some(d) => d,
                    };
                    rules.push( Rule { regex: match_regex.clone(), destination: String::from(destination_string) })
                }
            }
        }

        Router { rules: rules }
    }
}
