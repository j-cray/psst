use psst_gui::ui::theme::AppTheme;
use xilem::masonry::dpi::LogicalSize;
use xilem::{
    core::Edit,
    view::{button, flex_col, flex_row, label},
    window, AppState, EventLoop, WidgetView, WindowId, Xilem,
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

type AppEdit = Edit<MyState>;

fn sidebar(_state: &MyState) -> impl WidgetView<AppEdit> {
    flex_col::<AppEdit, _, _>((
        label("Psst"),
        label("Sidebar (Playlists)"),
        button::<AppEdit, _, _, _>(label("Home"), |state| state.nav_title = "Home".to_string()),
        button::<AppEdit, _, _, _>(label("Tracks"), |state| {
            state.nav_title = "Saved Tracks".to_string()
        }),
        button::<AppEdit, _, _, _>(label("Albums"), |state| {
            state.nav_title = "Saved Albums".to_string()
        }),
        button::<AppEdit, _, _, _>(label("Toggle Theme"), |state| {
            state.theme = AppTheme::light();
        }),
    ))
}

fn playback_panel(_state: &MyState) -> impl WidgetView<AppEdit> {
    flex_row::<AppEdit, _, _>((
        label("Playback Controls"),
        button::<AppEdit, _, _, _>(label("Play/Pause"), |_state| {
            println!("Toggle playback");
        }),
    ))
}

fn app_logic(state: &mut MyState) -> impl WidgetView<AppEdit> {
    let topbar = flex_row::<AppEdit, _, _>((
        label("Back"),
        label(format!("Current View: {}", state.nav_title)),
        label("Search"),
    ));

    let main_content = flex_col::<AppEdit, _, _>((
        topbar,
        label(format!("Counter: {}", state.count)),
        button::<AppEdit, _, _, _>(label("Increment"), |state| state.count += 1),
    ));

    flex_row::<AppEdit, _, _>((
        sidebar(state),
        flex_col::<AppEdit, _, _>((main_content, playback_panel(state))),
    ))
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
