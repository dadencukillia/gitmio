use std::sync::Arc;
use thrussh::*;
use thrussh::server::{Auth, Session};
use thrussh_keys::*;

pub struct SSHServer {
    secret_key: String
}

impl SSHServer {
    pub fn new(secret_key: String) -> Self {
        SSHServer {
            secret_key
        }
    }

    pub async fn run(self, port: i16) -> anyhow::Result<()> {
        let pair = thrussh_keys::decode_secret_key(&self.secret_key, None).unwrap();

        let mut config = thrussh::server::Config::default();

        config.connection_timeout = Some(std::time::Duration::from_secs(3));
        config.auth_rejection_time = std::time::Duration::from_secs(3);
        config.keys.push(pair);
        config.methods = MethodSet::PUBLICKEY;

        let config = Arc::new(config);

        let listening_address = format!("0.0.0.0:{}", port);

        thrussh::server::run(config, &listening_address, self).await?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct SSHConnection { }

impl server::Server for SSHServer {
    type Handler = SSHConnection;

    fn new(&mut self, _: Option<std::net::SocketAddr>) -> SSHConnection {
        SSHConnection { }
    }
}

impl server::Handler for SSHConnection {
    type Error = anyhow::Error;

    async fn auth_publickey(self, _: &str, _: &key::PublicKey) -> Result<(Self, Auth), Self::Error> {
        Ok((self, server::Auth::Accept))
    }

    async fn exec_request(
        self,
        channel: ChannelId,
        data: &[u8],
        mut session: Session,
    ) -> Result<(Self, Session), Self::Error> {
        let command = std::str::from_utf8(data).unwrap_or("");
        println!("{}", command);

        session.data(channel, CryptoVec::from_slice(b"Hey there!"));
        session.eof(channel);
        session.close(channel);

        Ok((self, session))
    }
}
