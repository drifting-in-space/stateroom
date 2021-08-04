use jamsocket::*;
use jamsocket_server::*;

struct EchoServer;

impl SimpleJamsocketService for EchoServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        EchoServer
    }

    fn message(&mut self, user: u32, message: &str, ctx: &impl JamsocketContext) {
        ctx.send_message(user, &format!("echo: {}", message));
    }
}

fn main() -> std::io::Result<()> {
    Server::new().serve_default::<EchoServer>()?;

    Ok(())
}