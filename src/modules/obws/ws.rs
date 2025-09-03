use obws::Client;

pub async fn client_connect(ip: &str, port: u16, passwd: &str) -> Client {
    Client::connect(ip, port, Some(passwd)).await.unwrap()
}
