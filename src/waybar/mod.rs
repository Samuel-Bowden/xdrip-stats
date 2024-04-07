use std::time::Duration;
use anyhow::Result;
use influxdb::ReadQuery;
use tokio::time::sleep;
use crate::glucose_reading::GlucoseReading;
use crate::status::Status;
use crate::XdripStats;
use self::output::Output;

mod output;
mod text;
mod tooltip;

pub struct WaybarStats<'a> {
    stats: &'a mut XdripStats,
}

impl<'a> WaybarStats<'a> {
    pub fn new(stats: &'a mut XdripStats) -> Self {
        Self {
            stats,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.iteration().await?;
            sleep(Duration::from_secs(10)).await
        }
    }


    async fn iteration(&mut self) -> Result<()> {
        let query = ReadQuery::new(
            "SELECT value_mmol, value_mgdl, direction from glucose WHERE time > now() - 5m",
        );
        let Ok(mut db_result) = self.stats.client.json_query(query).await else {
            return self.error("Failed to send request to database");
        };

        let mut val = db_result.deserialize_next::<GlucoseReading>()?;

        let Some(glucose) = val.series.iter_mut().find(|s| s.name == "glucose") else {
            return self.error("No glucose reading for past 5 minutes");
        };

        let Some(reading) = glucose.values.pop() else {
            return self.error("No glucose reading for past 5 minutes");
        };

        Output::new(&reading, &Status::Ok, self.stats.unit.clone()).print_json()?;
        self.stats.last_reading = Some(reading);

        Ok(())
    }

    fn error(&self, msg: &'static str) -> Result<()> {
        match &self.stats.last_reading {
            Some(reading) => {
                Output::new(reading, &Status::Error(msg), self.stats.unit.clone())
                    .print_json()
            }
            None => Output::error_with_no_data(msg).print_json(),
        }
    }
}
