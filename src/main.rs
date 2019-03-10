extern crate clap;
extern crate speedtestr;
#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::*;
use clap::{Arg, App, AppSettings, SubCommand};
use speedtestr::{server, server::Server};

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
        ]
    ).unwrap();
    info!("speedtestr init");


    let app = App::new("speedtestr")
        .version("0.0.1")
        .about("Unofficial speedtest cli")
        .author("Zach Peters")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("list").about("Lists available servers"))
        .subcommand(SubCommand::with_name("ping")
                    .about("Pings the best server")
                    .arg(Arg::with_name("numservers")
                         .short("n")
                         .takes_value(true)
                         .help("Number of servers to test with")
                    )
                    .arg(Arg::with_name("numpings")
                         .short("p")
                         .takes_value(true)
                         .help("Number of pings to test with")
                    )
                    .arg(Arg::with_name("server")
                         .short("s")
                         .takes_value(true)
                         .help("specify a server number to ping")))
        .get_matches();

    info!("Arguments {:#?}", app);

    if app.is_present("list") {
        let resp = server::list_servers();
        match resp {
            Ok(n) => print_servers(n),
            Err(n) => error!("Err: {}", n),
        }
    }

    if let Some(app) = app.subcommand_matches("ping") {
        let best;
        let num_best = app.value_of("numservers").unwrap_or("3");
        let num_pings = app.value_of("numpings").unwrap_or("3").parse::<u128>().unwrap();
        info!("Ping test (servers: {}/Given? {}, pings: {}/Given? {})", num_best, app.is_present("numservers"), num_pings, app.is_present("numpings"));

        println!("[ping test]");
        let svr =
            if app.is_present("server") {
                app.value_of("server").unwrap()
            } else {
                best = server::best_server(num_best).unwrap().to_owned();
                best.id.as_str()
            };

        let resp = server::ping_server(svr, num_pings);
        println!("Avg ms: {}", resp.unwrap());
    }

    fn print_servers(servers: Vec<Server>) {
        for s in servers {
            println!("{} - [distance {}] - ({}) {} {}", s.id, s.distance, s.sponsor, s.name, s.cc);
        };
    }

}
