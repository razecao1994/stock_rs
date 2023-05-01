use tui::{
    style::Color,
    widgets::canvas::{Line, Painter, Shape},
};

/// 0.970
///  •   •       •
///  •  •••     •••  •
///  •  • •  •  • • •••
///  •  • •  •  • • • •
///  •  • • ••• • • • •
///  •  ••• ••• • • • •
///  •   •   •  ••• • •
///  •           •  • •
///  •              •••
///  •               •
///  •               •
///  •••••••••••••••••••••••••••••••••••••
/// 4-15           4-20
#[derive(Debug, Clone)]
pub struct Kblock {
    pub x: f64,
    pub min: f64,
    pub block_bottom: f64,
    pub block_top: f64,
    pub max: f64,
    pub color: Color,
}

impl Shape for Kblock {
    fn draw(&self, painter: &mut Painter) {
        let lines: [Line; 6] = [
            Line {
                x1: self.x,
                y1: self.min,
                x2: self.x,
                y2: self.block_bottom,
                color: self.color,
            },
            Line {
                x1: self.x,
                y1: self.block_bottom,
                x2: self.x + 0.05,
                y2: self.block_bottom,
                color: self.color,
            },
            Line {
                x1: self.x,
                y1: self.block_bottom,
                x2: self.x,
                y2: self.block_top,
                color: self.color,
            },
            Line {
                x1: self.x + 0.05,
                y1: self.block_bottom,
                x2: self.x + 0.05,
                y2: self.block_top,
                color: self.color,
            },
            Line {
                x1: self.x,
                y1: self.block_top,
                x2: self.x + 0.05,
                y2: self.block_top,
                color: self.color,
            },
            Line {
                x1: self.x,
                y1: self.block_top,
                x2: self.x,
                y2: self.max,
                color: self.color,
            },
        ];
        for line in &lines {
            line.draw(painter);
        }
    }
}

#[derive(Debug, Clone)]
pub struct KlineData<'a> {
    // "2023-04-07"
    pub date: &'a str,
    // 0.904
    pub pre_close: f64,
    // 0.902
    pub start: f64,
    // 0.9
    pub end: f64,
    // 0.894
    pub min: f64,
    // 0.912
    pub max: f64,
    // 7707861
    pub quantity: i64,
    // 695367487
    pub amount: i64,
    // -0.44
    pub up_rate: f64,
    // 5.52
    pub cr: f64,
    // 0.913
    pub avg5: f64,
    // 0.908
    pub avg10: f64,
    // 0.896
    pub avg20: f64,
    // 0.905
    pub avg30: f64,
    // 7832059.6
    pub avg_q5: f64,
    // 7745399.3
    pub avg_q10: f64,
}

pub struct KlineDatas<'a> {
    pub coords: &'a [KlineData<'a>],
}

impl<'a> Shape for KlineDatas<'a> {
    fn draw(&self, painter: &mut Painter) {
        for (x, k_line_data) in self.coords.iter().enumerate() {
            let mut block_bottom = k_line_data.start;
            let mut block_top = k_line_data.end;
            let mut color = Color::Red;
            if k_line_data.start > k_line_data.end {
                block_bottom = k_line_data.end;
                block_top = k_line_data.start;
                color = Color::Green;
            }
            let kblock = Kblock {
                x: x as f64,
                min: k_line_data.min,
                block_bottom: block_bottom,
                block_top: block_top,
                max: k_line_data.max,
                color: color,
            };
            kblock.draw(painter);
        }
    }
}
