use std::io;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, style::Stylize, text::Line, widgets::Widget};

fn main() -> io::Result<()> {
    // Initialize ratatui by setting the terminal in raw mode
    let mut terminal = ratatui::init();

    // Create an instance of our app
    let mut app = App{ exit_flag: false };

    // Run the app instance
    let app_result = app.run(&mut terminal);

    // Restore the terminal state by exiting raw mode
    ratatui::restore();

    // Return the result
    app_result
}

struct App {
    exit_flag: bool
}

impl App {
    /// Function responsible for running an app instance
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // Endless loop that re-renders until we receive the exit flag
        while !self.exit_flag {
            // We need to read the keys clicked by the user to handle the exit from our application
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }

            terminal.draw(|frame| self.draw(frame))?;
        }
        
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit_flag = true;
        }

        Ok(())
    }
}

/// We are using a reference to the App struct to ensure we are not changing it's state while rendering our widgets as guaranteed in our custom draw function.
impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        
        // Render a title on top of the layout
        Line::from("Hello, World!").bold().render(area, buf);
    }
}