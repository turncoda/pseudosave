use clap::Parser;
use std::fs::File;
use gvas::GvasFile;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Save file
    #[arg(short, long)]
    file: String,

    /// Whether to show upgrades only
    #[arg(short, long, default_value_t = false)]
    upgrades_only: bool,

    /// Whether to show names only
    #[arg(short, long, default_value_t = false)]
    names_only: bool,
}

fn main() {
    let args = Args::parse();
    let mut file = File::open(args.file).unwrap();
    let gvas_file = GvasFile::read(&mut file).unwrap();
    if args.upgrades_only && args.names_only {
        let index_map = &gvas_file.properties.get("upgrades").unwrap().get_map().unwrap().value;
        for (k, _) in index_map {
            println!("{}", &k.get_name().unwrap().value);
        }
    } else if !args.upgrades_only && args.names_only {
        panic!();
    } else if args.upgrades_only && !args.names_only {
        let index_map = &gvas_file.properties.get("upgrades").unwrap().get_map().unwrap().value;
        for (k, v) in index_map {
            println!("{} {}", &v.get_int().unwrap().value, &k.get_name().unwrap().value);
        }
    }else {
        println!("{:#?}", gvas_file);
    }
}
