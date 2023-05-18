use app::App;

#[tokio::main]
async fn main() {
    let client = ifconfig::Client::new();
    let app = App::<ifconfig::Client>::build(client);

    match app.get().await {
        Ok(data) => println!("{}", serde_json::to_string_pretty(&data).unwrap()),
        Err(err) => println!("{}", err),
    }
}
