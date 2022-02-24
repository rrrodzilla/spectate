use futures_util::StreamExt;
use spectate::{
    spectate_server::{Spectate, SpectateServer},
    LogEntry, SendRecordsReply,
};
use tonic::{transport::Server, Request, Response, Status};
pub(crate) mod spectate {
    tonic::include_proto!("spectate_proto");
}

#[derive(Debug, Default)]
pub struct Spectator {}

#[tonic::async_trait]
impl Spectate for Spectator {
    async fn send_records(
        &self,
        request: Request<tonic::Streaming<LogEntry>>,
    ) -> Result<Response<SendRecordsReply>, Status> {
        let reply = SendRecordsReply::default();
        let mut stream = request.into_inner();

        while let Some(entry) = stream.next().await {
            let data = entry?.log;
            if let Ok(message) = String::from_utf8(data) {
                print!("{}", message);
            }
        }

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let spectator = Spectator::default();
    let svc = SpectateServer::new(spectator);
    println!("Started server");
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
