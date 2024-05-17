use crate::tui::{App, CurrentScreen};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Home => {
            // define layout

            let container_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(60), Constraint::Percentage(10)])
                .split(frame.size());

            // would build widgets here

            let nested_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(container_layout[1]);

            // then render to the frame

            frame.render_widget(
                Paragraph::new("outer 0")
                    .block(Block::new().borders(Borders::ALL)),
                container_layout[0]);
            frame.render_widget(
                Paragraph::new("inner 0")
                    .block(Block::new().borders(Borders::ALL)),
                nested_layout[0]);
            frame.render_widget(
                Paragraph::new("inner 1")
                    .block(Block::new().borders(Borders::ALL)),
                nested_layout[1]);

            frame.render_widget(
                Paragraph::new("outer 1")
                    .block(Block::new().borders(Borders::ALL)),
                container_layout[2]);
        }
        CurrentScreen::Exiting => {
            let block = Block::default().title("Exiting").borders(Borders::ALL);
            frame.render_widget(block, frame.size());
        }
    }
}
