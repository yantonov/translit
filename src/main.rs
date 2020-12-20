use crate::cli::Command;

mod cli;

fn entry_point() -> Result<(), String> {
    match cli::arguments().command() {
        Command::Convert(convert) => {
            println!("{}", iuliia_rust::parse_by_schema_name(&convert.token(),
                                                             &convert.schema()));
        }
    }
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
