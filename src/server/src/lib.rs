pub use tonic::{transport::Server, transport::server::Router, Request, Response, Status };
use tonic_reflection::server::Error;
pub use protocol::message_service::message_service_server::{ MessageService, MessageServiceServer };
pub use protocol::message_service::{ MessageRequest, MessageRequestResult, FILE_DESCRIPTOR_SET };


#[derive(Debug, Default)]
pub struct PublicServer {}


impl PublicServer {
    pub fn build_router() -> Result<Router, Error> {
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build_v1()?;
    
        Ok(Server::builder()
            .add_service(reflection_service)
            .add_service(MessageServiceServer::new(Self::default())))
    }
}

#[tonic::async_trait]
impl MessageService for PublicServer {
    async fn send_message(&self, request: Request<MessageRequest>) -> Result<Response<MessageRequestResult>, Status> {
        println!("Got a request: {:?}", request);

        let input = request.get_ref();

        Ok(Response::new(MessageRequestResult{status: true, text: input.ciphertext.clone()}))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
