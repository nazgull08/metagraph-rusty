mod conf;

#[macro_use] extern crate rocket;

use std::io::Write;

use clap::Parser;
use log::LevelFilter;
use log::Level::*;
use chrono::Local;
use env_logger::Builder;

use conf::load_config;

#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Parser, Debug, Clone)]
enum SubCommand {
    Serve {
        #[clap(
            long,
            short,
            env = "RUSTETH_CONFIG"
        )]
        config: String
    }
}

#[launch]
async fn rocket() -> _ {
    let args = Args::parse();
    match args.subcmd.clone() {
        SubCommand::Serve {
            config
        } => { println!("args: {:0}", config);}
    }

    let cfg = load_config("config.json");

    Builder::new()
        .format(|buf, record| {
            let mut level_style = buf.style();
            match record.level() {
                Error => {level_style.set_color(env_logger::fmt::Color::Red).set_bold(true);},
                Info  => {level_style.set_color(env_logger::fmt::Color::Green).set_bold(true);},
                Warn  => {level_style.set_color(env_logger::fmt::Color::Yellow).set_bold(true);},
                Debug => {level_style.set_color(env_logger::fmt::Color::Blue).set_bold(true);},
                Trace => {level_style.set_color(env_logger::fmt::Color::White).set_bold(true);},
            };
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                level_style.value(record.level()),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    rocket::build()
        .manage(cfg)
}
