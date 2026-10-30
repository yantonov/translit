use crate::cli::{Command, SchemesSubcommand};

mod cli;
mod version;

fn schemes() -> Vec<&'static str> {
    let mut schemes = vec![
        "ala_lc",
        "ala_lc_alt",
        "bgn_pcgn",
        "bgn_pcgn_alt",
        "bs_2979",
        "bs_2979_alt",
        "gost_52290",
        "gost_7034",
        "icao_doc_9303",
        "gost_779",
        "gost_779_alt",
        "ungegn_1987",
        "mosmetro",
        "scientific",
        "telegram",
        "wikipedia",
        "yandex_maps",
        "yandex_money",
    ];
    schemes.sort_unstable();
    schemes
}

fn entry_point() -> Result<(), String> {
    match cli::arguments().command() {
        Command::Convert(convert) => {
            if convert.all() {
                for (index, scheme) in schemes().iter().enumerate() {
                    if index > 0 {
                        println!();
                    }
                    println!("scheme: {}", scheme);
                    println!(
                        "{}",
                        iuliia_rust::parse_by_schema_name(convert.token(), scheme)
                    );
                }
            } else if let Some(schema) = convert.schema() {
                println!(
                    "{}",
                    iuliia_rust::parse_by_schema_name(convert.token(), schema)
                );
            }
        }
        Command::Schemes(schema_command) => match schema_command.subcommand() {
            SchemesSubcommand::List(_) => {
                for scheme in schemes().iter() {
                    println!("{}", scheme);
                }
            }
        },
        Command::Version => {
            println!("{}", version::version_info());
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
