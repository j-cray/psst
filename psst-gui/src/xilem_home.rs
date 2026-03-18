use xilem::{
    view::{button, flex_col, flex_row, label, FlexSpacer, ViewCtx},
    core::View,
};
use crate::data::AppState;

pub fn home_view() -> impl View<AppState, (), xilem::ViewCtx> {
    label("Home View")
}
