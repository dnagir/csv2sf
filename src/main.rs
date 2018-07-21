struct SecurityGroup {
    name: String,
    description: String,
}

fn main() {
    let groups = vec![
        SecurityGroup {
            name: "asd".to_string(),
            description: "ddd".to_string(),
        },
        SecurityGroup {
            name: "bc".to_string(),
            description: "def".to_string(),
        },
    ];

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
