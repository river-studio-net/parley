pub use message_service::{ MessageRequest, MessageRequestResult };
pub use message_service::message_service_server::{ MessageService, MessageServiceServer };
pub use message_service::message_service_client::MessageServiceClient;


pub mod tables {
    tonic::include_proto!("parley.tables");
}


pub mod server_models {
    tonic::include_proto!("parley.server_models");
}


pub mod services {
    tonic::include_proto!("parley.services");

    pub const FILE_DESCRIPTOR_SET: &[u8] = 
        tonic::include_file_descriptor_set!("services_descriptor");
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
