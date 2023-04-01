use clap::{self, arg, Command};

fn get_valid_port(s: &str) -> Result<u16, String> {
    let port: usize = s.parse().map_err(|_| format!("{s} is not a valid port"))?;

    if port > 65535 {
        return Err("Invalid port number".to_string());
    }
    Ok(port as u16)
}

pub fn clap_parser() -> clap::ArgMatches {
    return Command::new("webdir")
        .version("1.0")
        .about("Directory server like python http.server")
        .author("Odin")
        .arg(
            arg!(
                -p --port <PORT> "Port number to listen on"
            )
            .value_parser(get_valid_port)
            .default_value("8000"),
        )
        .arg(
            arg!(
                -b --bind <BIND> "Specify alternate bind address [default: all interfaces]"
            )
            .default_value("0.0.0.0"),
        )
        .arg(
            arg!(
                -d --dir <DIRECTORY> "Specify alternative directory [default:current directory"
            )
            .default_value("."),
        )
        .get_matches();
}
