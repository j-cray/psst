use psst_gui::ui::xilem_theme::AppTheme;
use xilem::{
    core::Edit,
    view::{button, flex, label, Axis},
    window,
    winit::dpi::LogicalSize,
    AppState, EventLoop, WidgetView, WindowId, Xilem,
};

#[derive(Clone, Debug)]
struct MyState {
    count: i32,
    nav_title: String,
    theme: AppTheme,
    window_id: WindowId,
}

impl Default for MyState {
    fn default() -> Self {
        Self {
            count: 0,
            nav_title: "Home".to_string(),
            theme: AppTheme::dark(),
            window_id: WindowId::next(),
        }
    }
}

impl AppState for MyState {
    fn keep_running(&self) -> bool {
        true
    }
}

fn sidebar(state: &MyState) -> impl WidgetView<MyState> {
    flex(
        Axis::Vertical,
        (
            label("Psst"),
            label("Sidebar (Playlists)"),
            button("Home", |state: &mut MyState| {
                state.nav_title = "Home".to_string()
            }),
            button("Tracks", |state: &mut MyState| {
                state.nav_title = "Saved Tracks".to_string()
            }),
            button("Albums", |state: &mut MyState| {
                state.nav_title = "Saved Albums".to_string()
            }),
            button("Toggle Theme", |state: &mut MyState| {
                state.theme = AppTheme::light();
            }),
        ),
    )
}

fn playback_panel(state: &MyState) -> impl WidgetView<MyState> {
    flex(
        Axis::Horizontal,
        (
            label("Playback Controls"),
            button("Play/Pause", |state: &mut MyState| {
                println!("Toggle playback");
            }),
        ),
    )
}

fn app_logic(state: &mut MyState) -> impl WidgetView<Edit<MyState>> {
    let topbar = flex(
        Axis::Horizontal,
        (
            label("Back"),
            label(format!("Current View: {}", state.nav_title)),
            label("Search"),
        ),
    );

    let main_content = flex(
        Axis::Vertical,
        (
            topbar,
            label(format!("Counter: {}", state.count)),
            button("Increment", |state: &mut MyState| state.count += 1),
        ),
    );

    flex(
        Axis::Horizontal,
        (
            sidebar(state),
            flex(Axis::Vertical, (main_content, playback_panel(state))),
        ),
    )
}

fn main() {
    let app = Xilem::new(MyState::default(), |state| {
        let min_width = state.theme.grid(65.0);
        let min_height = state.theme.grid(50.0);

        std::iter::once(
            window(state.window_id, "Psst", app_logic(state)).with_options(move |opts| {
                opts.with_min_inner_size(LogicalSize::new(min_width, min_height))
                    .with_initial_inner_size(LogicalSize::new(min_width * 1.5, min_height * 1.5))
            }),
        )
    });

    app.run_in(EventLoop::with_user_event()).unwrap();
}
