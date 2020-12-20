use crate::cli::Command;

mod cli;

fn entry_point() -> Result<(), String> {
    match cli::arguments().command() {
        Command::Convert(convert) => {
            println!("{}", iuliia_rust::parse_by_schema_name(
                &convert.token(),
                &convert.schema()));
        }
        Command::ListSchemes(_) => {
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
            schemes.sort();
            for scheme in schemes.iter() {
                println!("{}", scheme);
            }
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
