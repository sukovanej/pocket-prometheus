use anyhow::{Context, Error};
use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;
use tokio::sync::mpsc::Sender;

use crate::query::MetricQuery;

#[derive(Debug)]
pub enum UserInput {
    MetricQuery(MetricQuery),
    Exit,
    ScrollUp,
    ScrollDown,
    ScrollPageUp,
    ScrollPageDown,
}

pub fn manage_user_input(user_input_tx: Sender<UserInput>) {
    tokio::spawn(async move {
        let mut query = MetricQuery::default();
        let mut reader = EventStream::new();

        loop {
            let event = reader.next().await.context("")??;

            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Char(char) => {
                        query.name.push(char);
                        user_input_tx
                            .send(UserInput::MetricQuery(query.clone()))
                            .await?;
                    }
                    KeyCode::Esc => {
                        user_input_tx.send(UserInput::Exit).await?;
                        break;
                    }
                    KeyCode::Up => {
                        user_input_tx.send(UserInput::ScrollUp).await?;
                    }
                    KeyCode::Down => {
                        user_input_tx.send(UserInput::ScrollDown).await?;
                    }
                    KeyCode::PageUp => {
                        user_input_tx.send(UserInput::ScrollPageUp).await?;
                    }
                    KeyCode::PageDown => {
                        user_input_tx.send(UserInput::ScrollPageDown).await?;
                    }
                    KeyCode::Backspace => {
                        query.name.pop();
                        user_input_tx
                            .send(UserInput::MetricQuery(query.clone()))
                            .await?;
                    }
                    _ => {}
                }
            }
        }

        Ok::<(), Error>(())
    });
}
