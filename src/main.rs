use std::{io, sync::mpsc, thread, time::Duration};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Gauge, Widget},
};

fn main() -> io::Result<()> {
    // Initialize ratatui by setting the terminal in raw mode
    let mut terminal = ratatui::init();

    // Create an instance of our app
    let mut app = App {
        exit_flag: false,
        progess_bar_colour: Color::Green,
        background_progress: 0_f64,
    };

    // Create the transmitter and receiver for our application
    let (custom_event_tx, custom_event_rx) = mpsc::channel::<CustomEvents>();

    // Use a cloned tx as our main thread to receive the key events from the user
    let tx_to_input_events = custom_event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    // Use the original tx to delegate the tasks of background process on a new thread
    let tx_to_background_process = custom_event_tx;
    thread::spawn(move || {
        handle_background_process(tx_to_background_process);
    });

    // Run the app instance
    let app_result = app.run(&mut terminal, custom_event_rx);

    // Restore the terminal state by exiting raw mode
    ratatui::restore();

    // Return the result
    app_result
}

// ENUM to define all the events supported by our application
enum CustomEvents {
    Input(crossterm::event::KeyEvent),
    Progress(f64),
}

/// Functionality offered by input handling thread
///
/// Args:
/// - We wil require some form of transmitter to send the events to our main thread
fn handle_input_events(tx: mpsc::Sender<CustomEvents>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => {
                tx.send(CustomEvents::Input(key_event)).unwrap()
            }
            _ => {}
        }
    }
}

fn handle_background_process(tx: mpsc::Sender<CustomEvents>) {
    let mut progress = 0_f64;
    let increment = 0.01_f64;

    loop {
        thread::sleep(Duration::from_millis(100));
        progress += increment;
        progress = progress.min(1_f64);
        tx.send(CustomEvents::Progress(progress)).unwrap();
    }
}

struct App {
    exit_flag: bool,
    progess_bar_colour: Color,
    background_progress: f64,
}

impl App {
    /// Function responsible for running an app instance
    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rx: mpsc::Receiver<CustomEvents>,
    ) -> io::Result<()> {
        // Endless loop that re-renders until we receive the exit flag
        while !self.exit_flag {
            // We need to read the keys clicked by the user to handle the exit from our application
            match rx.recv().unwrap() {
                CustomEvents::Input(key_event) => self.handle_key_event(key_event)?,
                CustomEvents::Progress(cur_progress) => self.background_progress = cur_progress,
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
        } else if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('c') {
            if self.progess_bar_colour == Color::Green {
                self.progess_bar_colour = Color::LightYellow
            } else {
                self.progess_bar_colour = Color::Green
            }
        }

        Ok(())
    }
}

/// We are using a reference to the App struct to ensure we are not changing it's state while rendering our widgets as guaranteed in our custom draw function.
impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vertical_layout =
            Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [top_area, gauge_area] = vertical_layout.areas(area);

        // Render a title on top of the layout
        Line::from("Process Overview").bold().centered().render(top_area, buf);

        // Render instruction texts
        let instructions = Line::from(vec![
            // Converts this string literal into span which is the most elementary widget for text representation
            " Change colour ".into(),
            "<C>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ])
        .centered();

        // Add a frame around the progress bar
        let block = Block::bordered()
            .title(Line::from(" Background Processes "))
            .title_bottom(instructions)
            .border_set(border::THICK);

        // Render a progress bar
        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progess_bar_colour))
            .block(block)
            .label(format!(
                "Process 1: {:.2}%",
                self.background_progress * 100_f64
            ))
            .ratio(self.background_progress);

        progress_bar.render(
            Rect {
                x: gauge_area.left(),
                y: gauge_area.top(),
                width: gauge_area.width,
                height: 3,
            },
            buf,
        );
    }
}
