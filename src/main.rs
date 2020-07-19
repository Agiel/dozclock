use clap::Clap;
use chrono::prelude::*;
use radix_fmt::radix_12;
use std::cmp::{min, max};
use std::time::Duration;
use std::thread::sleep;

const SECONDS_PER_DAY: f64 = 60. * 60. * 24.;
const TIMELS_PER_DAY: f64 = 12. * 12. * 12. * 12. * 12. * 12.;

/// Prints the current dozenal time
#[derive(Clap)]
#[clap(author = "Agiel Negura <agiel.negura@gmail.com>")]
struct Opts {
    /// Number of digits, between 1 and 6
    #[clap(short, long, default_value = "3")]
    precision: u32,
    /// Character to use for 10
    #[clap(short, long, default_value = "T")]
    ten: String,
    /// Character to use for 10
    #[clap(short, long, default_value = "E")]
    eleven: String,
    /// Continuous output
    #[clap(short, long)]
    continuous: bool,
}

fn timels_to_duration(timels: f64) -> Duration {
    Duration::from_secs_f64(timels * SECONDS_PER_DAY / TIMELS_PER_DAY)
}

fn duration_to_timels(duration: Duration) -> f64 {
    duration.as_secs_f64() * TIMELS_PER_DAY / SECONDS_PER_DAY
}

fn get_time_as_timels() -> f64 {
    let now = Local::now();
    let midnight = Local.ymd(
        now.year(), 
        now.month(), 
        now.day()
    ).and_hms(0, 0, 0);

    duration_to_timels((now - midnight).to_std().unwrap())
}

fn print(timels: f64, opts: &Opts) {
    let display = radix_12(timels as u64 / 12u64.pow(6 - opts.precision)).to_string()
        .replace("a", &opts.ten)
        .replace("b", &opts.eleven);

    println!("{}", display);
}

fn main() {
    let mut opts: Opts = Opts::parse();
    opts.precision = max(1, min(6, opts.precision));

    let mut timels = get_time_as_timels();
    print(timels, &opts);

    if !opts.continuous {
        return;
    }

    let timels_per_tick = 12u64.pow(6 - opts.precision) as f64;
    loop {
        let drift = (timels / timels_per_tick).fract() * timels_per_tick;
        sleep(timels_to_duration(timels_per_tick - drift));
        timels = get_time_as_timels();
        print(timels, &opts);
    }
}
