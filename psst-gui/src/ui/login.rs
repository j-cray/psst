use xilem::{
    view::{button, flex_col, flex_row, label, text_input},
    WidgetView,
};
use xilem::core::Edit;
use crate::data::{AppState, AppEvent, Promise};

pub fn login_view(state: &AppState) -> impl WidgetView<Edit<AppState>> {
    let auth = &state.preferences.auth;
    
    let username_input = text_input(
        auth.username.clone(),
        |state: &mut AppState, new_val| {
            state.preferences.auth.username = new_val;
        }
    );
    
    let password_input = text_input(
        auth.password.clone(),
        |state: &mut AppState, new_val| {
            state.preferences.auth.password = new_val;
        }
    );
    
    let login_button = button(
        label("Log In"),
        |state: &mut AppState| {
            state.preferences.auth.result = Promise::Deferred { def: () };
            let _ = state.event_sender.send(AppEvent::SubmitLogin);
        }
    );
    
    let status_label = match &auth.result {
        Promise::Empty => label(""),
        Promise::Deferred { .. } => label("Authenticating..."),
        Promise::Resolved { .. } => label("Success!"),
        Promise::Rejected { err, .. } => label(format!("Error: {}", err)),
    };
    
    flex_col((
        label("Psst Login (No credentials found)"),
        flex_row((label("Username: "), username_input)),
        flex_row((label("Password: "), password_input)),
        login_button,
        status_label,
    ))
}
