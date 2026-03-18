use xilem::{
    view::{button, flex_col, flex_row, label, FlexSpacer},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::AppState;

pub fn home_view() -> impl WidgetView<Edit<AppState>> {
    label("Home View")
}
