use clap::{Command, Arg};
use std::error::Error;
use ulid::Ulid;
use hex;
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = create_cli().get_matches();

    match matches.subcommand() {
        Some(("find", matches)) => handle_find_command(matches),
        Some(("convert", matches)) => handle_convert_command(matches),
        Some(("db", matches)) => handle_db_command(matches),
        _ => {
            display_quick_help();
            exit(0);
        }
    }

    Ok(())
}

fn create_cli() -> Command {
    Command::new("ulid")
        .version("1.0.0")
        .about("ULID utility for working with BINARY(16) columns in databases")
        .subcommand(
            Command::new("find")
                .about("Generate SQL to find a record by ULID")
                .arg(
                    Arg::new("ULID")
                        .help("The ULID string to search for")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("table")
                        .short('t')
                        .long("table")
                        .value_name("TABLE")
                        .help("The table name")
                        .required(true)
                )
                .arg(
                    Arg::new("column")
                        .short('c')
                        .long("column")
                        .value_name("COLUMN")
                        .help("The column name (default: identifier)")
                        .default_value("identifier")
                )
        )
        .subcommand(
            Command::new("convert")
                .about("Convert between ULID and hex formats")
                .arg(
                    Arg::new("VALUE")
                        .help("ULID string or hex value to convert")
                        .required(true)
                        .index(1),
                )
        )
        .subcommand(
            Command::new("db")
                .about("Quick MySQL commands for the given ULID")
                .arg(
                    Arg::new("ULID")
                        .help("The ULID to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("table")
                        .short('t')
                        .long("table")
                        .value_name("TABLE")
                        .help("The table name")
                        .required(true)
                )
                .arg(
                    Arg::new("column")
                        .short('c')
                        .long("column")
                        .value_name("COLUMN")
                        .help("The column name (default: identifier)")
                        .default_value("identifier")
                )
                .arg(
                    Arg::new("copy")
                        .long("copy")
                        .help("Copy the SQL to clipboard")
                        .action(clap::ArgAction::SetTrue)
                )
        )
}

fn handle_find_command(matches: &clap::ArgMatches) {
    let ulid_str = matches.get_one::<String>("ULID").unwrap();
    let table = matches.get_one::<String>("table").unwrap();
    let column = matches.get_one::<String>("column").unwrap();

    let ulid = parse_ulid(ulid_str);
    let bytes = ulid.to_bytes();
    let hex_str = hex::encode(&bytes);

    println!("ULID: {}", ulid);
    println!("MySQL BINARY(16): 0x{}", hex_str);
    println!("\n-- Direct hex literal:");
    println!("SELECT * FROM `{}` WHERE `{}` = 0x{};", table, column, hex_str);
}

fn handle_convert_command(matches: &clap::ArgMatches) {
    let value = matches.get_one::<String>("VALUE").unwrap();

    if is_hex_format(value) {
        let clean_hex = value.trim_start_matches("0x");
        let bytes = parse_hex(clean_hex);
        let ulid = Ulid::from_bytes(bytes);

        println!("ULID: {}", ulid);
        println!("HEX:  0x{}", hex::encode(&bytes));
    } else {
        let ulid = parse_ulid(value);
        let bytes = ulid.to_bytes();

        println!("ULID: {}", ulid);
        println!("HEX:  0x{}", hex::encode(&bytes));
    }
}

fn handle_db_command(matches: &clap::ArgMatches) {
    let ulid_str = matches.get_one::<String>("ULID").unwrap();
    let table = matches.get_one::<String>("table").unwrap();
    let column = matches.get_one::<String>("column").unwrap();
    let should_copy = matches.get_flag("copy");

    let ulid = parse_ulid(ulid_str);
    let bytes = ulid.to_bytes();
    let hex_str = hex::encode(&bytes);

    let sql = format!("SELECT * FROM `{}` WHERE `{}` = 0x{};", table, column, hex_str);
    println!("{}", sql);

    if should_copy {
        copy_to_clipboard(&sql);
    }
}

fn display_quick_help() {
    println!("ULID Tool - Work with ULIDs and BINARY(16) columns\n");
    println!("Commands:");
    println!("  ulid convert <ULID_OR_HEX>                - Convert between ULID and hex");
    println!("  ulid find <ULID> --table <TABLE> [--column <COL>]     - Generate SQL to find a record");
    println!("  ulid db <ULID> --table <TABLE> [--column <COL>] [--copy]  - Quick SQL command");
    println!("\nExamples:");
    println!("  ulid convert 01J5KERCT1VJCKV1VEVZZ3NFY4");
    println!("  ulid find 01J5KERCT1VJCKV1VEVZZ3NFY4 --table users");
    println!("  ulid db 01J5KERCT1VJCKV1VEVZZ3NFY4 --table products --column product_id --copy");
    println!("\nFor more details, run: ulid --help");
}

fn parse_ulid(ulid_str: &str) -> Ulid {
    match Ulid::from_string(ulid_str) {
        Ok(ulid) => ulid,
        Err(e) => {
            eprintln!("Error: Could not parse '{}' as ULID: {}", ulid_str, e);
            exit(1);
        }
    }
}

fn is_hex_format(value: &str) -> bool {
    value.starts_with("0x") || (value.len() == 32 && value.chars().all(|c| c.is_ascii_hexdigit()))
}

fn parse_hex(hex_str: &str) -> [u8; 16] {
    match hex::decode(hex_str) {
        Ok(b) => {
            if b.len() != 16 {
                eprintln!("Error: Expected 16 bytes, got {}", b.len());
                exit(1);
            }
            let mut arr = [0u8; 16];
            arr.copy_from_slice(&b);
            arr
        },
        Err(e) => {
            eprintln!("Error: Could not parse hex string: {}", e);
            exit(1);
        }
    }
}

fn copy_to_clipboard(text: &str) {
    use std::process::{Command as ProcessCommand, Stdio};
    use std::io::Write;

    let mut child = match ProcessCommand::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Error: Could not copy to clipboard: {}", e);
            exit(1);
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        if let Err(e) = stdin.write_all(text.as_bytes()) {
            eprintln!("Error: Could not copy to clipboard: {}", e);
            exit(1);
        }
    }

    println!("SQL copied to clipboard!");
}