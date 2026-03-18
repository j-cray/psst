use xilem::{
    view::{button, flex_col, label, FlexSpacer},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, Nav};

pub fn sidebar() -> impl WidgetView<Edit<AppState>> {
    flex_col((
        // Top logo
        crate::widget::icons::LOGO.view(xilem::Color::WHITE, 32.0),
        
        // Some filler navigation buttons
        button(label("Home"), |state: &mut AppState| state.nav = Nav::Home),
        button(label("Search"), |state: &mut AppState| state.nav = Nav::SearchResults("".into())),
        button(label("Library"), |state: &mut AppState| state.nav = Nav::SavedTracks),
        
        // Display a Remote Image at the bottom of the sidebar (e.g. current track art)
        // using the remote_image function
        crate::widget::remote_image::remote_image(
            |_state, _loc| (),
            None, // No cached image yet
            Some(std::sync::Arc::from("https://example.com/cover.jpg")),
            label("Loading Image...")
        )
    ))
}
