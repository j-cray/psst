use xilem::core::Edit;
use xilem::view::{flex_row, flex_col, label, FlexSpacer, FlexExt};
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};
use xilem::masonry::layout::AsUnit;
use psst_gui::ui::xilem_playback::playback_bar;
use psst_gui::ui::xilem_sidebar::sidebar;
use psst_gui::ui::xilem_home::home_view;
use psst_gui::data::{AppState, Config};
use psst_core::cache::Cache;
use psst_gui::webapi::WebApi;

fn main_content() -> impl WidgetView<Edit<AppState>> + use<> {
    flex_col((
        label("Topbar").flex(0.0),
        FlexSpacer::Fixed(1.px()),
        home_view().flex(1.0),
        FlexSpacer::Fixed(1.px()),
        playback_bar().flex(0.0),
    ))
}

fn app_logic(_data: &mut AppState) -> impl WidgetView<Edit<AppState>> + use<> {
    flex_row((
        sidebar().flex(0.2), // acts like a Split view sidebar
        FlexSpacer::Fixed(1.px()), // Split bar
        main_content().flex(0.8),
    ))
}

fn main() {
    let config = Config::load().unwrap_or_default();
    let state = AppState::default_with_config(config.clone());

    let app = Xilem::new_simple(state, app_logic, WindowOptions::new("Psst Xilem"));
    let _ = app.run_in(EventLoop::with_user_event());
}
