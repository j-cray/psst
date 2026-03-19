use xilem::{
    view::{flex_col, flex_row, label, portal},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, Promise, MixedView};

fn render_mixed_view(title: &str, promise: &Promise<MixedView>) -> impl WidgetView<Edit<AppState>> {
    let content = match promise {
        Promise::Empty => label("...").boxed(),
        Promise::Deferred { .. } => label("Loading...").boxed(),
        Promise::Resolved { val, .. } => {
            let mut row = Vec::new();
            for p in &val.playlists {
                row.push(label(format!("Playlist: {}", p.name)).boxed());
            }
            for a in &val.albums {
                row.push(label(format!("Album: {}", a.name)).boxed());
            }
            for art in &val.artists {
                row.push(label(format!("Artist: {}", art.name)).boxed());
            }
            for s in &val.shows {
                row.push(label(format!("Show: {}", s.name)).boxed());
            }
            if row.is_empty() {
                label("No items").boxed()
            } else {
                portal(flex_row(row)).boxed()
            }
        }
        Promise::Rejected { .. } => label("Failed to load").boxed(),
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
