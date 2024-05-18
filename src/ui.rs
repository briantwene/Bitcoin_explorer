use crate::tui::{reduce, Action, AppState, CurrentScreen};
use crate::widgets::{block_table, container_layout, nested_layout, transaction_table};
use crossterm::event::{self, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;

pub fn ui(frame: &mut Frame, app_state: &mut AppState) {
    // variable for focusing on the different screens

    let empty_transactions = vec![];

    match app_state.current_screen {
        CurrentScreen::Home => {
            // define layout

            let container_layout = container_layout(frame.size());
            let nested_layout = nested_layout(container_layout[1]);

            // then render to the frame

            let block_list_component = block_table(&app_state.block_list);
   
            let transaction_table_component = if let Some(current_block) = app_state.current_block.as_ref() {
                transaction_table(&current_block.transactions)
            } else {
                transaction_table(&empty_transactions)
            };

            let widths = [
                Constraint::Length(20),
                Constraint::Length(20),
                Constraint::Length(20),
            ];

            // // transaction list
            // let transaction_table = Table::new(
            //     vec![
            //         Row::new(vec!["one", "two", "three"]),
            //         Row::new(vec!["four", "five", "six"]),
            //         Row::new(vec!["seven", "eight", "nine"]),
            //     ],
            //     widths,
            // )
            // .column_spacing(1)
            // .style(Style::new().blue())
            // .header(
            //     Row::new(vec!["Header 1", "Header 2", "Header 3"])
            //         .style(Style::new().bold())
            //         .bottom_margin(1),
            // )
            // .footer(Row::new(vec!["Updated on Dec 28"]))
            // .block(Block::default().title("Table"))
            // .highlight_style(Style::new().reversed())
            // .highlight_symbol(">>");

            // block info

            frame.render_widget(
                Paragraph::new("outer 0").block(Block::new().borders(Borders::ALL)),
                container_layout[0],
            );
            frame.render_stateful_widget(block_list_component, nested_layout[0], &mut app_state.block_table_state);
            frame.render_stateful_widget(transaction_table_component, nested_layout[1], &mut app_state.transaction_table_state);

            frame.render_widget(
                Paragraph::new("outer 1").block(Block::new().borders(Borders::ALL)),
                container_layout[2],
            );

           
        },

        CurrentScreen::Exiting => {
            let block = Block::default().title("Exiting").borders(Borders::ALL);
            frame.render_widget(block, frame.size());

        }

        CurrentScreen::Exit => {}
    }
}
