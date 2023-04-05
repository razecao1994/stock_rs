use serde::{Serialize, Deserialize};

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

/// ```
/// use stock_rs::stock::Kline;
/// let kline = Kline::new(1.639, 1.639, 13552);
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Kline {
    end: f32,
    avg: f32,
    qty: i32,
}

impl Kline {
    pub fn new(end: f32, avg: f32, qty: i32) -> Self {
        Kline { end, avg, qty }
    }
}

/// ```
/// use stock_rs::stock::MainInOut;
/// let mut list = Vec::new();
/// list.push(-254);
/// list.push(-47);
/// list.push(-207);
/// let main_in_out = MainInOut::new(list, 11475);
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct MainInOut {
    list: Vec<i32>,
    max: i32,
}

impl MainInOut {
    pub fn new(list: Vec<i32>, max: i32) -> Self {
        MainInOut { list, max }
    }
}

/// ```
/// use stock_rs::stock::MainIns;
///
/// let main_ins = MainIns::new(
///     String::from("2023-04-04"),
///     -9184,
///     -11475,
///     2291,
///     -2.62,
///     1.597,
/// );
/// ```
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

impl MainIns {
    pub fn new(date: String, main: i32, bigger: i32, big: i32, up_rate: f32, price: f32) -> Self {
        MainIns {
            date,
            main,
            bigger,
            big,
            up_rate,
            price,
        }
    }
}

/// ```
/// use stock_rs::stock::KLinesData;
///
/// let k_lines_data = KLinesData::new(
///     String::from("2023-04-04"),
///     1.64,
///     1.639,
///     1.597,
///     1.589,
///     1.639,
///     2998423,
///     480608548,
///     -2.62,
///     4.18,
///     1.623,
///     1.619,
///     1.616,
///     1.649,
///     2086039.8,
///     1961367.4
/// );
/// ```
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

impl KLinesData {
    pub fn new(
        date: String,
        pre_close: f32,
        start: f32,
        end: f32,
        min: f32,
        max: f32,
        quantity: i32,
        amount: f64,
        up_rate: f32,
        cr: f32,
        avg_5: f32,
        avg_10: f32,
        avg_20: f32,
        avg_30: f32,
        avg_q5: f32,
        avg_q10: f32,
    ) -> Self {
        KLinesData {
            date,
            pre_close,
            start,
            end,
            min,
            max,
            quantity,
            amount,
            up_rate,
            cr,
            avg_5,
            avg_10,
            avg_20,
            avg_30,
            avg_q5,
            avg_q10,
        }
    }
}

/// ```
/// use stock_rs::stock::Macd;
///
/// let macd = Macd::new(-0.025, -0.033, 0.014);
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Macd {
    dif: f32,
    dea: f32,
    macd: f32,
}

impl Macd {
    pub fn new(dif: f32, dea: f32, macd: f32) -> Self {
        Macd { dif, dea, macd }
    }
}

/// ```
/// use stock_rs::stock::Kdj;
///
/// let kdj = Kdj::new(53.618, 63.915, 33.023);
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Kdj {
    k: f32,
    d: f32,
    j: f32,
}

impl Kdj {
    pub fn new(k: f32, d: f32, j: f32) -> Self {
        Kdj { k, d, j }
    }
}

/// ```
/// use stock_rs::stock::Report;
///
/// let report = Report::new(
///     121.9,
///     177.9,
///     48061,
///     176,
///     23664,
///     32848,
///     6345,
///     17820,
///     17319,
///     17319
/// );
/// ```
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

impl Report {
    pub fn new(
        w: f32,
        n: f32,
        amount: i32,
        lb: i32,
        main_in: i32,
        main_out: Option<i32>,
        bigger_in: Option<i32>,
        bigger_out: Option<i32>,
        big_in: i32,
        big_out: i32,
    ) -> Self {
        Report {
            w,
            n,
            amount,
            lb,
            main_in,
            main_out,
            bigger_in,
            bigger_out,
            big_in,
            big_out,
        }
    }
}

