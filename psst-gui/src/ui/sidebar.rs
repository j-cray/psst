use xilem::{
    view::{button, flex_col, label, FlexSpacer},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, Nav};

pub fn sidebar() -> impl WidgetView<Edit<AppState>> {
    label("Sidebar")
}
