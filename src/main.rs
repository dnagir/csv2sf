extern crate csv;
use std::io;

struct SecurityGroup {
    name: String,
    description: String,
}

fn read_security_groups() -> Result<Vec<SecurityGroup>, io::Error> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    let groups = reader.records()
        .map(|record| record.expect("failed to read a CSV row"))
        .map(|record| SecurityGroup { name: record[0].to_string(), description: record[1].to_string() });

    Ok(groups.collect())
}

fn main() {
    let groups = read_security_groups().expect("couldn't read the CSV file");
    let resources: Vec<String> = groups
        .iter()
        .map(|sg| {
            format!(
                include_str!("sg.tmpl"),
                name = sg.name,
                description = sg.description
            )
        })
        .collect();
    let outputs: Vec<String> = groups
        .iter()
        .map(|sg| {
            format!(
                include_str!("sg-out.tmpl"),
                name = sg.name,
                description = sg.description
            )
        })
        .collect();

    println!(
        include_str!("main.tmpl"),
        resources = resources.join("\n"),
        outputs = outputs.join("\n")
    );
}
