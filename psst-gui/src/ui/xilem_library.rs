use xilem::core::Edit;
use xilem::view::{flex_row, flex_col, label, button, FlexSpacer, FlexExt};
use xilem::WidgetView;

pub fn library_view() -> impl WidgetView<Edit<()>> + use<> {
    flex_col((
        label("Your Library").flex(0.0),
        // Placeholder for saved tracks/playlists
        flex_row((
            button("Liked Songs", |_| {}).flex(1.0),
            button("Albums", |_| {}).flex(1.0),
        )).flex(1.0),
    ))
}
