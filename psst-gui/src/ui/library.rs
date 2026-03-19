use xilem::{
    view::{flex_col, label},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::AppState;

pub fn library_view(_state: &AppState) -> impl WidgetView<Edit<AppState>> {
    flex_col((
        label("Library (Saved Tracks, Albums, Shows)"),
    ))
}
