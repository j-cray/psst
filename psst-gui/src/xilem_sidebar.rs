use xilem::{
    view::{button, flex_col, label, FlexSpacer, ViewCtx},
    core::View,
};
use crate::data::{AppState, Nav};

pub fn sidebar() -> impl View<AppState, (), xilem::ViewCtx> {
    label("Sidebar")
}
