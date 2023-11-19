pub mod ui {
    use std::{
        io::{stdout, Result},
        rc::Rc,
    };

    use crossterm::{
        event::{self, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{
        prelude::{Constraint, CrosstermBackend, Direction, Layout, Rect},
        style::{Color, Style},
        widgets::{Block, Borders, Cell, Row, Table, TableState},
        Frame, Terminal,
    };

    use crate::json::json::JsonEntity;

    pub struct App {
        state: TableState,
        should_quit: bool,
        pub colums: Vec<JsonEntity>,
    }

    impl App {
        pub fn new(colums: Vec<JsonEntity>) -> Self {
            Self {
                state: TableState::default(),
                should_quit: false,
                colums,
            }
        }
    }

    fn ui(app: &mut App, layout: Rc<[Rect]>, f: &mut Frame<'_>) {
        let header_cells: Vec<Cell> = app
            .colums
            .iter()
            .map(|json_entity| json_entity.title.to_uppercase())
            .map(|h| Cell::from(h).style(Style::default().fg(Color::Green)))
            .collect();

        let header = Row::new(header_cells);

        let row_cells: Vec<Cell> = app
            .colums
            .iter()
            .map(|entity| Cell::from(entity.value.to_string()))
            .collect();

        let rows = vec![Row::new(row_cells)];

        let widths: Vec<Constraint> = app
            .colums
            .iter()
            .map(|_| Constraint::Percentage(10))
            .collect();

        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Status table"))
            .highlight_symbol("# ")
            .widths(&widths);
        /*
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Max(10),
                Constraint::Min(10),
            ])
        */

        f.render_stateful_widget(t, layout[0], &mut app.state)

        /*
        for column in &app.colums {
            f.render_widget(
                Paragraph::new(
                    format!("title: {}, val: {}", column.title, column.value.to_string())
                        .light_red()
                        .on_black(),
                )
                .block(
                    Block::default()
                        .title("Counter App")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ),
                layout[0],
            );
        }
        */
    }

    fn update(app: &mut App) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(250))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    pub fn run(mut app: App) -> Result<()> {
        startup()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            terminal.draw(|frame| {
                let laytout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Percentage(100)])
                    //                    .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
                    .split(frame.size());

                ui(&mut app, laytout, frame);
            })?;
            update(&mut app)?;

            if app.should_quit {
                break;
            }
        }
        shutdown()
    }

    fn startup() -> Result<()> {
        enable_raw_mode()?;
        execute!(std::io::stderr(), EnterAlternateScreen)?;
        Ok(())
    }

    fn shutdown() -> Result<()> {
        execute!(std::io::stderr(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}
