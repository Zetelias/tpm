mod download;
use download::download_and_extract;

#[tokio::main]
async fn main() {
    if let Err(e) = download_and_extract(
        "https://github.com/tokio-rs/tokio/archive/refs/tags/tokio-1.35.1.tar.gz",
        "tokio.tar.gz",
        "."
    )
        .await
    {
        eprintln!("Error: {}", e);
    }
}