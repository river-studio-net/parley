pub mod tables {
    tonic::include_proto!("parley.tables");
}

pub mod server_models {
    tonic::include_proto!("parley.server_models");
}

pub mod services {
    tonic::include_proto!("parley.services");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("parley_descriptor");
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
