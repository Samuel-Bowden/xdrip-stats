use std::time::Duration;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use influxdb::{Client, ReadQuery};
use tokio::time::sleep;
use self::glucose_reading::GlucoseReading;
use self::unit::Unit;
use self::waybar::WaybarStats;
use self::mangohud::MangoHudStats;

mod direction;
mod glucose_reading;
mod unit;
mod status;
mod waybar;
mod mangohud;

#[derive(Subcommand)]
enum Commands {
    Waybar,
    MangoHud,
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

#[derive(Debug)]
enum XdripError {
    RequestFailed,
    GlucoseReadingOld
}

impl std::fmt::Display for XdripError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestFailed => write!(f, "Failed to send request to database"),
            Self::GlucoseReadingOld => write!(f, "No glucose reading for past 5 minutes"),
        }
    }
}

impl std::error::Error for XdripError {}

trait XdripStats {
    fn output_reading(&mut self, reading: GlucoseReading) -> Result<()>;

    fn output_error(&self, msg: String) -> Result<()>;

    async fn get_reading(&self, client: &mut Client) -> Result<GlucoseReading> {
        let query = ReadQuery::new(
            "SELECT value_mmol, value_mgdl, direction from glucose WHERE time > now() - 5m",
        );
        let mut db_result = client.json_query(query).await.map_err(|_| anyhow!(XdripError::RequestFailed))?;

        let mut val = db_result.deserialize_next::<GlucoseReading>()?;

        let glucose = val.series.iter_mut().find(|s| s.name == "glucose").ok_or_else(|| anyhow!(XdripError::GlucoseReadingOld))?;

        glucose.values.pop().ok_or_else(|| anyhow!(XdripError::GlucoseReadingOld))
    }

    async fn run(&mut self, url: String, database: String, token: String) -> Result<()> {
        let mut client = Client::new(url, database).with_token(token);

        loop {
            match self.get_reading(&mut client).await {
                Ok(reading) => {
                    self.output_reading(reading)?;
                }
                Err(e) => match e.downcast_ref::<XdripError>() {
                    Some(e) => self.output_error(e.to_string())?,
                    None => self.output_error(format!("Unexpected error: {e}"))?,
                }
            }
            sleep(Duration::from_secs(10)).await
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Waybar => {
            WaybarStats::new(args.unit).run(args.influx_url, args.influx_database, args.influx_token).await
        }
        Commands::MangoHud => {
            MangoHudStats::new(args.unit).run(args.influx_url, args.influx_database, args.influx_token).await
        }
    }
}
