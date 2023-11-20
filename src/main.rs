mod args;
mod httpclient;
mod json;
mod tui;
mod ui;

use clap::Parser;
use color_eyre::Result;

use crate::{args::args::TJsonArgs, ui::ui::App};

#[tokio::main]
async fn main() -> Result<()> {
    let args = TJsonArgs::parse();
    let mut app = App::new(args);
    app.run().await?;
    Ok(())
}
/*
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

*/
