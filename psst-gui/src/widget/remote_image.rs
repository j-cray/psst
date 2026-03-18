use std::sync::Arc;
use xilem::core::{MessageProxy, ViewMarker, ViewArgument};
use xilem::view::{task, image as img_view, zstack};
use xilem::masonry::peniko::ImageBrush;
use xilem::view::ObjectFit;
use xilem::{WidgetView, AnyWidgetView};

// Common trait for data that can provide an image location
pub trait ImageLoc {
    fn image_location(&self) -> Option<Arc<str>>;
}

impl ImageLoc for Option<Arc<str>> {
    fn image_location(&self) -> Option<Arc<str>> {
        self.clone()
    }
}

pub fn remote_image<State, Action, V>(
    _loader: impl Fn(&mut State, Arc<str>) -> Action + Send + Sync + 'static,
    image: Option<ImageBrush>,
    _location: Option<Arc<str>>,
    placeholder: V,
) -> Box<AnyWidgetView<State, Action>>
where
    State: ViewArgument + 'static,
    Action: Send + Sync + 'static,
    V: WidgetView<State, Action> + ViewMarker + 'static,
{
    if let Some(img) = image {
        // Map the img_view's Action (which is `()`) to the caller's `Action` type
        use xilem::core::View;
        img_view(img).fit(ObjectFit::Cover).map_action(|_, _: ()| unreachable!()).boxed()
    } else {
        // Temporarily disable the task until we sort out Xilem's generic View bounds for Background Tasks
        placeholder.boxed()
    }
}

