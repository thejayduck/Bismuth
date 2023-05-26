use args::Arguments;
use clap::Parser;
use directories::BaseDirs;
use inflector::{self, Inflector};
use notify_rust::Notification;
use std::{path::PathBuf, process::Command};
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;

mod args;
mod errors;

const ICON: &str = "image-jpeg";
const NAME: &str = env!("CARGO_PKG_NAME");

struct ImageObject {
    title: String,
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US";

    let args = Arguments::parse();
    let mode = mode(args.mode);

    let client = reqwest::Client::new();
    let response: serde_json::Value = client
        .get(api_url)
        .send()
        .await
        .map_err(|_| errors::Error::Domain(api_url.to_owned()))?
        .json()
        .await
        .map_err(|_| errors::Error::ImageRequest(api_url.to_owned()))?;

    let image_object = response["images"].as_array().unwrap().first().unwrap();
    let image = ImageObject {
        title: image_object["title"].as_str().unwrap().to_string(),
        url: image_object["url"].as_str().unwrap().to_string(),
    };

    let image_url = format!("https://bing.com/{}", image.url);
    let destination = save_image(&image_url).await?;

    let feh_command = Command::new("feh").arg(mode).arg(destination).output()?;

    if feh_command.status.success() && !args.silent {
        send_notification(
            &NAME.to_pascal_case(),
            &format!(
                "Wallpaper successfully set with {0} mode.\nTitle: {1}",
                mode, image.title
            ),
            ICON,
        )?;
    }

    if !feh_command.status.success() {
        return Err(errors::Error::Feh(
            std::string::String::from_utf8(feh_command.stderr).unwrap(),
        )
        .into());
    }

    Ok(())
}

async fn save_image(image_url: &String) -> anyhow::Result<PathBuf> {
    if let Some(base_dirs) = BaseDirs::new() {
        let mut dir = base_dirs.data_local_dir().to_path_buf();

        let file_name = PathBuf::from(".wallpaper.jpg");

        let response = reqwest::get(image_url).await?;
        dir.push(file_name);
        let mut file = tokio::fs::File::create(&dir).await?;

        let content = response.bytes_stream().map(|result| {
            result.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
        });

        tokio::io::copy(&mut StreamReader::new(content), &mut file).await?;

        Ok(dir)
    } else {
        Err(errors::Error::Directory.into())
    }
}

fn send_notification(summary: &str, body: &str, icon: &str) -> anyhow::Result<()> {
    Notification::new()
        .summary(summary)
        .body(body)
        .icon(icon)
        .show()?;

    Ok(())
}

fn mode(value: args::Modes) -> &'static str {
    match value {
        args::Modes::Center => "--bg-center",
        args::Modes::Fill => "--bg-fill",
        args::Modes::Max => "--bg-max",
        args::Modes::Scale => "--bg-scale",
        args::Modes::Tile => "--bg-tile",
    }
}
