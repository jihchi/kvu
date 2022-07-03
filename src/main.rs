use getopts::Options;
use std::{collections::HashMap, env, io};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] KEY=VALUE...", program);
    println!("{}", opts.usage(&brief));
}

fn match_line<'a, 'b>(
    line: &'a str,
    candidates: &HashMap<&'b str, &'b str>,
) -> Option<(&'a str, &'b str)> {
    line.split_once('=')
        .and_then(|(key, _)| candidates.get(key).map(|&value| (key, value)))
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let program = args[0].clone();
    let lines = io::stdin().lines();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("{}", e.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }

    let candidates: HashMap<&str, &str> = matches
        .free
        .iter()
        .filter_map(|candidate| candidate.split_once('='))
        .collect();

    for line in lines {
        let line = line?;
        if let Some((key, value)) = match_line(&line, &candidates) {
            println!("{}={}", key, value);
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
