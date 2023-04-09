use crate::model::StockDataWrapper;
use std::error::Error;

pub async fn get_stock_datas(gp_id: i32) -> Result<StockDataWrapper, Box<dyn Error>> {
    let mut url = String::from("http://xxx.xx.xx.xx:xxxxx");
    url.push_str("?block=");
    url.push_str(gp_id.to_string().as_str());
    url.push_str("&type=local&view=dtl");
    let resp = reqwest::get(url).await?;
    let result = resp.json::<StockDataWrapper>().await?;
    Ok(result)
}
