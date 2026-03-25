use xilem::{
    view::{flex_col, label, portal},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, Promise, nav::Nav};
use crate::ui::utils::image_widget;
use xilem::view::flex_row;

pub fn library_view(state: &AppState) -> impl WidgetView<Edit<AppState>> {
    let content = match state.nav {
        Nav::SavedTracks => {
            match &state.library.saved_tracks {
                Promise::Empty => label("...").boxed(),
                Promise::Deferred { .. } => label("Loading tracks...").boxed(),
                Promise::Resolved { val, .. } => {
                    let mut col = Vec::new();
                    for track in &val.tracks {
                        let img_url = track.album.as_ref().and_then(|a| a.image(300.0, 300.0)).map(|i| i.url.clone());
                        col.push(flex_row((
                            image_widget(state, img_url),
                            label(format!("Track: {} by {}", track.name, track.artists.first().map(|a| a.name.as_ref()).unwrap_or("Unknown")))
                        )).boxed());
                    }
                    if col.is_empty() {
                        label("No saved tracks").boxed()
                    } else {
                        portal(flex_col(col)).boxed()
                    }
                }
                Promise::Rejected { .. } => label("Failed to load tracks").boxed(),
            }
        }
        Nav::SavedAlbums => {
            match &state.library.saved_albums {
                Promise::Empty => label("...").boxed(),
                Promise::Deferred { .. } => label("Loading albums...").boxed(),
                Promise::Resolved { val, .. } => {
                    let mut col = Vec::new();
                    for album in &val.albums {
                        let img_url = album.image(300.0, 300.0).map(|i| i.url.clone());
                        col.push(flex_row((
                            image_widget(state, img_url),
                            label(format!("Album: {} by {}", album.name, album.artists.first().map(|a| a.name.as_ref()).unwrap_or("Unknown")))
                        )).boxed());
                    }
                    if col.is_empty() {
                        label("No saved albums").boxed()
                    } else {
                        portal(flex_col(col)).boxed()
                    }
                }
                Promise::Rejected { .. } => label("Failed to load albums").boxed(),
            }
        }
        _ => label("Library (Shows Unimplemented)").boxed(),
    };

    flex_col((
        label("Library"),
        content,
    ))
}
