use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::*,
};



pub enum CurrentScreen {
    Home,
    Exiting,
}

pub struct CurrentBlock {
    pub time: u8,
    pub nonce: u8,
    pub data: u8,
}
pub struct App {
    pub current_screen: CurrentScreen,
    pub current_block: Option<CurrentBlock>,
    counter: u8,
    exit: bool,
}



impl App {


    pub fn new() -> App {
        App {
            counter: 0,
            exit: false,
            current_screen: CurrentScreen::Home,
            current_block: None,
        }
    }
}

