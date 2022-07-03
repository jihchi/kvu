use getopts::Options;
use std::{collections::HashMap, env, io};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] KEY=VALUE...", program);
    println!("{}", opts.usage(&brief));
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let program = args[0].clone();
    let lines = io::stdin().lines();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the version");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("{}", e.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    } else if matches.opt_present("v") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let mut candidates: HashMap<&str, (&str, bool)> = matches
        .free
        .iter()
        .filter_map(|candidate| {
            candidate
                .split_once('=')
                .map(|(key, value)| (key, (value, false)))
        })
        .collect();

    for line in lines {
        let line = line?;
        let pair = line.split_once('=').and_then(|(key, _)| {
            candidates.get_mut(key).map(|(value, found)| {
                *found = true;
                (key, value)
            })
        });

        if let Some((key, value)) = pair {
            println!("{}={}", key, value);
        } else {
            println!("{}", line);
        }
    }

    for (key, (value, _)) in candidates.iter().filter(|(_, (_, found))| !found) {
        println!("{}={}", key, value);
    }

    Ok(())
}
