use app::App;

#[tokio::main]
async fn main() {
    let app = App::new();
    match app.get().await {
        Ok(data) => println!("{}", serde_json::to_string_pretty(&data).unwrap()),
        Err(err) => println!("{}", err),
    }
}
