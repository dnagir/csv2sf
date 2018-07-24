extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate itertools;
use std::io;
use itertools::Itertools;

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

fn resources_section<'a>(groups: &'a SecurityGroups) -> impl Iterator<Item = String> + 'a {
    groups.iter().map(|sg| {
        format!(
            include_str!("sg.tmpl"),
            name = sg.name,
            description = sg.description
        )
    })
}

fn outputs_section<'a>(groups: &'a SecurityGroups) -> impl Iterator<Item = String> + 'a {
    groups.iter().map(|sg| {
        format!(
            include_str!("sg-out.tmpl"),
            name = sg.name,
            description = sg.description
        )
    })
}

fn main() -> Result<(), io::Error> {
    let groups = read_security_groups()?;
    println!(
        include_str!("main.tmpl"),
        resources = resources_section(&groups).format("\n"),
        outputs = outputs_section(&groups).join("\n")
    );
    Ok(())
}
