use async_channel::Sender;
use shared::message::{
    ClientAuthentication, ClientSetupSecureConnection, Insertion, Message,
};
use shared::query::Query;
use tracing::debug;
use tracing::{error, info};

use crate::command::Command;
use crate::insert;
use crate::query_engine;

pub async fn parse_message(message: Message, tx: Sender<Message>) -> Command {
    match message {
        Message::ClientSetup(param) => parse_client_setup(param),
        Message::ClientAuthentification(param) => parse_authentification(param),
        Message::EndOfCommunication => end_communication(tx).await,
        Message::Insert(param) => insert(param, tx).await,
        Message::Query(param) => handle_query(param, tx).await,
    }
}

fn parse_authentification(authentification: ClientAuthentication) -> Command {
    info!("authentification: {:?}", authentification);
    Command::Continue
}

fn parse_client_setup(secure_connection_message: ClientSetupSecureConnection) -> Command {
    info!("secure message: {:?}", secure_connection_message);
    Command::Continue
}

async fn end_communication(tx: Sender<Message>) -> Command {
    // if let Err(err) = tcp.shutdown().await {
    //     error!("Error while shutdown: {:#?}", err);
    // }
    Command::Exit
}

async fn insert(insertion: Insertion, tx: Sender<Message>) -> Command {
    match insert::insert(insertion).await {
        Ok(uuid) => info!("inserted uuid: {}", uuid),
        Err(err) => debug!("{:?}", err),
    }
    Command::Continue
}

async fn handle_query(query: Query, tx: Sender<Message>) -> Command {
    match query_engine::handle_query(query, tx).await {
        Ok(command) => command,
        Err(err) => {
            error!("{:?}", err);
            Command::Exit
        }
    }
}
