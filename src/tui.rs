use crate::{models::FarmState, persistence};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

static NAVIGATION_TEXT: &str = " Move <Up/Down/Left/Right>, Change Tabs: <Tab/Shift+Tab> ";

pub fn run() {
    let _ = ratatui::run(|terminal| App::new().run(terminal));
}

#[derive(PartialEq)]
enum Tabs {
    Farm,
    Inventory,
    Market,
}

pub struct App {
    active_tab: Tabs,
    running: bool,
    farm: FarmState,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: Tabs::Farm,
            running: true,
            farm: persistence::load_farm(),
        }
    }

    fn tab(&mut self, reverse: bool) {
        match self.active_tab {
            Tabs::Farm => match reverse {
                true => self.active_tab = Tabs::Market,
                false => self.active_tab = Tabs::Inventory,
            },
            Tabs::Inventory => match reverse {
                true => self.active_tab = Tabs::Farm,
                false => self.active_tab = Tabs::Market,
            },
            Tabs::Market => match reverse {
                true => self.active_tab = Tabs::Inventory,
                false => self.active_tab = Tabs::Farm,
            },
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.keybinds()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let master_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3)])
            .split(frame.area());

        match self.active_tab {
            Tabs::Farm => {
                frame.render_widget(
                    Paragraph::new("[Farm] | Inventory | Market").block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Thick)
                            .title_top(" termfarm ")
                            .title_bottom(
                                Line::from("".to_string() + &NAVIGATION_TEXT).right_aligned(),
                            ),
                    ),
                    master_layout[0],
                );
            }
            Tabs::Inventory => frame.render_widget(
                Paragraph::new("Farm | [Inventory] | Market").block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                        .title_top(" termfarm ")
                        .title_bottom(
                            Line::from("".to_string() + &NAVIGATION_TEXT).right_aligned(),
                        ),
                ),
                master_layout[0],
            ),
            Tabs::Market => frame.render_widget(
                Paragraph::new("Farm | Inventory | [Market]").block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                        .title_top(" termfarm ")
                        .title_bottom(
                            Line::from("".to_string() + &NAVIGATION_TEXT).right_aligned(),
                        ),
                ),
                master_layout[0],
            ),
        }
    }

    fn keybinds(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => {
                        persistence::save_farm(&self.farm);
                        self.running = false
                    }
                    KeyCode::Tab => self.tab(false),
                    KeyCode::BackTab => self.tab(true),
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
}