/// ```
/// use stock_rs::stock::Kline;
/// use stock_rs::stock::MainInOut;
/// use stock_rs::stock::MainIns;
/// use stock_rs::stock::KLinesData;
/// use stock_rs::stock::Macd;
/// use stock_rs::stock::Kdj;
/// use stock_rs::stock::Report;
/// use stock_rs::stock::StockData;
/// 
/// let k_line = Kline::new(1.639, 1.639, 13552);
/// 
/// let mut k_lines = Vec::new();
/// k_lines.push(k_line);
/// 
/// let mut list = Vec::new();
/// list.push(-254);
/// list.push(-47);
/// list.push(-207);
/// let main_in_out = MainInOut::new(list, 11475);
///
/// let main_ins = MainIns::new(
///     String::from("2023-04-04"),
///     -9184,
///     -11475,
///     2291,
///     -2.62,
///     1.597,
/// );
/// 
/// let mut past_main_ins = Vec::new();
/// past_main_ins.push(main_ins); 
/// 
/// let k_lines_data = KLinesData::new(
///     String::from("2023-04-04"),
///     1.64,
///     1.639,
///     1.597,
///     1.589,
///     1.639,
///     2998423,
///     480608548,
///     -2.62,
///     4.18,
///     1.623,
///     1.619,
///     1.616,
///     1.649,
///     2086039.8,
///     1961367.4
/// );
/// 
/// let mut past_k_lines = Vec::new();
/// past_k_lines.push(k_lines_data);
/// 
/// let macd = Macd::new(-0.025, -0.033, 0.014);
/// 
/// let mut macds = Vec::new();
/// macds.push(macd);
/// 
/// let kdj = Kdj::new(53.618, 63.915, 33.023);
/// 
/// let mut kdjs = Vec::new();
/// kdjs.push(kdj);
/// 
/// let report = Report::new(
///     121.9,
///     177.9,
///     48061,
///     176,
///     23664,
///     32848,
///     6345,
///     17820,
///     17319,
///     17319
/// );
/// 
/// let stock_data = StockData::new(
///     String::from("515030"),
///     String::from("新能源车ETF"),
///     String::from(""),
///     1.64,
///     k_lines,
///     main_in_out,
///     past_main_ins,
///     past_k_lines,
///     macds,
///     kdjs,
///     1.93,
///     1.548,
///     String::from("day"),
///     report,
///     0,
///     false,
///     0,
/// );
/// ```
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
    k_lines: Vec<Kline>,           // 当日的成交详情
    #[serde(rename = "mainInOut")]
    main_in_out: MainInOut,        // 当日的主力买卖详情
    #[serde(rename = "pastMainIns")]
    past_main_ins: Vec<MainIns>,   // 过去90天的主力买卖数据
    #[serde(rename = "pastKLines")]
    past_k_lines: Vec<KLinesData>, // 过去90天的k线
    macd: Vec<Macd>,
    kdj: Vec<Kdj>, // 过去90天的kdj
    max: f32,
    min: f32,
    view: String,   // k线级别，有day、week、month、5min~120min
    report: Option<Report>, // 今天的内盘、外盘、主力买卖统计
    #[serde(rename = "holdCount")]
    hold_count: i32,
    #[serde(rename = "isInterested")]
    is_interested: bool,
    #[serde(rename = "markPrice")]
    mark_price: i32,
}

impl StockData {
    pub fn new(
        gp_id: String,
        name: String,
        bk_name: String,
        pre_close: f32,
        k_lines: Vec<Kline>,
        main_in_out: MainInOut,
        past_main_ins: Vec<MainIns>,
        past_k_lines: Vec<KLinesData>,
        macd: Vec<Macd>,
        kdj: Vec<Kdj>,
        max: f32,
        min: f32,
        view: String,
        report: Option<Report>,
        hold_count: i32,
        is_interested: bool,
        mark_price: i32,
    ) -> Self {
        StockData {
            gp_id,
            name,
            bk_name,
            pre_close,
            k_lines,
            main_in_out,
            past_main_ins,
            past_k_lines,
            macd,
            kdj,
            max,
            min,
            view,
            report,
            hold_count,
            is_interested,
            mark_price,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockDataWrapper {
    pre: String,
    next: String,
    szzs: Szzs,
    data: Vec<StockData>,
}

impl StockDataWrapper {
    pub fn getStockData(&self) -> &Vec<StockData> {
        &self.data
    }
}