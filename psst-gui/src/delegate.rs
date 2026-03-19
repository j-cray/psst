use directories::UserDirs;
use std::fs;
use std::thread;
use crate::data::AppState;

pub struct Delegate;

impl Delegate {
    pub fn copy_to_clipboard(_state: &mut AppState, _text: &str) {
        #[cfg(feature = "clipboard")]
        {
            // Note: In Xilem, clipboard access might need a specific integration
            // or the winit/masonry clipboard API
            // For now, this is a placeholder if copypasta or similar is used.
        }
    }

    pub fn open_url(_state: &mut AppState, url: &str) {
        let _ = open::that(url);
    }

    pub fn download_artwork(_state: &mut AppState, url: String, title: String) {
        let safe_title = title.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        let file_name = format!("{safe_title} cover.jpg");

        if let Some(user_dirs) = UserDirs::new() {
            if let Some(download_dir) = user_dirs.download_dir() {
                let path = download_dir.join(file_name);

                // Spawn a background thread so the download doesn't block the UI thread.
                // TODO: wire the result back to AppState via a Xilem event channel once
                // the event-loop integration is in place.
                thread::spawn(move || {
                    match ureq::get(&url)
                        .call()
                        .and_then(|response| -> Result<(), ureq::Error> {
                            let mut file = fs::File::create(&path)?;
                            let mut reader = response.into_body().into_reader();
                            std::io::copy(&mut reader, &mut file)?;
                            Ok(())
                        }) {
                        Ok(_) => log::info!("Cover art saved to Downloads folder."),
                        Err(e) => log::error!("Failed to download artwork: {e}"),
                    }
                });
            }
        }
    }
}
