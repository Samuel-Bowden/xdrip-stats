use anyhow::Result;
use clap::{Parser, Subcommand};
use influxdb::Client;
use self::glucose_reading::GlucoseReading;
use self::unit::Unit;
use self::waybar::WaybarStats;

mod direction;
mod glucose_reading;
mod unit;
mod status;
mod waybar;

#[derive(Subcommand)]
enum Commands {
    Waybar,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[clap(long, env)]
    influx_token: String,
    #[clap(long, env)]
    influx_url: String,
    #[clap(long, env)]
    influx_database: String,
    #[clap(short, long, default_value = "mmol")]
    unit: Unit,
}

struct XdripStats {
    client: Client,
    unit: Unit,
    last_reading: Option<GlucoseReading>,
}

impl XdripStats {
    pub async fn run() -> Result<()> {
        let args = Args::parse();

        let mut this = Self {
            client: Client::new(args.influx_url, args.influx_database)
                .with_token(args.influx_token),
            unit: args.unit,
            last_reading: None,
        };

        match args.command {
            Commands::Waybar => {
                let mut waybar_output = WaybarStats::new(&mut this);
                waybar_output.run().await?
            },
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    XdripStats::run().await
}
