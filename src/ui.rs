pub mod ui {
    use eyre::eyre;
    use std::{io::stdout, rc::Rc, time::Instant};

    use color_eyre::Result;
    use crossterm::{
        event::{self, KeyCode, KeyEvent, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{
        prelude::{Constraint, CrosstermBackend, Direction, Layout, Rect},
        style::{Color, Style},
        widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
        Frame, Terminal,
    };

    use crate::{
        json::json::JsonEntity,
        tui::{Event, Tui},
    };

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub enum AppState {
        Stopped,
        #[default]
        Running,
        Quitting,
    }

    pub struct App {
        table_state: TableState,
        should_quit: bool,
        update_source_c: usize,
        last_updated: Instant,
        pub columns: Vec<JsonEntity>,
        pub state: AppState,
    }

    impl App {
        pub fn new(columns: Vec<JsonEntity>) -> Self {
            Self {
                table_state: TableState::default(),
                should_quit: false,
                columns,
                state: AppState::default(),
                last_updated: Instant::now(),
                update_source_c: 0,
            }
        }

        pub async fn run(&mut self) -> Result<()> {
            let mut tui = Tui::new()?;
            tui.enter()?;

            while self.state != AppState::Quitting {
                tui.draw(|f| self.ui(f).expect("Error drawing"))?;
                let e = tui.next().await.ok_or(eyre!("Unable to get event"))?; // blocks until next event
                let message = self.handle_event(e)?;
                self.update(message)?;
            }
            tui.exit()?;

            Ok(())
        }

        fn handle_event(&self, event: Event) -> Result<Message> {
            let now = Instant::now();
            let elapsed = (now - self.last_updated).as_secs_f64();
            if elapsed >= 2.0 {
                return Ok(Message::UpdateSource);
            }

            let msg = match event {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => Message::Quit,
                    _ => Message::Tick,
                },
                _ => Message::Tick,
            };

            Ok(msg)
        }

        fn update(&mut self, message: Message) -> Result<()> {
            match message {
                Message::Quit => self.stop(),
                Message::Tick => self.tick(),
                Message::UpdateSource => {
                    self.last_updated = Instant::now();
                    self.update_source_c += 1;
                }
            }
            Ok(())
        }

        fn tick(&mut self) {
            // Do nothing
        }

        fn stop(&mut self) {
            self.state = AppState::Quitting
        }

        fn start(&mut self) {
            self.state = AppState::Running;
        }

        fn ui(&mut self, f: &mut Frame) -> Result<()> {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(100)])
                //                    .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(f.size());

            f.render_widget(
                Paragraph::new(format!("Update source: {}", self.update_source_c)),
                layout[0],
            );
            Ok(())
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum AppEvent {
        Error,
        Tick,
        Key(KeyEvent),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Message {
        UpdateSource,
        Quit,
        Tick,
    }

    fn ui(app: &mut App, layout: Rc<[Rect]>, f: &mut Frame<'_>) {
        let header_cells: Vec<Cell> = app
            .columns
            .iter()
            .map(|json_entity| json_entity.title.to_uppercase())
            .map(|h| Cell::from(h).style(Style::default().fg(Color::Green)))
            .collect();

        let header = Row::new(header_cells);

        let row_cells: Vec<Cell> = app
            .columns
            .iter()
            .map(|entity| Cell::from(entity.value.to_string()))
            .collect();

        let rows = vec![Row::new(row_cells)];

        let widths: Vec<Constraint> = app
            .columns
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

        f.render_stateful_widget(t, layout[0], &mut app.table_state)

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
        println!("hej");
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
