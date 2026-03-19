use xilem::{
    view::{flex_col, label},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::AppState;

pub fn search_view(_state: &AppState) -> impl WidgetView<Edit<AppState>> {
    flex_col((
        label("Search View"),
        label("Search results would appear here..."),
    ))
}
