use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Write;
use tar::Archive;
use futures_util::stream::StreamExt;
use tokio;

pub async fn download_and_extract(
    url: &str,
    path: &str,
    unzip_pack: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let file = File::create(path)?;
        let mut writer = std::io::BufWriter::new(file);
        let total_bytes = response.content_length().unwrap_or(1);
        let mut stream = response.bytes_stream();

        let mut bytes_downloaded = 0;
        while let Some(item) = stream.next().await {
            let chunk = item?;
            writer.write_all(&chunk)?;
            bytes_downloaded += chunk.len() as u64;
            print!(
                "\r[{path}] Downloading {} bytes of {} bytes ({}%)",
                bytes_downloaded,
                total_bytes,
                bytes_downloaded * 100 / total_bytes
            );
        }

        writer.flush()?;
        println!("\n[{path}] File downloaded successfully!");

        let file = File::open(path)?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);

        archive.unpack(unzip_pack)?;
        println!("[{path}] File extracted and renamed successfully!");
    } else {
        println!("[{path}] Failed to fetch the file: {}", response.status());
    }

    // Delete the downloaded file
    std::fs::remove_file(path)?;

    println!("[{path}] Cached tarball deleted successfully!");

    Ok(())
}
