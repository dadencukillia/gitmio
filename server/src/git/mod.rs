mod server;
mod services;
mod lib;

const SECRET_KEY: &str = "-----BEGIN OPENSSH PRIVATE KEY-----\nb3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW\nQyNTUxOQAAACBFImB8CMP08Y/yBGNrF33sDip2XRcf8dUd2ELX07evFQAAAJhxfMzhcXzM\n4QAAAAtzc2gtZWQyNTUxOQAAACBFImB8CMP08Y/yBGNrF33sDip2XRcf8dUd2ELX07evFQ\nAAAEC7oHsjkmA6xa0ceaqSXEBlxkir7ybogjmVTF6xnJDWUkUiYHwIw/Txj/IEY2sXfewO\nKnZdFx/x1R3YQtfTt68VAAAAEWNyb2NvYnlAYXJjaGxpbnV4AQIDBA==\n-----END OPENSSH PRIVATE KEY-----\n";

pub async fn run_git_server(port: i16) -> anyhow::Result<()> {
    server::SSHServer::new(
        SECRET_KEY.to_string()
    ).run(port).await
}
