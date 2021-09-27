use jamsocket_wasm::prelude::*;

#[jamsocket_wasm]
struct EchoServer;

impl SimpleJamsocketService for EchoServer {
    fn new(_: &str, _: &impl JamsocketContext) -> Self {
        EchoServer
    }

    fn connect(&mut self, client_id: ClientId, ctx: &impl JamsocketContext) {
        ctx.send_message(client_id, &format!("User {:?} connected.", client_id));
    }

    fn message(&mut self, client_id: ClientId, message: &str, ctx: &impl JamsocketContext) {
        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("User {:?} sent '{}'", client_id, message),
        );
    }

    fn disconnect(&mut self, client_id: ClientId, ctx: &impl JamsocketContext) {
        ctx.send_message(
            MessageRecipient::Broadcast,
            &format!("User {:?} left.", client_id),
        );
    }
}
