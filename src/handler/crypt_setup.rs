use crate::client::Client;
use crate::error::MumbleError;
use crate::handler::Handler;
use crate::proto::mumble::CryptSetup;
use crate::ServerState;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
impl Handler for CryptSetup {
    async fn handle(&self, _state: Arc<RwLock<ServerState>>, client: Arc<RwLock<Client>>) -> Result<(), MumbleError> {
        if self.has_client_nonce() {
            client
                .read()
                .await
                .crypt_state
                .write()
                .await
                .set_decrypt_nonce(self.get_client_nonce());
        } else {
            client.read().await.send_crypt_setup(false).await?;
        }

        Ok(())
    }
}
