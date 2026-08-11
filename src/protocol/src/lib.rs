pub use message_service::{ MessageRequest, MessageRequestResult };
pub use message_service::message_service_server::{ MessageService, MessageServiceServer };
pub use message_service::message_service_client::MessageServiceClient;


pub mod models {
    tonic::include_proto!("models");
}


pub mod message_service {
    tonic::include_proto!("message_service");

    pub const FILE_DESCRIPTOR_SET: &[u8] = 
        tonic::include_file_descriptor_set!("parley_descriptor");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
