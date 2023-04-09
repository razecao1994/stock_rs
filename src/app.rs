use crate::{list::StatefulList, model::StockDataWrapper, net};
use std::error::Error;
use std::process::exit;

pub struct StockBriefInfo<'a> {
    pub stock_name: &'a str,
    pub stock_code: &'a str,
    pub stock_code_num: i32,
    pub stock_data: Option<StockDataWrapper>,
}

impl StockBriefInfo<'_> {
    pub async fn get_stock_datas(&self) -> Result<StockDataWrapper, Box<dyn Error>> {
        let stock_data = net::get_stock_datas(self.stock_code_num).await?;
        Ok(stock_data)
    }
}

pub struct App<'a> {
    pub stock_ids: StatefulList<StockBriefInfo<'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            stock_ids: StatefulList::with_items(vec![
                StockBriefInfo {
                    stock_name: "酒ETF",
                    stock_code: "SH512690",
                    stock_code_num: 512690,
                    stock_data: None,
                },
                StockBriefInfo {
                    stock_name: "新能源车ETF",
                    stock_code: "SH515030",
                    stock_code_num: 515030,
                    stock_data: None,
                },
                StockBriefInfo {
                    stock_name: "钢铁ETF",
                    stock_code: "SH515210",
                    stock_code_num: 515210,
                    stock_data: None,
                },
                StockBriefInfo {
                    stock_name: "地产ETF",
                    stock_code: "SZ159707",
                    stock_code_num: 159707,
                    stock_data: None,
                },
                StockBriefInfo {
                    stock_name: "创业板ETF",
                    stock_code: "SZ159915",
                    stock_code_num: 159915,
                    stock_data: None,
                },
            ]),
        }
    }

    pub fn on_key(&mut self, i: char) {
        match i {
            'i' => {
                // input
                log::info!("input")
            }
            'q' => {
                // quit
                self.stock_ids.unselect();
                exit(0)
            }
            _ => {}
        }
    }

    pub fn on_left(&self) {}

    pub fn on_up(&mut self) {
        self.stock_ids.previous();
    }

    pub fn on_right(&self) {}

    pub fn on_bottom(&mut self) {
        self.stock_ids.next();
    }

    pub fn selected_item_brief_info(
        &mut self,
    ) -> &StockBriefInfo {
        let i = match self.stock_ids.state.selected() {
            Some(i) => i,
            None => 0,
        };
        &self.stock_ids.items[i]
    }
}
