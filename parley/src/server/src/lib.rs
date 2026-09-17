pub use tonic::{transport::Server, transport::server::Router, Request, Response, Status };
use tonic_reflection::server::Error;
pub use protocol::services::services::{ Parley, ParleyServer };
pub use protocol::services::{ GetCommuneStructureRequest, FILE_DESCRIPTOR_SET };
pub use protocol::tables::{ Commune };


#[derive(Debug, Default)]
pub struct ParleyService {}


impl ParleyService {
    pub fn build_router() -> Result<Router, Error> {
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build_v1()?;
    
        Ok(Server::builder()
            .add_service(reflection_service)
            .add_service(ParleyServer::new(Self::default())))
    }
}

#[tonic::async_trait]
impl ParleyServer for ParleyService {
    fn get_commune_structure(
        &self,
        request: Request<GetCommuneStructureRequest>
    ) -> Result<Response<Commune>, Status> {
        println!("request: {:?}", request);

        let input = request.get_ref();
        Err(Status::Error)
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
