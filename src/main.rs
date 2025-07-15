use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use rand::{rng, seq::IteratorRandom};

static ADJECTIVES: &'static str = include_str!("adjectives.txt");
static ANIMALS: &'static str = include_str!("animals.txt");

/// A placeholder project name generator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of adjectives
    #[arg(short, long, default_value_t = 1)]
    length: usize,

    /// Number of project names
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Adjective word list
    #[arg(long, value_name = "FILE")]
    adjectives: Option<PathBuf>,

    /// Noun word list
    #[arg(long, value_name = "FILE")]
    nouns: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let adjectives_file = args.adjectives.map(|path| {
        read_to_string(&path).expect(&format!("Failed to read from {}!", path.display()))
    });
    let adjectives = adjectives_file
        .as_ref()
        .map(|content| content.lines())
        .unwrap_or_else(|| ADJECTIVES.lines());

    let nouns_file = args.nouns.map(|path| {
        read_to_string(&path).expect(&format!("Failed to read from {}!", path.display()))
    });
    let nouns = nouns_file
        .as_ref()
        .map(|content| content.lines())
        .unwrap_or_else(|| ANIMALS.lines());

    for _ in 0..args.count {
        print_name(adjectives.clone(), nouns.clone(), args.length);
    }
}

fn print_name<'a, 'b>(
    adjectives: impl IteratorRandom<Item = &'a str>,
    nouns: impl IteratorRandom<Item = &'b str>,
    length: usize,
) {
    let mut rng = rng();
    let adjectives = adjectives.choose_multiple(&mut rng, length);
    let animal = nouns.choose(&mut rng).expect("Empty animal word list!");

    for adjective in adjectives {
        print!("{} ", adjective);
    }
    println!("{}", animal);
}
