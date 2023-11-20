pub mod args;
pub mod json;
pub mod tui;
pub mod ui;

use color_eyre::Result;
use json::json::{get_cell, EntityResult};
use serde_json::Value;
use std::io::{stdin, Read};

use clap::Parser;

use crate::{args::args::TJsonArgs, json::json::JsonEntity, ui::ui::App};

#[tokio::main]
async fn main() -> Result<()> {
    let args = TJsonArgs::parse();

    let mut std = stdin().lock();
    let mut json_str = String::new();
    std.read_to_string(&mut json_str)?;

    let jo: Value = serde_json::from_str(&json_str).unwrap();

    let result: Vec<JsonEntity> = args
        .pointers
        .iter()
        .map(|pointer| get_cell(&jo, &pointer))
        .filter_map(|f| f)
        .flat_map(|v| -> Vec<json::json::JsonEntity> {
            match v {
                EntityResult::Entities(cs) => cs,
                EntityResult::Entity(c) => vec![c],
            }
        })
        .collect();

    let mut app = App::new(result);

    app.run().await?;

    Ok(())
}
