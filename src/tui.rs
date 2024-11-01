use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct Tui {
    exit: bool,
}

impl Tui {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }


    /// Warning! Example code written by ChatGPT
    fn draw(&self, frame: &mut Frame) {
        // Get the size of the terminal
        let size = frame.area();
        
        // Create a layout with two vertical sections
        let layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .margin(1)
            .constraints([
                ratatui::layout::Constraint::Percentage(20), // Left section
                ratatui::layout::Constraint::Percentage(80), // Right section (graph)
            ])
            .split(size);

        // Draw the left section with a vertical split
        let left_section = layout[0];
        let left_layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Percentage(70), // Top for input fields
                ratatui::layout::Constraint::Percentage(30), // Bottom for info block
            ])
            .split(left_section);

        // Draw the top part of the left section (input fields)
        let input_block = Block::default()
            .title("Input Fields")
            .borders(ratatui::widgets::Borders::ALL);
        frame.render_widget(input_block, left_layout[0]);

        // Draw the bottom part of the left section (info block)
        let info_block = Block::default()
            .title("Info Block")
            .borders(ratatui::widgets::Borders::ALL);
        frame.render_widget(info_block, left_layout[1]);

        // Draw the right section (graph)
        let graph_section = layout[1];
        let graph_block = Block::default()
            .title("Graph")
            .borders(ratatui::widgets::Borders::ALL);
        frame.render_widget(graph_block, graph_section);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Enter => self.new_graph(),
            _ => {}
        }
    }

    /// Renders a new graph overtop the current graph with the current expression
    fn new_graph(&self) {
        todo!()
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
