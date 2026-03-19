use xilem::{
    view::label,
    WidgetView,
};
use xilem::core::Edit;
use crate::data::AppState;

pub fn home_view() -> impl WidgetView<Edit<AppState>> {
    label("Home View")
}
