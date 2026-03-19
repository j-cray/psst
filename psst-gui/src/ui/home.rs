use xilem::{
    view::{flex_col, label},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, Promise, MixedView};

fn render_mixed_view(title: &str, promise: &Promise<MixedView>) -> impl WidgetView<Edit<AppState>> {
    let content = match promise {
        Promise::Empty => label("..."),
        Promise::Deferred { .. } => label("Loading..."),
        Promise::Resolved { val, .. } => label(format!("Loaded {} items", val.playlists.len() + val.albums.len() + val.artists.len() + val.shows.len())),
        Promise::Rejected { .. } => label("Failed to load"),
    };
    
    flex_col((
        label(title.to_string()),
        content,
    ))
}

pub fn home_view(state: &AppState) -> impl WidgetView<Edit<AppState>> {
    flex_col((
        label("Home"),
        render_mixed_view("Made For You", &state.home_detail.made_for_you),
        render_mixed_view("Your Top Mixes", &state.home_detail.user_top_mixes),
        render_mixed_view("Best of Artists", &state.home_detail.best_of_artists),
        render_mixed_view("Recommended Stations", &state.home_detail.recommended_stations),
    ))
}
