use crate::stock::StockDataWrapper;

pub const STOCK_IDS: [i32; 1] = [
    515030, // 新能源车ETF
];

const URL: &str = "http://xxx.xxx.xxx/";

pub async fn get_stock_datas(gp_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut url = String::from(URL);
    url.push_str("?block=");
    url.push_str(gp_id.to_string().as_str());
    url.push_str("&type=local&view=dtl");
    let resp = reqwest::get(url).await?.text().await?;
    let deserialized: StockDataWrapper = serde_json::from_str(&resp).unwrap();
    println!("getStockDatas: {:?}", deserialized.getStockData()[0]);
    Ok(())
}
