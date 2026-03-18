use xilem::view::label;
use xilem::{EventLoop, WindowOptions, Xilem, WidgetView};
use xilem::core::Edit;
use psst_gui::data::{AppState, Config};

fn app_logic(_data: &mut AppState) -> impl WidgetView<Edit<AppState>> {
    label("Psst Xilem UI")
}

fn main() {
    let config = Config::load().unwrap_or_default();
    let state = AppState::default_with_config(config.clone());

    let window_options = WindowOptions::new("Psst Xilem");
    let app = Xilem::new_simple(state, app_logic, window_options);
    let _ = app.run_in(EventLoop::with_user_event());
}
