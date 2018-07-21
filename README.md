## Generate a CloudFormation from a CSV

This is just a playground to do something with Rust but some interesting points are:

- it is using compile-time "templating". For example: it won't compile if you'll add a new variable to template but will not pass it in or the template file will be missing.
- has relatively strict CSV parsing with somewhat useful error messages (kind-of getting this for free with the `Result`)


The "templating" is done using built-in "string formatting tools" (`println!` and `format!`) for simplicity so there are 3 templates that are combined in the end.
Wanted something simple without bringing in a templating tool.
It turns out it is quite nice actually with the compile-time checks.

The CSV parsing: first I tried using the simple CSV crate which was fine. But was curious about the `sarde` deserialization.
A lot more dependencies than necessary with Sarde for this purpose, but the code looks nicer. So I decided to commit and leave it.


## Usage

```
cargo run < groups.csv
echo "FooApi,permitted to access Foo API over gRPC" | cargo run
```

Binary:

```
cargo build --release && ll -sh target/release/csv2cf
1304 -rwxr-xr-x  2 dima  staff   650K 22 Jul 02:04 target/release/csv2cf
```
