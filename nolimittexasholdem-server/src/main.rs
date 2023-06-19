extern crate core;

use clap::{command, Arg, ArgAction};

mod cards;
mod game;
mod player;
mod raw_message;

/// Main entry point of the No Limit Texas Hold'em Server.
fn main() {
    let matches = command!()
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .action(ArgAction::Set)
                .help("Specify the port")
                .default_value("8080")
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            Arg::new("small_blind")
                .short('b')
                .long("small-blind")
                .action(ArgAction::Set)
                .help("Specify the small blind")
                .default_value("1")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("start_money")
                .short('m')
                .long("start-money")
                .action(ArgAction::Set)
                .help("Specify the start money")
                .default_value("1000")
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    async_std::task::block_on(async {
        game::start(
            async_std::future::pending(),
            *matches.get_one::<u16>("port").unwrap(),
            *matches.get_one::<usize>("small_blind").unwrap(),
            *matches.get_one::<usize>("start_money").unwrap(),
        )
        .await
        .unwrap();
    });
}
