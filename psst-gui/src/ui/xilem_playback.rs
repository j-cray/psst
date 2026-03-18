use xilem::core::Edit;
use xilem::view::{flex_row, flex_col, label, button, FlexSpacer, FlexExt};
use xilem::WidgetView;
use crate::data::AppState;

pub fn playback_bar() -> impl WidgetView<Edit<AppState>> + use<> {
    flex_row((
        // Left side: track info
        flex_col((
            label("Now Playing").flex(0.0),
            label("Artist Name").flex(0.0),
        )).flex(1.0),

        // Center: playback controls
        flex_col((
            flex_row((
                button("Prev", |_| {}).flex(0.0),
                button("Play/Pause", |_| {}).flex(0.0),
                button("Next", |_| {}).flex(0.0),
            )).flex(0.0),
            // progress bar placeholder
            label("----o--------").flex(0.0),
        )).flex(2.0),

        // Right side: Volume and extra controls
        flex_row((
            label("Volume").flex(0.0),
        )).flex(1.0),
    ))
}
