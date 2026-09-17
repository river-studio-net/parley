use protocol::{
    services::{
        FILE_DESCRIPTOR_SET, GetCommuneStructureRequest,
        parley_server::{Parley, ParleyServer},
    },
    tables::Commune,
};
use tonic::{
    Request, Response, Status,
    transport::{Server, server::Router},
};
use tonic_reflection::server::Error;

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
impl Parley for ParleyService {
    async fn get_commune_structure(
        &self,
        request: Request<GetCommuneStructureRequest>,
    ) -> Result<Response<Commune>, Status> {
        println!("request: {:?}", request);

        // let input = request.get_ref();
        Err(Status::cancelled("f"))
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
