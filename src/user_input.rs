use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;
use tokio::sync::mpsc::Sender;

use crate::query::MetricQuery;

#[derive(Debug)]
pub enum UserInput {
    MetricQuery(MetricQuery),
    Exit,
}

pub fn manage_user_input(user_input_tx: Sender<UserInput>) {
    tokio::spawn(async move {
        let mut query = MetricQuery::empty();
        let mut reader = EventStream::new();

        loop {
            let event = reader.next().await.unwrap().unwrap();

            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Char(char) => {
                        query.name.push(char.clone());
                        user_input_tx
                            .send(UserInput::MetricQuery(query.clone()))
                            .await
                            .unwrap();
                    }
                    KeyCode::Esc => {
                        user_input_tx.send(UserInput::Exit).await.unwrap();
                    }
                    KeyCode::Backspace => {
                        query.name.pop();
                        user_input_tx
                            .send(UserInput::MetricQuery(query.clone()))
                            .await
                            .unwrap();
                    }
                    _ => {}
                }
            }
        }
    });
}
