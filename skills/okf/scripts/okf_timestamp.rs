//! ```cargo
//! [dependencies]
//! chrono = { version = "0.4", features = ["serde"] }
//! clap = { version = "4.5", features = ["derive"] }
//! ```

use chrono::{DateTime, FixedOffset, Local, Utc};
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "okf_timestamp",
    about = "Output RFC 3339 timestamp with explicit offset"
)]
struct Args {
    #[arg(short, long)]
    utc: bool,
}

fn main() {
    let args = Args::parse();
    let dt: DateTime<FixedOffset> = if args.utc {
        Utc::now().fixed_offset()
    } else {
        Local::now().fixed_offset()
    };
    println!("{}", dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, false));
}
