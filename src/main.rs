mod account;
mod character;
mod login_server;
mod network_message;
mod server;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let addr = [127, 0, 0, 1];
    let port = 7171;

    login_server::listen(addr, port).await?;

    Ok(())
}
