use xilem::masonry::dpi::LogicalSize;
use xilem::view::{flex_col, flex_row, label, FlexSpacer};
use xilem::{EventLoop, WindowOptions, Xilem, WidgetView};
use xilem::core::Edit;
use psst_gui::data::{AppState, Config, nav::Nav};

use psst_gui::ui::{
    home::home_view, playback::playback_bar, sidebar::sidebar, search::search_view,
    library::library_view, playlist::playlist_detail_view, preferences::preferences_view,
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
    if state.config.has_credentials() && state.nav == Nav::Home {
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
    
    let root_layout = if !state.config.has_credentials() {
        psst_gui::ui::login::login_view(state).boxed()
    } else {
        if let Nav::SearchResults(ref query) = state.nav {
            let search_query = psst_gui::data::SearchQuery {
                input: query.clone().into(),
                topic: state.search.topic,
            };
            let needs_fetch = match &state.search.results {
                psst_gui::data::Promise::Empty => true,
                psst_gui::data::Promise::Deferred { def } |
                psst_gui::data::Promise::Resolved { def, .. } |
                psst_gui::data::Promise::Rejected { def, .. } => def != &search_query,
            };
            if needs_fetch && !query.is_empty() {
                state.search.results.defer(search_query.clone());
                let sender = state.event_sender.clone();
                let limit = state.config.paginated_limit;
                std::thread::spawn(move || {
                    let topics = search_query.topic
                        .map(|t| vec![t])
                        .unwrap_or_else(|| psst_gui::data::SearchTopic::all().to_vec());
                    let res = psst_gui::webapi::WebApi::global().search(&search_query.input, &topics, limit);
                    let _ = sender.send(psst_gui::data::AppEvent::SearchResultsLoaded(res));
                });
            }
        }

        let content = match state.nav {
        Nav::Home => home_view(state).boxed(),
        Nav::SearchResults(_) => search_view(state).boxed(),
        Nav::SavedTracks | Nav::SavedAlbums | Nav::Shows => library_view(state).boxed(),
        Nav::PlaylistDetail(_) => playlist_detail_view(state).boxed(),
        Nav::Preferences => preferences_view(state).boxed(),
        _ => label("Unimplemented Route").boxed(),
    };

    let main_content = flex_col((
        topbar(state),
        content,
    ));

    flex_row((
        sidebar(),
        flex_col((
            main_content,
            playback_bar(state),
        )),
    )).boxed()
    };

    xilem::core::fork(
        root_layout,
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
                                log::info!("Backend received Play command for: {:?}", item.name());
                                let playback_item = psst_core::player::item::PlaybackItem {
                                    item_id: item.id(),
                                    norm_level: psst_core::audio::normalize::NormalizationLevel::Track,
                                };
                                if let Some(sender) = &state.player_sender {
                                    let _ = sender.send(psst_core::player::PlayerEvent::Command(
                                        psst_core::player::PlayerCommand::LoadAndPlay { item: playback_item }
                                    ));
                                }
                            }
                            psst_gui::data::AppEvent::CommandPause => {
                                if let Some(sender) = &state.player_sender {
                                    let _ = sender.send(psst_core::player::PlayerEvent::Command(
                                        psst_core::player::PlayerCommand::Pause
                                    ));
                                }
                            }
                            psst_gui::data::AppEvent::CommandResume => {
                                if let Some(sender) = &state.player_sender {
                                    let _ = sender.send(psst_core::player::PlayerEvent::Command(
                                        psst_core::player::PlayerCommand::Resume
                                    ));
                                }
                            }
                            psst_gui::data::AppEvent::CommandStop => {
                                if let Some(sender) = &state.player_sender {
                                    let _ = sender.send(psst_core::player::PlayerEvent::Command(
                                        psst_core::player::PlayerCommand::Stop
                                    ));
                                }
                            }
                            psst_gui::data::AppEvent::SearchResultsLoaded(res) => {
                                let query = if let psst_gui::data::Promise::Deferred { def } = &state.search.results {
                                    Some(def.clone())
                                } else {
                                    None
                                };
                                if let Some(q) = query {
                                    state.search.results.resolve_or_reject(q, res);
                                }
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
                            psst_gui::data::AppEvent::SubmitLogin => {
                                let auth = state.preferences.auth.clone();
                                let sender = state.event_sender.clone();
                                std::thread::spawn(move || {
                                    let config = auth.session_config();
                                    let res = psst_gui::data::config::Authentication::authenticate_and_get_credentials(config);
                                    let _ = sender.send(psst_gui::data::AppEvent::LoginResult(res));
                                });
                            }
                            psst_gui::data::AppEvent::SubmitOAuthLogin(port) => {
                                let sender = state.event_sender.clone();
                                std::thread::spawn(move || {
                                    let (auth_url, pkce_verifier) = psst_core::oauth::generate_auth_url(port);
                                    if let Err(e) = open::that(&auth_url) {
                                        let _ = sender.send(psst_gui::data::AppEvent::LoginResult(Err(format!("Failed to open browser: {}", e))));
                                        return;
                                    }
                                    let bind_addr = std::net::SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), port);
                                    let code = match psst_core::oauth::get_authcode_listener(bind_addr, std::time::Duration::from_secs(300)) {
                                        Ok(c) => c,
                                        Err(e) => {
                                            let _ = sender.send(psst_gui::data::AppEvent::LoginResult(Err(format!("OAuth failed or timed out: {:?}", e))));
                                            return;
                                        }
                                    };
                                    let token = psst_core::oauth::exchange_code_for_token(port, code, pkce_verifier);
                                    let config = psst_core::session::SessionConfig {
                                        login_creds: psst_core::connection::Credentials::from_access_token(token),
                                        proxy_url: psst_gui::data::Config::proxy()
                                    };
                                    let res = psst_gui::data::config::Authentication::authenticate_and_get_credentials(config);
                                    let _ = sender.send(psst_gui::data::AppEvent::LoginResult(res));
                                });
                            }
                            psst_gui::data::AppEvent::LoginResult(res) => {
                                state.preferences.auth.result.resolve_or_reject((), res.clone().map(|_| ()));
                                match res {
                                    Ok(creds) => {
                                        state.config.store_credentials(creds);
                                        let _ = state.config.save();
                                        state.info_alert("Logged in successfully".to_owned());
                                        // Update the webapi global session to ensure subsequent requests work
                                        if let Some(session_config) = state.config.session() {
                                            state.session.update_config(session_config);
                                            let webapi = psst_gui::webapi::WebApi::new(
                                                state.session.clone(),
                                                psst_gui::data::Config::proxy().as_deref(),
                                                psst_gui::data::Config::cache_dir(),
                                                state.config.paginated_limit,
                                            );
                                            webapi.install_as_global();
                                        }
                                    }
                                    Err(err) => {
                                        state.error_alert(format!("Login failed: {}", err));
                                    }
                                }
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
    let mut state = AppState::default_with_config(config.clone());


    let webapi = psst_gui::webapi::WebApi::new(
        state.session.clone(),
        Config::proxy().as_deref(),
        Config::cache_dir(),
        config.paginated_limit,
    );
    webapi.install_as_global();
    
    // Initialize audio backend
    let audio_output = psst_core::audio::output::DefaultAudioOutput::open().expect("Failed to open audio output");
    let cdn = psst_core::cdn::Cdn::new(state.session.clone(), Config::proxy().as_deref()).expect("Failed to create CDN");
    let cache = psst_core::cache::Cache::new(Config::cache_dir().unwrap_or_else(|| std::path::PathBuf::from(".cache"))).expect("Failed to create cache");
    
    let mut player = psst_core::player::Player::new(
        state.session.clone(),
        cdn,
        cache,
        psst_core::player::PlaybackConfig::default(),
        &audio_output,
    );
    let player_sender = player.sender();
    let player_receiver = player.receiver();
    let player_loop_sender = state.event_sender.clone();

    std::thread::spawn(move || {
        // This thread acts as the actor loop for the Player. In this design, 
        // instead of the player yielding outbound events directly to the UI, 
        // the player actor consumes its own internal state events and commands 
        // through this receiver, processing them synchronously.
        
        // Keep audio_output alive for the player's lifetime; dropping it would close the device.
        let _audio_output = audio_output;
        
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for event in player_receiver {
                player.handle(event);
            }
        }));
        
        if let Err(err) = res {
            let msg = if let Some(s) = err.downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = err.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic in player thread".to_string()
            };
            let _ = player_loop_sender.send(psst_gui::data::AppEvent::SessionError(format!("Player thread crashed: {}", msg)));
        }
    });
    
    // Inject the real player_sender
    state.player_sender = Some(player_sender);

    let window_options = WindowOptions::new("Psst Xilem")
        .with_min_inner_size(LogicalSize::new(800.0, 600.0))
        .with_initial_inner_size(LogicalSize::new(1024.0, 768.0));
        
    let app = Xilem::new_simple(state, app_logic, window_options);
    
    app.run_in(EventLoop::with_user_event()).expect("Failed to run Psst application");
}
