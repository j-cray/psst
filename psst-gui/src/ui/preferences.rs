use xilem::{
    view::{button, flex_col, flex_row, label},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, config::{AudioQuality, Theme}};

pub fn preferences_view(state: &AppState) -> impl WidgetView<Edit<AppState>> {
    let theme_label = match state.config.theme {
        Theme::Light => "Light",
        Theme::Dark => "Dark",
    };
    
    let audio_label = match state.config.audio_quality {
        AudioQuality::Low => "Low (96kbps)",
        AudioQuality::Normal => "Normal (160kbps)",
        AudioQuality::High => "High (320kbps)",
    };

    flex_col((
        label("Preferences"),
        
        label("Theme"),
        flex_row((
            button(label("Light"), |state: &mut AppState| {
                state.config.theme = Theme::Light;
                let _ = state.config.save();
            }),
            button(label("Dark"), |state: &mut AppState| {
                state.config.theme = Theme::Dark;
                let _ = state.config.save();
            }),
            label(format!("Current: {}", theme_label)),
        )),
        
        label("Audio Quality"),
        flex_row((
            button(label("Low"), |state: &mut AppState| {
                state.config.audio_quality = AudioQuality::Low;
                let _ = state.config.save();
            }),
            button(label("Normal"), |state: &mut AppState| {
                state.config.audio_quality = AudioQuality::Normal;
                let _ = state.config.save();
            }),
            button(label("High"), |state: &mut AppState| {
                state.config.audio_quality = AudioQuality::High;
                let _ = state.config.save();
            }),
            label(format!("Current: {}", audio_label)),
        )),
    ))
}
