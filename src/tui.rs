use ratatui::{
    DefaultTerminal,
    Frame,
    widgets::{Paragraph, Block},
    layout::{
        Constraint,
        Direction,
        Layout
    },
    crossterm::event::{
        self,
        Event,
        KeyCode,
        KeyEventKind
    },
};
use crate::models::FarmState;

pub fn run() {
    ratatui::run(|terminal| App::new().run(terminal));
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
    farm:
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: Tabs::Farm,
            running: true,
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
            }
            Tabs::Market => match reverse {
                true => self.active_tab = Tabs::Inventory,
                false => self.active_tab = Tabs::Farm,
            }
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
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
            ])
            .split(frame.area());

        match self.active_tab {
            Tabs::Farm => {
                frame.render_widget(
                    Paragraph::new("[Farm] | Inventory | Market"),
                    layout[0]
                );
            },
            Tabs::Inventory => {
                frame.render_widget(
                    Paragraph::new("Farm | [Inventory] | Market"),
                    layout[0]
                )
            },
            Tabs::Market => {
                frame.render_widget(
                    Paragraph::new("Farm | Inventory | [Market]"),
                    layout[0]
                )
            },
        }
    }

    fn keybinds(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.running = false,
                    KeyCode::Tab => self.tab(false),
                    KeyCode::BackTab => self.tab(true),
                    _ => {},
                }
            }
            _ => {}
        }

        Ok(())
    }
}
