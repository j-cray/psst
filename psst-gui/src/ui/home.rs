use xilem::{
    view::{flex_col, flex_row, label},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, Promise, MixedView, nav::Nav, PlaylistLink, AlbumLink, ArtistLink, ShowLink};
use crate::ui::utils::image_widget;
use xilem::view::{button, sized_box};
use xilem::style::Style;
use xilem::masonry::layout::Length;

fn render_mixed_view(state: &AppState, title: &str, promise: &Promise<MixedView>) -> impl WidgetView<Edit<AppState>> {
    let content = match promise {
        Promise::Empty => label("...").boxed(),
        Promise::Deferred { .. } => label("Loading...").boxed(),
        Promise::Resolved { val, .. } => {
            let mut row = Vec::new();
            for p in &val.playlists {
                let img_url = p.image(150.0, 150.0).map(|img| img.url.clone());
                let link = PlaylistLink { id: p.id.clone(), name: p.name.clone() };
                row.push(button(
                    flex_col((
                        sized_box(image_widget(state, img_url))
                            .fixed_width(Length::px(150.0))
                            .fixed_height(Length::px(150.0)),
                        label(format!("Playlist: {}", p.name)),
                    )),
                    move |s: &mut AppState| s.navigate(&Nav::PlaylistDetail(link.clone()))
                ).corner_radius(10.0).boxed());
            }
            for a in &val.albums {
                let img_url = a.image(150.0, 150.0).map(|img| img.url.clone());
                let link = AlbumLink { id: a.id.clone(), name: a.name.clone(), images: a.images.clone() };
                row.push(button(
                    flex_col((
                        sized_box(image_widget(state, img_url))
                            .fixed_width(Length::px(150.0))
                            .fixed_height(Length::px(150.0)),
                        label(format!("Album: {}", a.name)),
                    )),
                    move |s: &mut AppState| s.navigate(&Nav::AlbumDetail(link.clone(), None))
                ).corner_radius(10.0).boxed());
            }
            for art in &val.artists {
                let img_url = art.image(150.0, 150.0).map(|img| img.url.clone());
                let link = ArtistLink { id: art.id.clone(), name: art.name.clone() };
                row.push(button(
                    flex_col((
                        sized_box(image_widget(state, img_url))
                            .fixed_width(Length::px(150.0))
                            .fixed_height(Length::px(150.0)),
                        label(format!("Artist: {}", art.name)),
                    )),
                    move |s: &mut AppState| s.navigate(&Nav::ArtistDetail(link.clone()))
                ).corner_radius(10.0).boxed());
            }
            for s in &val.shows {
                let img_url = s.image(150.0, 150.0).map(|img| img.url.clone());
                let link = ShowLink { id: s.id.clone(), name: s.name.clone() };
                row.push(button(
                    flex_col((
                        sized_box(image_widget(state, img_url))
                            .fixed_width(Length::px(150.0))
                            .fixed_height(Length::px(150.0)),
                        label(format!("Show: {}", s.name)),
                    )),
                    move |state: &mut AppState| state.navigate(&Nav::ShowDetail(link.clone()))
                ).corner_radius(10.0).boxed());
            }
            if row.is_empty() {
                label("No items").boxed()
            } else {
                let mut rows = Vec::new();
                let mut current_row = Vec::new();
                for item in row {
                    current_row.push(item);
                    if current_row.len() == 5 { // TODO: use dynamic row count based on window width
                        rows.push(flex_row(current_row).boxed());
                        current_row = Vec::new();
                    }
                }
                if !current_row.is_empty() {
                    rows.push(flex_row(current_row).boxed());
                }
                flex_col(rows).boxed()
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
