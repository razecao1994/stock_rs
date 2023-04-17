use tui::{style::Color, widgets::canvas::{Painter, Shape}};

#[derive(Debug)]
pub struct KLineData {
    pub start: usize,
    pub end: usize,
    pub min: usize,
    pub max: usize,
    pub date: usize,
}

///   •       •
///  •••     •••
///  •••  •  •••
///  •••  •  •••
///  ••• ••• •••
///  ••• ••• •••
///   •   •  •••
///           •
/// 
pub struct Kblock<'a> {
    pub coords: &'a [KLineData]
}

impl<'a> Shape for Kblock<'a> {
    fn draw(&self, painter: &mut Painter) {
        for k_line_data in self.coords {
            let mut color = Color::Red;
            let mut block_bottom = k_line_data.start;
            let mut block_top = k_line_data.end;
            // 开盘价 > 收盘价，显示为绿色
            if k_line_data.start > k_line_data.end {
                color = Color::Green;
                block_bottom = k_line_data.end;
                block_top = k_line_data.start;
            }
            // 绘制下影线
            draw_line(painter, k_line_data.min, block_bottom, k_line_data.date, color);
            // 绘制实体
            draw_block(painter, block_bottom, block_top, k_line_data.date, color);
            // 绘制上影线
            draw_line(painter, block_top, k_line_data.max, k_line_data.date, color);
        }
    }
}

fn draw_line(painter: &mut Painter, y1: usize, y2: usize, x: usize, color: Color) {
    for y in y1..=y2 {
        painter.paint(x, y, color);
    }
}

fn draw_block(painter: &mut Painter, y1: usize, y2: usize, x: usize, color: Color) {
    for y in y1..=y2 {
        for x in x-1..=x+1 {
            painter.paint(x, y, color);
        }
    }
}