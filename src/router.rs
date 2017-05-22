use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use yaml_rust::YamlLoader;

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
            Ok(_) => { /* noop */ }
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
                    rules.push(Rule {
                                   regex: match_regex.clone(),
                                   destination: String::from(destination_string),
                               })
                }
            }
        }

        Router { rules: rules }
    }
}

#[test]
fn test_new_with_valid_yaml() {
    let config_path_string = String::from("test/fixture/valid_config.yml");
    let router = Router::new(&config_path_string);

    let ref first_rule = router.rules[0];
    let ref second_rule = router.rules[1];
    let ref third_rule = router.rules[2];

    assert_eq!(router.rules.len(), 3);
    assert_eq!(first_rule.regex.as_str(), "personal-.+");
    assert_eq!(first_rule.destination, "personal-timeline");
    assert_eq!(second_rule.regex.as_str(), ".*");
    assert_eq!(second_rule.destination, "public-timeline");
    assert_eq!(third_rule.regex.as_str(), ".*");
    assert_eq!(third_rule.destination, "general");
}

#[test]
#[should_panic(expected = "No `match` term in")]
fn test_new_with_invalid_yaml_1() {
    Router::new(&String::from("test/fixture/invalid_config_1.yml"));
}

#[test]
#[should_panic(expected = "Failed to parse regex:")]
fn test_new_with_invalid_yaml_2() {
    Router::new(&String::from("test/fixture/invalid_config_2.yml"));
}

#[test]
#[should_panic(expected = "No `destinations` term in")]
fn test_new_with_invalid_yaml_3() {
    Router::new(&String::from("test/fixture/invalid_config_3.yml"));
}
