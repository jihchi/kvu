use getopts::Options;
use kvu::{Operation, FLAG_CREATE, FLAG_DELETE, FLAG_UPDATE};
use std::collections::{HashMap, HashSet};
use std::{env, io};

fn print_usage(program: &str, options: Options) {
    let brief = format!("Usage: {} [options] KEY=VALUE...", program);
    println!("{}", options.usage(&brief));
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let program = args[0].clone();
    let lines = io::stdin().lines();
    let mut options = Options::new();

    options.optflag("h", "help", "print this help menu");
    options.optflag("v", "version", "print the version");

    options.optmulti(
        "c",
        FLAG_CREATE,
        "create the pair only if the key does not exist",
        "KEY=VALUE",
    );

    options.optmulti(
        "u",
        FLAG_UPDATE,
        "update the pair only if the key does exist",
        "KEY=VALUE",
    );

    options.optmulti("d", FLAG_DELETE, "delete the pair by the key", "KEY");

    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("{}", e.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, options);
        return Ok(());
    } else if matches.opt_present("v") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let pairs_create = matches.opt_strs(FLAG_CREATE);
    let pairs_update = matches.opt_strs(FLAG_UPDATE);
    let pairs_delete = matches.opt_strs(FLAG_DELETE);
    let mut operations: HashMap<&str, Operation> = HashMap::default();
    let mut to_create: HashSet<&str> = HashSet::default();

    matches
        .free
        .iter()
        .filter_map(|pair| pair.split_once('='))
        .for_each(|(key, value)| {
            operations.insert(key, Operation::Upsert(value));
            to_create.insert(key);
        });

    pairs_create
        .iter()
        .filter_map(|pair| pair.split_once('='))
        .for_each(|(key, value)| {
            operations.insert(key, Operation::Create(value));
            to_create.insert(key);
        });

    pairs_update
        .iter()
        .filter_map(|pair| pair.split_once('='))
        .for_each(|(key, value)| {
            operations.insert(key, Operation::Update(value));
        });

    pairs_delete.iter().for_each(|key| {
        operations.insert(key, Operation::Delete);
        to_create.remove(key.as_str());
    });

    for line in lines {
        let line = line?;

        if let Some((key, operation)) = line
            .split_once('=')
            .and_then(|(key, _)| operations.get(key).map(|operation| (key, operation)))
        {
            match operation {
                Operation::Create(_) => println!("{}", line),
                Operation::Update(value) | Operation::Upsert(value) => {
                    println!("{}={}", key, value)
                }
                Operation::Delete => (),
            }
            // don't create new key-value pair when the key already exists
            match operation {
                Operation::Create(_) | Operation::Update(_) | Operation::Upsert(_) => {
                    to_create.remove(key);
                }
                Operation::Delete => (),
            }
        } else {
            println!("{}", line);
        }
    }

    for (key, value) in operations
        .iter()
        .filter(|(&key, _)| to_create.contains(key))
        .filter_map(|(key, operation)| match operation {
            Operation::Create(value) | Operation::Upsert(value) => Some((key, value)),
            Operation::Update(_) | Operation::Delete => None,
        })
    {
        println!("{}={}", key, value);
    }

    Ok(())
}
