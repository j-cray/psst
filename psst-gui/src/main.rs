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
    if state.nav == Nav::Home {
        if state.home_detail.made_for_you.state() == psst_gui::data::PromiseState::Empty {
            state.home_detail.made_for_you.defer_default();
            let sender = state.event_sender.clone();
            std::thread::spawn(move || {
                let res = psst_gui::webapi::WebApi::global().get_made_for_you();
                let _ = sender.send(psst_gui::data::AppEvent::MadeForYouLoaded(res));
            });
        }
        if state.home_detail.user_top_mixes.state() == psst_gui::data::PromiseState::Empty {
            state.home_detail.user_top_mixes.defer_default();
            let sender = state.event_sender.clone();
            std::thread::spawn(move || {
                let res = psst_gui::webapi::WebApi::global().get_top_mixes();
                let _ = sender.send(psst_gui::data::AppEvent::TopMixesLoaded(res));
            });
        }
        if state.home_detail.best_of_artists.state() == psst_gui::data::PromiseState::Empty {
            state.home_detail.best_of_artists.defer_default();
            let sender = state.event_sender.clone();
            std::thread::spawn(move || {
                let res = psst_gui::webapi::WebApi::global().best_of_artists();
                let _ = sender.send(psst_gui::data::AppEvent::BestOfArtistsLoaded(res));
            });
        }
        if state.home_detail.recommended_stations.state() == psst_gui::data::PromiseState::Empty {
            state.home_detail.recommended_stations.defer_default();
            let sender = state.event_sender.clone();
            std::thread::spawn(move || {
                let res = psst_gui::webapi::WebApi::global().recommended_stations();
                let _ = sender.send(psst_gui::data::AppEvent::RecommendedStationsLoaded(res));
            });
        }
        
        // TODO: The following HomeDetail fields are declared but not currently fetched:
        // your_shows, shows_that_you_might_like, uniquely_yours, jump_back_in, user_top_tracks, user_top_artists.
        // They remain permanently empty. We should add fetch logic and AppEvent variants for them.
    }
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
        xilem::core::memoize(
            (),
            |()| {
                xilem::view::task_raw(
                    |proxy, state: &mut AppState| {
                        let receiver = state.take_event_receiver().expect("event_receiver already taken");
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
                            psst_gui::data::AppEvent::CommandPlay(item) => {
                                log::info!("Backend received Play command for: {}", item.name());
                                state.info_alert(format!("Playing: {}", item.name()));
                                // player_sender.send(PlayerCommand::LoadAndPlay { item })
                            }
                            psst_gui::data::AppEvent::CommandPause => {
                                log::info!("Backend received Pause command");
                                state.info_alert("Playback paused".to_string());
                            }
                            psst_gui::data::AppEvent::CommandResume => {
                                log::info!("Backend received Resume command");
                                state.info_alert("Playback resumed".to_string());
                            }
                            psst_gui::data::AppEvent::CommandStop => {
                                log::info!("Backend received Stop command");
                                state.info_alert("Playback stopped".to_string());
                            }
                            psst_gui::data::AppEvent::MadeForYouLoaded(res) => {
                                state.home_detail.made_for_you.resolve_or_reject((), res);
                            }
                            psst_gui::data::AppEvent::TopMixesLoaded(res) => {
                                state.home_detail.user_top_mixes.resolve_or_reject((), res);
                            }
                            psst_gui::data::AppEvent::BestOfArtistsLoaded(res) => {
                                state.home_detail.best_of_artists.resolve_or_reject((), res);
                            }
                            psst_gui::data::AppEvent::RecommendedStationsLoaded(res) => {
                                state.home_detail.recommended_stations.resolve_or_reject((), res);
                            }
                        }
                    }
                )
            }
        )
    )
}

fn main() {
    let config = Config::load().unwrap_or_default();
    let state = AppState::default_with_config(config.clone());

    // TODO: Add an authentication flow / session controller interface.
    // The current Xilem implementation is missing a login/auth UI, so if
    // no cached credentials exist, network requests will fail silently.
    let webapi = psst_gui::webapi::WebApi::new(
        state.session.clone(),
        Config::proxy().as_deref(),
        Config::cache_dir(),
        config.paginated_limit,
    );
    webapi.install_as_global();

    let window_options = WindowOptions::new("Psst Xilem")
        .with_min_inner_size(LogicalSize::new(800.0, 600.0))
        .with_initial_inner_size(LogicalSize::new(1024.0, 768.0));
        
    let app = Xilem::new_simple(state, app_logic, window_options);
    
    app.run_in(EventLoop::with_user_event()).expect("Failed to run Psst application");
}
