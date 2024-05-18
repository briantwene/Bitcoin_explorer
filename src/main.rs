use connection::Connection;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use structures::BlockData;
use tui::{reduce, Action, AppState, CurrentScreen, Direction, TableType};
mod connection;
mod serialisers;
mod structures;
mod tui;
mod ui;
mod utils;
mod widgets;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::{io, thread};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;
use ui::ui;

fn main() -> io::Result<()> {

    let (sender, reciever) = mpsc::channel();
    let mut connection = Connection::new();
    let _ = connection.connect();
    let _ = connection.handshake();

    let _ = connection.handle_stream(sender);
    // let handle = thread::spawn(move || {
       
    // });
   

    // enable_raw_mode()?;
    // let mut stdout = io::stdout(); // This is a special case. Normally using stdout is fine
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;

    // // create app and run it
    // let mut app_state = AppState::new();
    // let res = run_app(&mut terminal, &mut app_state, reciever);

    // handle.join().unwrap();

    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;


    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app_state: &mut AppState, receiver: Receiver<BlockData>) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app_state))?;

        if let Ok(block) = receiver.try_recv() {
            // Dispatch an action to add the block to the block list
            let action = Action::AddBlock(block);
            reduce(app_state, action);
        }

        // Key event handling
        match app_state.current_screen {
            CurrentScreen::Home => {
                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let action = match key.code {
                            KeyCode::Down | KeyCode::Up => {
                                let direction = if key.code == KeyCode::Down {
                                    Direction::Down
                                } else {
                                    Direction::Up
                                };
                                match app_state.active_table {
                                    TableType::Block => Action::NavigateTable(direction, TableType::Block),
                                    TableType::Transaction => Action::NavigateTable(direction, TableType::Transaction),
                                }
                            }
                            KeyCode::Tab => Action::SwitchTable,
                            KeyCode::BackTab => {
                                // Switch to the previous panel
                                let prev_panel = (app_state.selected_panel + 1) % 2; // Assuming you have 2 panels
                                Action::SetPanel(prev_panel)
                            }
                            KeyCode::Char('q') => Action::SetScreen(CurrentScreen::Exiting),
                            _ => Action::DoNothing,
                        };
        
                        reduce(app_state, action);
                    }
                }
        

            }
            CurrentScreen::Exit => {

                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let action = match key.code {
                            KeyCode::Char('y') => Action::SetScreen(CurrentScreen::Exit),
                            KeyCode::Char('n') => Action::SetScreen(CurrentScreen::Home),
                            _ => Action::DoNothing,
                        };
    
                        reduce(app_state, action);
                    }
                }
            }
            CurrentScreen::Exiting => break,
            _ => {}
        }



    }

    Ok(true)
}
