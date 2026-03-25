use xilem::{
    view::{button, flex_row, label, FlexSpacer},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, PlaybackState};
use crate::ui::utils::image_widget;


pub fn playback_bar(state: &AppState) -> impl WidgetView<Edit<AppState>> {
    let title = if let Some(np) = &state.playback.now_playing {
        np.item.name().to_string()
    } else {
        "Nothing playing".to_string()
    };
    
    let is_playing = state.playback.state == PlaybackState::Playing;
    let is_loading = state.playback.state == PlaybackState::Loading;
    
    let play_label = if is_playing {
        "Pause"
    } else if is_loading {
        "Loading..."
    } else {
        "Play"
    };

    let img_url = state.playback.now_playing.as_ref().and_then(|np| np.cover_image_url(300.0, 300.0)).map(|url| url.into());

    flex_row((
        image_widget(state, img_url),
        label(title),
        FlexSpacer::Flex(1.0),
        button(label(play_label), |s: &mut AppState| {
            if s.playback.state == PlaybackState::Playing {
                s.pause_playback();
                let _ = s.event_sender.send(crate::data::AppEvent::CommandPause);
            } else if s.playback.state == PlaybackState::Paused {
                s.resume_playback();
                let _ = s.event_sender.send(crate::data::AppEvent::CommandResume);
            } else if s.playback.state == PlaybackState::Loading {
                // Ignore clicks while loading
            } else {
                let item = s.playback.now_playing.as_ref().map(|np| np.item.clone());
                if let Some(item) = item {
                    s.resume_playback();
                    let _ = s.event_sender.send(crate::data::AppEvent::CommandPlay(item));
                }
            }
        }),
        button(label("Stop"), |s: &mut AppState| {
            s.stop_playback();
            let _ = s.event_sender.send(crate::data::AppEvent::CommandStop);
        }),
    ))
}
