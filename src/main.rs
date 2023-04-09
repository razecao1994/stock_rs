mod app;
mod list;
mod model;
mod net;
mod ui;
use crate::app::App;
use crate::ui::run;
use std::{error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    run(app, tick_rate)?;
    Ok(())
    // let json = app::get_stock_datas(512690).await?;
    // println!("{:?}", json);
    // Ok(())
}
