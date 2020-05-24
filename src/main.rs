use clap::Clap;
use chrono::prelude::*;
use radix_fmt::*;
use std::cmp::*;

const MILLIS_PER_DAY: i64 = 1000 * 60 * 60 * 24;
const TIMELS_PER_DAY: i64 = 12 * 12 * 12 * 12 * 12 * 12;

/// Prints the current dozenal time
#[derive(Clap)]
#[clap(author = "Agiel Negura <agiel.negura@gmail.com>")]
struct Opts {
    /// Number of digits, between 1 and 6
    #[clap(short, long, default_value = "3")]
    precision: u32,
    /// Character to use for 10
    #[clap(short, long, default_value = "X")]
    ten: String,
    /// Character to use for 10
    #[clap(short, long, default_value = "E")]
    eleven: String
}

fn main() {
    let opts: Opts = Opts::parse();
    let precision = max(1, min(6, opts.precision));

    let now = Local::now();
    let midnight = Local.ymd(
        now.year(), 
        now.month(), 
        now.day()
    ).and_hms(0, 0, 0);

    let duration = (now - midnight).num_milliseconds();
    let timels = duration * TIMELS_PER_DAY / MILLIS_PER_DAY;

    let display = radix_12(timels / 12i64.pow(6 - precision)).to_string()
        .replace("a", &opts.ten)
        .replace("b", &opts.eleven);

    println!("{}", display);
}
