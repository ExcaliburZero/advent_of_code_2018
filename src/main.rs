extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate advent_of_code_2018;

fn main() {
    let matches =
        App::new("advent_of_code_2018")
            .version("day1")
            .author("Christopher Wells <cwellsny@nycap.rr.com>")
            .subcommand(SubCommand::with_name("day1")
                .arg(Arg::with_name("part")
                    .help("Selects the part to run (one, two)")
                    .required(true)
                    .index(1)
                )
            )
            .subcommand(SubCommand::with_name("day2")
                .arg(Arg::with_name("part")
                    .help("Selects the part to run (one, two)")
                    .required(true)
                    .index(1)
                )
            )
            .subcommand(SubCommand::with_name("day3")
                .arg(Arg::with_name("part")
                    .help("Selects the part to run (one, two)")
                    .required(true)
                    .index(1)
                )
            )
        .get_matches()
    ;

    if let Some(matches) = matches.subcommand_matches("day1") {
        let part = matches.value_of("part").unwrap();

        match part {
            "one" => { advent_of_code_2018::one::part_one(); }
            "two" => { advent_of_code_2018::one::part_two(); }
            p => { println!("Unknown part: {}", p); }
        }
    } else if let Some(matches) = matches.subcommand_matches("day2") {
        let part = matches.value_of("part").unwrap();

        match part {
            "one" => { advent_of_code_2018::two::part_one(); }
            "two" => { advent_of_code_2018::two::part_two(); }
            p => { println!("Unknown part: {}", p); }
        }
    } else if let Some(matches) = matches.subcommand_matches("day3") {
        let part = matches.value_of("part").unwrap();

        match part {
            "one" => { advent_of_code_2018::three::part_one(); }
            "two" => { advent_of_code_2018::three::part_two(); }
            p => { println!("Unknown part: {}", p); }
        }
    }
}
