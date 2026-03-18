use directories::UserDirs;
use std::fs;
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

    pub fn download_artwork(state: &mut AppState, url: String, title: String) {
        let safe_title = title.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        let file_name = format!("{safe_title} cover.jpg");

        if let Some(user_dirs) = UserDirs::new() {
            if let Some(download_dir) = user_dirs.download_dir() {
                let path = download_dir.join(file_name);

                // In a real Xilem app, this should be spawned as an async task
                // so it doesn't block the UI thread. Doing it synchronously here
                // as a direct port of the old delegate logic for now.
                match ureq::get(&url)
                    .call()
                    .and_then(|response| -> Result<(), ureq::Error> {
                        let mut file = fs::File::create(&path)?;
                        let mut reader = response.into_body().into_reader();
                        std::io::copy(&mut reader, &mut file)?;
                        Ok(())
                    }) {
                    Ok(_) => state.info_alert("Cover saved to Downloads folder."),
                    Err(_) => state.error_alert("Failed to download and save artwork"),
                }
            }
        }
    }
}
