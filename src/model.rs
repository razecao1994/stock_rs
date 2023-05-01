use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Trend {
    end: f32,
    avg: f32,
    qty: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Szzs {
    name: String,
    #[serde(rename = "preClose")]
    pre_close: f32,
    trends: Vec<Trend>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Kline {
    end: f32,
    avg: f32,
    qty: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainInOut {
    list: Vec<i32>,
    max: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainIns {
    date: String,
    main: i32,
    bigger: i32,
    big: i32,
    #[serde(rename = "upRate")]
    up_rate: f32,
    price: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KLinesData {
    date: String,
    #[serde(rename = "preClose")]
    pre_close: f32,
    start: f32,
    end: f32,
    min: f32,
    max: f32,
    quantity: i32,
    amount: f64,
    #[serde(rename = "upRate")]
    up_rate: f32,
    cr: f32,
    #[serde(default, rename = "avg5")]
    avg_5: f32,
    #[serde(default, rename = "avg10")]
    avg_10: f32,
    #[serde(default, rename = "avg20")]
    avg_20: f32,
    #[serde(default, rename = "avg30")]
    avg_30: f32,
    #[serde(default, rename = "avgQ5")]
    avg_q5: f32,
    #[serde(default, rename = "avgQ10")]
    avg_q10: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Macd {
    dif: f32,
    dea: f32,
    macd: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Kdj {
    k: f32,
    d: f32,
    j: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    w: f32,
    n: f32,
    amount: i32,
    lb: i32,
    #[serde(rename = "mainIn")]
    main_in: i32,
    #[serde(rename = "mainOut")]
    main_out: Option<i32>,
    #[serde(rename = "biggerIn")]
    bigger_in: Option<i32>,
    #[serde(rename = "biggerOut")]
    bigger_out: Option<i32>,
    #[serde(rename = "bigIn")]
    big_in: i32,
    #[serde(rename = "bigOut")]
    big_out: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockData {
    #[serde(rename = "gpId")]
    gp_id: String,
    name: String,
    #[serde(rename = "bkName")]
    bk_name: String,
    #[serde(rename = "preClose")]
    pre_close: f32,
    #[serde(rename = "kLines")]
    k_lines: Vec<Kline>, // 当日的成交详情
    #[serde(rename = "mainInOut")]
    main_in_out: MainInOut, // 当日的主力买卖详情
    #[serde(rename = "pastMainIns")]
    past_main_ins: Vec<MainIns>, // 过去90天的主力买卖数据
    #[serde(rename = "pastKLines")]
    past_k_lines: Vec<KLinesData>, // 过去90天的k线
    macd: Vec<Macd>,
    kdj: Vec<Kdj>, // 过去90天的kdj
    max: f32,
    min: f32,
    view: String,           // k线级别，有day、week、month、5min~120min
    report: Option<Report>, // 今天的内盘、外盘、主力买卖统计
    #[serde(rename = "holdCount")]
    hold_count: i32,
    #[serde(rename = "isInterested")]
    is_interested: bool,
    #[serde(rename = "markPrice")]
    mark_price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockDataWrapper {
    pre: String,
    next: String,
    szzs: Szzs,
    data: Vec<StockData>,
}

impl StockDataWrapper {
    pub fn get_stock_datas(&self) -> &Vec<StockData> {
        &self.data
    }
}
