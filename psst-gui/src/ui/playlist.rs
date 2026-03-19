use xilem::{
    view::{flex_col, label},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::AppState;

pub fn playlist_detail_view(_state: &AppState) -> impl WidgetView<Edit<AppState>> {
    flex_col((
        label("Playlist Detail"),
        label("Tracks would appear here"),
    ))
}
