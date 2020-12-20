fn entry_point() -> Result<(), String> {
    let args: Vec<String> = std::env::args()
        .into_iter()
        .collect();
    let token = args.get(1)
        .expect("first argument required - token to transliterate");
    let schema = args.get(2)
        .expect("schema is required as second parameter");
    println!("{}", iuliia_rust::parse_by_schema_name(token, schema));
    Ok(())
}

fn main() {
    match entry_point() {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(error_message) => {
            eprintln!("{}", error_message);
            std::process::exit(1);
        }
    }
}
