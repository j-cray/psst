use xilem::masonry::dpi::LogicalSize;
use xilem::view::{flex_col, flex_row, label, FlexSpacer};
use xilem::{EventLoop, WindowOptions, Xilem, WidgetView};
use xilem::core::Edit;
use psst_gui::data::{AppState, Config, nav::Nav};

use psst_gui::ui::{
    home::home_view, playback::playback_bar, sidebar::sidebar,
};

fn topbar(state: &mut AppState) -> impl WidgetView<Edit<AppState>> {
    flex_row((
        label("Back"),
        label(format!("Route: {:?}", state.nav)),
        FlexSpacer::Flex(1.0),
        label("Search"),
    ))
}

fn app_logic(state: &mut AppState) -> impl WidgetView<Edit<AppState>> {
    let content = match state.nav {
        Nav::Home => home_view().boxed(),
        _ => label("Unimplemented Route").boxed(),
    };

    let main_content = flex_col((
        topbar(state),
        content,
    ));

    flex_row((
        sidebar(),
        flex_col((
            main_content,
            playback_bar(),
        )),
    ))
}

fn main() {
    let config = Config::load().unwrap_or_default();
    let state = AppState::default_with_config(config.clone());

    let window_options = WindowOptions::new("Psst Xilem")
        .with_min_inner_size(LogicalSize::new(800.0, 600.0))
        .with_initial_inner_size(LogicalSize::new(1024.0, 768.0));
        
    let app = Xilem::new_simple(state, app_logic, window_options);
    
    app.run_in(EventLoop::with_user_event()).expect("Failed to run Psst application");
}
