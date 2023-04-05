mod app;
mod crossterm;
mod stock;
mod ui;

use crate::crossterm::run;
use std::{error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(250);
    run(tick_rate)?;
    // for id in STOCK_IDS {
    //     app::get_stock_datas(id).await?;
    // }
    Ok(())
}
