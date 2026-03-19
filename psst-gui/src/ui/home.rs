use xilem::{
    view::{flex_col, flex_row, label, portal},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, Promise, MixedView};
use crate::ui::utils::image_widget;

fn render_mixed_view(state: &AppState, title: &str, promise: &Promise<MixedView>) -> impl WidgetView<Edit<AppState>> {
    let content = match promise {
        Promise::Empty => label("...").boxed(),
        Promise::Deferred { .. } => label("Loading...").boxed(),
        Promise::Resolved { val, .. } => {
            let mut row = Vec::new();
            for p in &val.playlists {
                let img_url = p.image(300.0, 300.0).map(|img| img.url.clone());
                row.push(flex_col((
                    image_widget(state, img_url),
                    label(format!("Playlist: {}", p.name)),
                )).boxed());
            }
            for a in &val.albums {
                let img_url = a.image(300.0, 300.0).map(|img| img.url.clone());
                row.push(flex_col((
                    image_widget(state, img_url),
                    label(format!("Album: {}", a.name)),
                )).boxed());
            }
            for art in &val.artists {
                let img_url = art.image(300.0, 300.0).map(|img| img.url.clone());
                row.push(flex_col((
                    image_widget(state, img_url),
                    label(format!("Artist: {}", art.name)),
                )).boxed());
            }
            for s in &val.shows {
                let img_url = s.image(300.0, 300.0).map(|img| img.url.clone());
                row.push(flex_col((
                    image_widget(state, img_url),
                    label(format!("Show: {}", s.name)),
                )).boxed());
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
        render_mixed_view(state, "Made For You", &state.home_detail.made_for_you),
        render_mixed_view(state, "Your Top Mixes", &state.home_detail.user_top_mixes),
        render_mixed_view(state, "Best of Artists", &state.home_detail.best_of_artists),
        render_mixed_view(state, "Recommended Stations", &state.home_detail.recommended_stations),
    ))
}
