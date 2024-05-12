use self::glucose_reading::GlucoseReading;
use self::unit::Unit;
use self::waybar::WaybarStats;
use crate::error::XdripError;
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use influxdb::ReadQuery;
use mangohud::MangoHudStats;
use state::XdripState;
use std::time::Duration;
use tokio::time::sleep;

mod direction;
mod error;
mod glucose_reading;
mod mangohud;
mod state;
mod status;
mod unit;
mod waybar;

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

trait XdripStats {
    fn output_reading(&mut self, reading: &GlucoseReading) -> Result<()>;

    fn output_error(&self, msg: String) -> Result<()>;

    fn state_mut(&mut self) -> &mut XdripState;

    async fn get_reading(&mut self) -> Result<GlucoseReading> {
        let query = ReadQuery::new(
            "SELECT value_mmol, value_mgdl, direction from glucose WHERE time > now() - 5m",
        );
        let mut db_result = tokio::time::timeout(
            Duration::from_secs(1),
            self.state_mut().client.json_query(query),
        )
        .await
        .map_err(|_| anyhow!(XdripError::RequestTimeout))?
        .map_err(|_| anyhow!(XdripError::RequestFailed))?;

        let mut val = db_result.deserialize_next::<GlucoseReading>()?;

        let glucose = val
            .series
            .iter_mut()
            .find(|s| s.name == "glucose")
            .ok_or_else(|| anyhow!(XdripError::GlucoseReadingOld))?;

        glucose
            .values
            .pop()
            .ok_or_else(|| anyhow!(XdripError::GlucoseReadingOld))
    }

    async fn run(&mut self) -> Result<()> {
        loop {
            match self.get_reading().await {
                Ok(reading) => {
                    self.output_reading(&reading)?;
                    self.state_mut().last_reading = Some(reading);
                }
                Err(e) => match e.downcast_ref::<XdripError>() {
                    Some(e) => self.output_error(e.to_string())?,
                    None => self.output_error(format!("Unexpected error: {e}"))?,
                },
            }
            sleep(Duration::from_secs(10)).await
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let state = XdripState::new(
        args.unit,
        args.influx_url,
        args.influx_database,
        args.influx_token,
    );

    match args.command {
        Commands::Waybar => WaybarStats::new(state).run().await,
        Commands::MangoHud => MangoHudStats::new(state).run().await,
    }
}
