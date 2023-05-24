use args::Arguments;
use clap::Parser;
use notify_rust::Notification;
use std::process::Command;

mod args;
mod errors;

const ICON: &str = "image-jpeg";
const SUMMARY: &str = "rust-wallpaper";

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
    let image_url = format!("https://bing.com/{}", image_object["url"].as_str().unwrap());

    let feh_command = Command::new("feh").arg(mode).arg(&image_url).output()?;

    if feh_command.status.success() && !args.silent {
        send_notification(
            SUMMARY,
            &format!("Wallpaper successfully set with '{}' mode", mode),
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
        args::Modes::BgCenter => "--bg-center",
        args::Modes::BgFill => "--bg-fill",
        args::Modes::BgMax => "--bg-max",
        args::Modes::BgScale => "--bg-scale",
        args::Modes::BgTile => "--bg-tile",
    }
}
