use xilem::{
    view::{button, flex_row, label, FlexSpacer},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, PlaybackState};


pub fn playback_bar(state: &AppState) -> impl WidgetView<Edit<AppState>> {
    let title = if let Some(np) = &state.playback.now_playing {
        np.item.name().to_string()
    } else {
        "Nothing playing".to_string()
    };
    
    let is_playing = state.playback.state == PlaybackState::Playing;

    flex_row((
        label(title),
        FlexSpacer::Flex(1.0),
        button(label(if is_playing { "Pause" } else { "Play" }), |s: &mut AppState| {
            if s.playback.state == PlaybackState::Playing {
                s.pause_playback();
            } else if s.playback.state == PlaybackState::Paused {
                s.resume_playback();
            }
        }),
        button(label("Stop"), |s: &mut AppState| {
            s.stop_playback();
        }),
    ))
}
