use xilem::view::label;
use xilem::core::View;
use crate::data::AppState;

pub fn playback_bar() -> impl View<AppState, (), xilem::ViewCtx> {
    label("Playback Bar")
}
