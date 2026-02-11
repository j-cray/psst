use xilem::{
    view::{button, flex, label},
    App, AppLauncher, View,
};

#[derive(Default)]
struct AppState {
    count: i32,
    nav_title: String,
}

impl AppState {
    fn new() -> Self {
        AppState {
            count: 0,
            nav_title: "Home".to_string(),
        }
    }
}

// Mimic sidebar_menu_widget
fn sidebar(state: &AppState) -> impl View<AppState> {
    flex((
        label("Sidebar (Playlists)"),
        button("Home", |state: &mut AppState| {
            state.nav_title = "Home".to_string()
        }),
        button("Tracks", |state: &mut AppState| {
            state.nav_title = "Saved Tracks".to_string()
        }),
        button("Albums", |state: &mut AppState| {
            state.nav_title = "Saved Albums".to_string()
        }),
    ))
}

// Mimic playback::panel_widget
fn playback_panel(state: &AppState) -> impl View<AppState> {
    flex((
        label("Playback Controls"),
        button("Play/Pause", |state: &mut AppState| {
            println!("Toggle playback");
        }),
    ))
}

// Mimic root_widget structure: Sidebar | Main (Header, Content, Footer)
fn app_logic(state: &mut AppState) -> impl View<AppState> {
    let main_content = flex((
        label(format!("Current View: {}", state.nav_title)),
        label(format!("Counter: {}", state.count)),
        button("Increment", |state: &mut AppState| state.count += 1),
    ));

    // Horizontal split: Sidebar | Main Column
    flex((
        sidebar(state),
        flex((
            label("Header (Nav, Search)"),
            main_content,
            playback_panel(state),
        )),
    ))
}

fn main() {
    let app = App::new(AppState::new(), app_logic);
    AppLauncher::new(app).run();
}
