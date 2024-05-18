// create the wigets here


use std::rc::Rc;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, widgets::{Block, Borders, List, Row, Table}};

use crate::structures::{BlockData, Transaction};


pub fn container_layout(frame_size: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(60),
            Constraint::Percentage(10)
        ])
        .split(frame_size)
}


pub fn nested_layout(container_layout: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(container_layout)
}

pub fn block_table(block_data: &Vec<BlockData>) -> Table {
    let widths = [
        Constraint::Length(30), // for timestamp
        Constraint::Length(70), // for block_hash
    ];

    let rows: Vec<Row> = block_data.into_iter().map(|block| {
        // Convert each BlockData into a Vec<String>
        let columns = vec![
            block.convert_date(),
            "hassss".to_string()
        ];
        Row::new(columns)
    }).collect();

    let headers = Row::new(vec!["Timestamp", "Block Hash"]);

    Table::new(rows, widths)
        .block(Block::default().title("BLOCKS").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC).bg(Color::Blue))
        .highlight_symbol(">>").header(headers)
}

// TODO: Make it work for transactions instead
pub fn transaction_table(row_data: &Vec<Transaction>) -> Table {
    let widths = [
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
    ];

    let rows: Vec<Row> = row_data.iter().map(|transaction| {
        let total_value: u64 = transaction.outputs.iter().map(|output| output.value).sum();
        Row::new(vec![
            transaction.version.to_string(),
            transaction.inputs.len().to_string(),
            transaction.outputs.len().to_string(),
            total_value.to_string(),
            transaction.locktime.to_string(),
        ])
    }).collect();

    Table::new(rows, widths)
        .header(
            Row::new(vec!["Version", "Inputs", "Outputs", "Total Value", "Locktime"])
                .style(Style::new().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(Block::default().title("Transactions"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
}