use xilem::view::label;
use xilem::WidgetView;
use xilem::core::Edit;
use crate::data::AppState;

pub fn playback_bar() -> impl WidgetView<Edit<AppState>> {
    label("Playback Bar")
}
