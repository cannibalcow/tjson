pub mod ui {
    use eyre::eyre;
    use std::time::{Duration, Instant};

    use color_eyre::Result;
    use crossterm::event::{KeyCode, KeyEvent};
    use ratatui::{
        prelude::{Constraint, Direction, Layout},
        style::{Color, Style},
        widgets::{Block, Borders, Cell, Row, Table, TableState},
        Frame,
    };

    use crate::{
        args::args::TJsonArgs,
        httpclient,
        json::json::{get_cell, EntityResult, JsonEntity},
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
        columns: Vec<JsonEntity>,
        state: AppState,
        args: TJsonArgs,
    }

    impl App {
        pub fn new(args: TJsonArgs) -> Self {
            Self {
                args,
                table_state: TableState::default(),
                should_quit: false,
                columns: vec![],
                state: AppState::default(),
                last_updated: Instant::now() - Duration::new(3, 0),
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
                self.update(message).await?;
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

        async fn update(&mut self, message: Message) -> Result<()> {
            match message {
                Message::Quit => self.stop(),
                Message::Tick => self.tick(),
                Message::UpdateSource => {
                    let json = httpclient::fetch(&self.args.source).await?;

                    let result: Vec<JsonEntity> = self
                        .args
                        .pointers
                        .iter()
                        .map(|pointer| get_cell(&json, &pointer))
                        .filter_map(|f| f)
                        .flat_map(|v| -> Vec<JsonEntity> {
                            match v {
                                EntityResult::Entities(cs) => cs,
                                EntityResult::Entity(c) => vec![c],
                            }
                        })
                        .collect();

                    self.columns = result;
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

            let header_cells: Vec<Cell> = self
                .columns
                .iter()
                .map(|json_entity| json_entity.title.to_uppercase())
                .map(|h| Cell::from(h).style(Style::default().fg(Color::Green)))
                .collect();

            let header = Row::new(header_cells);

            let row_cells: Vec<Cell> = self
                .columns
                .iter()
                .map(|entity| Cell::from(entity.value.to_string()))
                .collect();

            let rows = vec![Row::new(row_cells)];

            let widths: Vec<Constraint> = self
                .columns
                .iter()
                .map(|_| Constraint::Percentage(15))
                .collect();

            let t = Table::new(rows)
                .header(header)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Status: {}", &self.args.source)),
                )
                .highlight_symbol("# ")
                .widths(&widths);

            f.render_stateful_widget(t, layout[0], &mut self.table_state);

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
}
