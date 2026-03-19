use xilem::masonry::dpi::LogicalSize;
use xilem::view::{flex_col, flex_row, label, FlexSpacer};
use xilem::{EventLoop, WindowOptions, Xilem, WidgetView};
use xilem::core::Edit;
use psst_gui::data::{AppState, Config, nav::Nav};

use psst_gui::ui::{
    home::home_view, playback::playback_bar, sidebar::sidebar, search::search_view,
    library::library_view, playlist::playlist_detail_view,
};

fn topbar(state: &AppState) -> impl WidgetView<Edit<AppState>> {
    flex_row((
        label("Back"),
        label(format!("Route: {:?}", state.nav)),
        FlexSpacer::Flex(1.0),
        label("Search"),
    ))
}

fn app_logic(state: &mut AppState) -> impl WidgetView<Edit<AppState>> {
    let content = match state.nav {
        Nav::Home => home_view(state).boxed(),
        Nav::SearchResults(_) => search_view(state).boxed(),
        Nav::SavedTracks | Nav::SavedAlbums | Nav::Shows => library_view(state).boxed(),
        Nav::PlaylistDetail(_) => playlist_detail_view(state).boxed(),
        _ => label("Unimplemented Route").boxed(),
    };

    let main_content = flex_col((
        topbar(state),
        content,
    ));

    xilem::core::fork(
        flex_row((
            sidebar(),
            flex_col((
                main_content,
                playback_bar(state),
            )),
        )),
        xilem::view::task_raw(
            |proxy, state: &mut AppState| {
                let receiver = state.event_receiver.clone();
                async move {
                    let _ = xilem::tokio::task::spawn_blocking(move || {
                        while let Ok(event) = receiver.recv() {
                            if proxy.message(event).is_err() {
                                break;
                            }
                        }
                    }).await;
                }
            },
            |state: &mut AppState, event: psst_gui::data::AppEvent| {
                match event {
                    psst_gui::data::AppEvent::ArtworkDownloaded { path, result } => {
                        match result {
                            Ok(()) => state.info_alert(format!("Artwork downloaded to {:?}", path)),
                            Err(e) => state.error_alert(format!("Failed to download artwork: {}", e)),
                        }
                    }
                    psst_gui::data::AppEvent::SessionConnected => {
                        state.info_alert("Spotify session connected successfully".to_owned());
                    }
                    psst_gui::data::AppEvent::SessionError(e) => {
                        state.error_alert(format!("Spotify session error: {}", e));
                    }
                    psst_gui::data::AppEvent::PlaybackStateChanged => {
                        // To be implemented when playback controls are wired up 
                    }
                }
            }
        )
    )
}

fn main() {
    let config = Config::load().unwrap_or_default();
    let state = AppState::default_with_config(config.clone());

    let window_options = WindowOptions::new("Psst Xilem")
        .with_min_inner_size(LogicalSize::new(800.0, 600.0))
        .with_initial_inner_size(LogicalSize::new(1024.0, 768.0));
        
    let app = Xilem::new_simple(state, app_logic, window_options);
    
    app.run_in(EventLoop::with_user_event()).expect("Failed to run Psst application");
}
