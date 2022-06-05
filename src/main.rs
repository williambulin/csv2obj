use std::{collections::HashMap, fs::File, io::Read};

use yaml_rust::YamlLoader;

fn main() {
    let env: Vec<String> = std::env::args().collect();
    if env.len() <= 1 {
        eprintln!("Usage: {} <yaml file>", &env[0]);
        return;
    }

    let filepath = &env[1];

    let mut file = File::open("config.yaml").unwrap();
    let mut config_content = String::new();
    file.read_to_string(&mut config_content).unwrap();
    let configs = YamlLoader::load_from_str(config_content.as_str()).unwrap();
    let config = &configs[0];

    let mut csv_file = csv::Reader::from_path(filepath).unwrap();
    let headers = csv_file.headers().cloned();
    let records = csv_file.records();

    let mut mapped_records = Vec::new();

    for result in records {
        let record = result.unwrap();
        let mut record_map = HashMap::new();
        for (i, field) in headers.as_ref().unwrap().iter().enumerate() {
            let value = record[i].trim().to_string();
            record_map.insert(field.trim(), value);
        }
        mapped_records.push(record_map);
    }

    let position_key = config["config"]["position"].as_str().unwrap();
    for mapped_record in mapped_records.iter() {
        println!(
            "v {} {} {}",
            mapped_record[format!("{}.x", position_key).as_str()],
            mapped_record[format!("{}.y", position_key).as_str()],
            mapped_record[format!("{}.z", position_key).as_str()],
        );
    }

    let uv_key = config["config"]["uv"].as_str().unwrap();
    for mapped_record in mapped_records.iter() {
        println!(
            "vt {} {}",
            mapped_record[format!("{}.x", uv_key).as_str()],
            mapped_record[format!("{}.y", uv_key).as_str()]
                .parse::<f32>()
                .unwrap()
                * -1.0,
        );
    }

    for i in 0..mapped_records.iter().count() / 3 {
        let offset_i = i * 3;
        println!(
            "f {0}/{0} {1}/{1} {2}/{2}",
            offset_i + 1,
            offset_i + 2,
            offset_i + 3,
        );
    }
}
