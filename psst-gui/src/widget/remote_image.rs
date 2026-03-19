use xilem::core::{ViewMarker, ViewArgument};
use xilem::view::image as img_view;
use xilem::masonry::peniko::ImageBrush;
use xilem::view::ObjectFit;
use xilem::{WidgetView, AnyWidgetView};

/// Display a remote image if a pre-loaded `ImageBrush` is provided,
/// otherwise render `placeholder`. Loading/fetching is the caller's
/// responsibility; wire your fetch logic to a background task and pass
/// the result in as `image` once available.
pub fn remote_image<State, Action, V>(
    image: Option<ImageBrush>,
    placeholder: V,
) -> Box<AnyWidgetView<State, Action>>
where
    State: ViewArgument + 'static,
    Action: Send + Sync + Default + 'static,
    V: WidgetView<State, Action> + ViewMarker + 'static,
{
    if let Some(img) = image {
        use xilem::core::View;
        img_view(img).fit(ObjectFit::Cover).map_action(|_, _: ()| Action::default()).boxed()
    } else {
        placeholder.boxed()
    }
}

