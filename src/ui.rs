use crate::app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use custom_widget::kline::kline::{Axis, Dataset, KLine};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    style::{Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

pub fn run<'a>(app: App<'a>, tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<'a, B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App<'a>,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(i) => app.on_key(i),
                    KeyCode::Left => {}
                    KeyCode::Up => app.on_up(),
                    KeyCode::Right => {}
                    KeyCode::Down => app.on_bottom(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());
    draw_list(f, app, chunks[0]);
    draw_stock_block(f, app, chunks[1]);
}

fn draw_list<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .stock_ids
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(Span::styled(
                i.stock_code,
                Style::default().add_modifier(Modifier::ITALIC),
            ))];
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();
    let widget_items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Stock List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(widget_items, area, &mut app.stock_ids.state);
}

fn draw_stock_block<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(app.selected_item_brief_info().stock_name);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(area);
    draw_stock_header(f, app, chunks[0]);
    draw_k_line_chart(f, app, chunks[1]);
}

fn draw_stock_header<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let text = vec![
        Spans::from(Span::styled(
            "最新: 0.900",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Spans::from(vec![
            Span::styled("今开: 0.902  ", Style::default().fg(Color::Green)),
            Span::styled("最高: 0.912  ", Style::default().fg(Color::Red)),
            Span::from("量: 770.7 万"),
        ]),
        Spans::from(vec![
            Span::from("昨收: 0.904  "),
            Span::styled("最低: 0.894  ", Style::default().fg(Color::Green)),
            Span::from("额: 6.95 亿"),
        ]),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "简要信息",
        Style::default().add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_k_line_chart<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    // let x_labels = vec![
    //     Span::styled(
    //         format!("{}", app.window[0]),
    //         Style::default().add_modifier(Modifier::BOLD),
    //     ),
    //     Span::raw(format!("{}", (app.window[0] + app.window[1]) / 2.0)),
    //     Span::styled(
    //         format!("{}", app.window[1]),
    //         Style::default().add_modifier(Modifier::BOLD),
    //     ),
    // ];
    let datasets = vec![Dataset::default()
        .name("K 线图")
        .marker(symbols::Marker::Braille)
        .data(&app.k_line_datas)];
    let kline = KLine::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Kline",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds(app.get_x_bounds())
                .labels(vec![
                    Span::styled(app.get_start_x_label(), Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(app.get_end_x_label(), Style::default().add_modifier(Modifier::BOLD))
                ])

        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds(app.get_y_bounds())
                .labels(vec![
                    Span::styled(app.get_y_bounds()[0].to_string(), Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(app.get_center_y_label().to_string(), Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(app.get_y_bounds()[1].to_string(), Style::default().add_modifier(Modifier::BOLD))
                ])
        );
    f.render_widget(kline, area);
}
