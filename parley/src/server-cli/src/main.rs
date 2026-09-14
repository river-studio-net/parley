use server::PublicServer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    PublicServer::build_router()?
        .serve(addr)
        .await?;

    Ok(())
}
