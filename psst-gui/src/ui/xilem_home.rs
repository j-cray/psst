use xilem::core::Edit;
use xilem::view::{flex_row, flex_col, label, button, FlexSpacer, FlexExt};
use xilem::WidgetView;
use crate::data::AppState;

pub fn home_view() -> impl WidgetView<Edit<AppState>> + use<> {
    flex_col((
        label("Home").flex(0.0),
        // Placeholder for playlists and recent albums
        flex_row((
            button("Recent Album 1", |_| {}).flex(1.0),
            button("Recent Album 2", |_| {}).flex(1.0),
        )).flex(1.0),
    ))
}
