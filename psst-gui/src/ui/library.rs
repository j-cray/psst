use xilem::view::label;
use xilem::WidgetView;
use xilem::core::Edit;
use crate::data::AppState;

pub fn library_view() -> impl WidgetView<Edit<AppState>> {
    label("Library View")
}
