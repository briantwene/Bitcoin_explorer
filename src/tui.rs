use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::*,
};
use ratatui::widgets::{Table, TableState};

use crate::structures::{BlockData, Transaction};



pub enum CurrentScreen {
    Home,
    Exiting,
    Exit
}

pub struct CurrentBlock {
    pub time: u8,
    pub nonce: u8,
    pub data: u8,

}

pub enum Direction {
    Up,
    Down
}

pub enum TableType {
    Block,
    Transaction,
}

pub enum Action {
    SetScreen(CurrentScreen),
    SelectBlock(usize),
    SetPanel(usize),
    AddBlock(BlockData),
    SetExit(bool),
    NavigateTable(Direction, TableType),
    SwitchTable,
    DoNothing,
    // other actions...
}

pub struct AppState {
    pub current_screen: CurrentScreen,
    pub current_block: Option<BlockData>,
    pub block_list: Vec<BlockData>,
    pub test_list: [&'static str; 3],
    pub selected_panel: usize,
    pub selected_block: usize,  
    pub transaction_list: Vec<Transaction>, // Assuming you have a transaction_list in your state
    pub counter: u8,
    pub exit: bool,
    pub block_table_state: TableState,
    pub transaction_table_state: TableState,
    pub active_table: TableType // Add this line
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            counter: 0,
            exit: false,
            current_screen: CurrentScreen::Home,
            current_block: None,
            test_list: ["one", "two", "three"],
            block_list: Vec::new(),
            selected_block: 0,
            selected_panel: 0,
            transaction_list: Vec::new(),
            block_table_state: TableState::default(), 
            transaction_table_state: TableState::default(),
            active_table: TableType::Block// Add this line
        }
    }
}


pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::SetScreen(screen) => state.current_screen = screen,
        Action::AddBlock(block) => {
        state.block_list.push(block);
        // Update the block_table_state
        state.block_table_state.select(Some(state.block_list.len() - 1));
    }
        Action::SetPanel(index) => {
            state.selected_panel = index;
        }
        Action::SelectBlock(_) => {
            if let Some(selected) = state.block_table_state.selected() {
                if selected < state.block_list.len() {
                    state.current_block = Some(state.block_list[selected].clone());
                    state.transaction_list = state.current_block.as_ref().unwrap().transactions.clone();
                    state.transaction_table_state.select(Some(0)); // Reset the selection in the transaction table
                }
            }
        },
        Action::NavigateTable(direction, table_type) => {
            let (len, current_selection) = match table_type {
                TableType::Block => (state.block_list.len(), state.block_table_state.selected().unwrap_or(0)),
                TableType::Transaction => (state.transaction_list.len(), state.transaction_table_state.selected().unwrap_or(0)), // Assuming you have a transaction_list in your state
            };

            let new_selection = match direction {
                Direction::Up => {
                    if current_selection > 0 { current_selection - 1 } else { len - 1 }
                }
                Direction::Down => {
                    if current_selection < len - 1 { current_selection + 1 } else { 0 }
                }
            };

            match table_type {
                TableType::Block => state.block_table_state.select(Some(new_selection)),
                TableType::Transaction => state.transaction_table_state.select(Some(new_selection)),
            };
        }
        Action::SwitchTable => {
            state.active_table = match state.active_table {
                TableType::Block => TableType::Transaction,
                TableType::Transaction => TableType::Block,
            };
        }
        Action::SetExit(value) => {
            state.exit = value;
        }
        Action::DoNothing => {}

        // other actions...
    }
}