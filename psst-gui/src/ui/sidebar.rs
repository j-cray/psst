use xilem::{
    view::{button, flex_col, label},
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
    ))
}
