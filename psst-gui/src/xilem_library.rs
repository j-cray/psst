use xilem::view::label;
use xilem::core::View;
use crate::data::AppState;

pub fn library_view() -> impl View<AppState, (), xilem::ViewCtx> {
    label("Library View")
}
