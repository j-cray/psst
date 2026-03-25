use xilem::{
    view::{flex_col, flex_row, label, portal},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, Promise, Nav, AlbumLink, ArtistLink, PlaylistLink, ShowLink};
use crate::ui::utils::image_widget;
use xilem::view::{button, sized_box, text_input};
use xilem::style::Style;
use xilem::masonry::layout::Length;

pub fn search_view(state: &AppState) -> impl WidgetView<Edit<AppState>> {
    let input = text_input(
        state.search.input.to_string(),
        |state: &mut AppState, new_val| {
            state.search.input = new_val.into();
        }
    );

    let search_button = button(label("Search"), |state: &mut AppState| {
        state.navigate(&Nav::SearchResults(state.search.input.clone().into()));
    });

    let results = match &state.search.results {
        Promise::Empty => label("Enter a search query to see results...").boxed(),
        Promise::Deferred { .. } => label("Loading...").boxed(),
        Promise::Rejected { err, .. } => label(format!("Error: {}", err)).boxed(),
        Promise::Resolved { val, .. } => {
            let mut sections = Vec::new();
            
            if !val.artists.is_empty() {
                let mut row = Vec::new();
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
                sections.push(flex_col((label("Artists"), portal(flex_row(row)))).boxed());
            }

            if !val.albums.is_empty() {
                let mut row = Vec::new();
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
                sections.push(flex_col((label("Albums"), portal(flex_row(row)))).boxed());
            }

            if !val.playlists.is_empty() {
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
                sections.push(flex_col((label("Playlists"), portal(flex_row(row)))).boxed());
            }

            if !val.shows.is_empty() {
                let mut row = Vec::new();
                for s in &val.shows {
                    let img_url = s.image(150.0, 150.0).map(|img| img.url.clone());
                    let link = ShowLink { id: s.id.clone(), name: s.name.clone() };
                    row.push(button(
                        flex_col((
                            sized_box(image_widget(state, img_url))
                                .fixed_width(Length::px(150.0))
                                .fixed_height(Length::px(150.0)),
                            label(format!("Podcast: {}", s.name)),
                        )),
                        move |s: &mut AppState| s.navigate(&Nav::ShowDetail(link.clone()))
                    ).corner_radius(10.0).boxed());
                }
                sections.push(flex_col((label("Podcasts"), portal(flex_row(row)))).boxed());
            }

            if !val.tracks.is_empty() {
                let mut row = Vec::new();
                for t in &val.tracks {
                    let img_url = t.album.as_ref().and_then(|a| a.image(150.0, 150.0)).map(|img| img.url.clone());
                    let track = t.clone();
                    row.push(button(
                        flex_col((
                            sized_box(image_widget(state, img_url))
                                .fixed_width(Length::px(150.0))
                                .fixed_height(Length::px(150.0)),
                            label(format!("Track: {}", t.name)),
                        )),
                        move |s: &mut AppState| {
                            let _ = s.event_sender.send(crate::data::AppEvent::CommandPlay(crate::data::Playable::Track(track.clone())));
                        }
                    ).corner_radius(10.0).boxed());
                }
                sections.push(flex_col((label("Tracks"), portal(flex_row(row)))).boxed());
            }

            if sections.is_empty() {
                label("No results found.").boxed()
            } else {
                portal(flex_col(sections)).boxed()
            }
        }
    };

    flex_col((
        flex_row((label("Search: "), input, search_button)),
        results,
    ))
}
