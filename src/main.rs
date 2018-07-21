use std::io;
extern crate csv;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct SecurityGroup {
    name: String,
    description: String,
}

type SecurityGroups = Vec<SecurityGroup>;

fn read_security_groups() -> Result<SecurityGroups, io::Error> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    let groups = reader
        .deserialize()
        .map(|r| r.expect("invalid row in CSV"))
        .collect();
    Ok(groups)
}

fn resources_section(groups: &SecurityGroups) -> Vec<String> {
    groups
        .iter()
        .map(|sg| {
            format!(
                include_str!("sg.tmpl"),
                name = sg.name,
                description = sg.description
            )
        })
        .collect()
}

fn outputs_section(groups: &SecurityGroups) -> Vec<String> {
    groups
        .iter()
        .map(|sg| {
            format!(
                include_str!("sg-out.tmpl"),
                name = sg.name,
                description = sg.description
            )
        })
        .collect()
}

fn main() -> Result<(), io::Error> {
    let groups = read_security_groups()?;
    println!(
        include_str!("main.tmpl"),
        resources = resources_section(&groups).join("\n"),
        outputs = outputs_section(&groups).join("\n")
    );
    Ok(())
}
