use xilem::core::Edit;
use xilem::view::{flex_row, flex_col, label, button, FlexSpacer, FlexExt};
use xilem::WidgetView;
use xilem::masonry::layout::AsUnit;
use crate::data::AppState;

pub fn sidebar() -> impl WidgetView<Edit<AppState>> + use<> {
    flex_col((
        label("Menu").flex(0.0),
        button("Home", |_| {}).flex(0.0),
        button("Search", |_| {}).flex(0.0),
        
        FlexSpacer::Fixed(20.px()),
        
        label("Your Library").flex(0.0),
        button("Saved Tracks", |_| {}).flex(0.0),
        button("Saved Albums", |_| {}).flex(0.0),

        FlexSpacer::Fixed(20.px()),

        label("Playlists").flex(0.0),
        // Placeholder for playlist list
        label("Playlist 1").flex(0.0),
        label("Playlist 2").flex(0.0),
        
        FlexSpacer::Flex(1.0),
        
        label("Controls").flex(0.0),
    ))
}
