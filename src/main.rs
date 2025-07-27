use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    Start,
    Running {
        text: String,
        input: String,
    },
    Done,
    Exit,
}

impl State {
    fn update(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            self.on_key_event(key)
        }
        self.check_complete();
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let title = Line::from("Rat Race").bold().blue().centered();
        let lines = match self {
            State::Start => vec![Line::from("Press 's' to start the test")],
            State::Running { text, input } => {
                vec![Line::from(text.as_str()), Line::from(input.as_str())]
            }
            State::Done => vec![
                Line::from("Done!"),
                Line::from("Press 's' to restart the test"),
            ],
            State::Exit => vec![],
        };
        let text = Text::from(lines);
        let p = Paragraph::new(text);
        frame.render_widget(
            p.block(Block::bordered().title(title)).centered(),
            frame.area(),
        )
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        // Always handle possible quit events.
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }

        // Fall back to individual state handlers.
        match self {
            State::Start | State::Done => {
                if let KeyCode::Char('s') = key.code {
                    *self = Self::Running {
                        text: String::from("The quick brown fox jumped over the lazy dog."),
                        input: String::new(),
                    }
                }
            }
            State::Running { .. } => match key.code {
                KeyCode::Backspace => {
                    self.pop_char();
                }
                KeyCode::Char(ch) => self.append_char(ch),
                _ => {}
            },
            State::Exit => todo!(),
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        *self = State::Exit;
    }

    fn append_char(&mut self, ch: char) {
        match self {
            State::Running { input, .. } => input.push(ch),
            _ => unreachable!(),
        }
    }

    fn pop_char(&mut self) -> Option<char> {
        match self {
            State::Running { input, .. } => input.pop(),
            _ => unreachable!(),
        }
    }

    fn check_complete(&mut self) {
        if let State::Running { text, input } = self {
            if text == input {
                *self = State::Done;
            }
        }
    }
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    state: State,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state != State::Exit {
            terminal.draw(|frame| self.state.draw(frame))?;
            self.state.update(event::read()?)?;
        }
        Ok(())
    }
}
