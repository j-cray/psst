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
        
        // Navigation buttons
        button(label("Home"), |state: &mut AppState| state.navigate(&Nav::Home)),
        button(label("Search"), |state: &mut AppState| state.navigate(&Nav::SearchResults("".into()))), // Needs special handling for focus
        button(label("Saved Tracks"), |state: &mut AppState| state.navigate(&Nav::SavedTracks)),
        button(label("Saved Albums"), |state: &mut AppState| state.navigate(&Nav::SavedAlbums)),
        button(label("Podcasts"), |state: &mut AppState| state.navigate(&Nav::Shows)),
    ))
}
