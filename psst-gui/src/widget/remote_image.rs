use std::sync::Arc;
use xilem::core::{MessageProxy, ViewMarker, ViewArgument};
use xilem::view::{task, image as img_view, zstack};
use xilem::masonry::peniko::ImageBrush;
use xilem::view::ObjectFit;
use xilem::{ViewCtx, WidgetView, AnyWidgetView};

// Legacy exports for Delegate
use druid::{ImageBuf, Selector};

pub const REQUEST_DATA: Selector<Arc<str>> = Selector::new("remote-image.request-data");
pub const PROVIDE_DATA: Selector<ImagePayload> = Selector::new("remote-image.provide-data");

#[derive(Clone)]
pub struct ImagePayload {
    pub location: Arc<str>,
    pub image_buf: ImageBuf,
}

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
    loader: impl Fn(&mut State, Arc<str>) -> Action + Send + Sync + 'static,
    image: Option<ImageBrush>,
    location: Option<Arc<str>>,
    placeholder: V,
) -> Box<AnyWidgetView<State, Action>>
where
    State: ViewArgument + 'static,
    Action: 'static,
    V: WidgetView<State> + ViewMarker + Clone,
{
    if let Some(img) = image {
        Box::new(img_view(img).fit(ObjectFit::Cover))
    } else {
        let placeholder_view = placeholder.clone();
        if let Some(loc) = location {
            let req_task = task(
                move |proxy: MessageProxy<Arc<str>>, _| async move {
                    let _ = proxy.message(loc);
                },
                move |state: &mut State, loc: Arc<str>| {
                    loader(state, loc)
                }
            );

            Box::new(zstack((placeholder_view, req_task)))
        } else {
             Box::new(placeholder_view)
        }
    }
}

use druid::{
    widget::{prelude::*, FillStrat, Image},
    Data, Point, WidgetPod,
};

pub struct RemoteImage<T> {
    placeholder: WidgetPod<T, Box<dyn Widget<T>>>,
    image: Option<WidgetPod<T, Image>>,
    locator: Box<dyn Fn(&T, &Env) -> Option<Arc<str>>>,
    location: Option<Arc<str>>,
}

impl<T: Data> RemoteImage<T> {
    pub fn new(
        placeholder: impl Widget<T> + 'static,
        locator: impl Fn(&T, &Env) -> Option<Arc<str>> + 'static,
    ) -> Self {
        Self {
            placeholder: WidgetPod::new(placeholder).boxed(),
            locator: Box::new(locator),
            location: None,
            image: None,
        }
    }
}

impl<T: Data> Widget<T> for RemoteImage<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::Command(cmd) = event {
            if let Some(payload) = cmd.get(PROVIDE_DATA) {
                if Some(&payload.location) == self.location.as_ref() {
                    self.image.replace(WidgetPod::new(
                        Image::new(payload.image_buf.clone()).fill_mode(FillStrat::Cover),
                    ));
                    ctx.children_changed();
                }
                return;
            }
        }
        if let Some(image) = self.image.as_mut() {
            image.event(ctx, event, data, env);
        } else {
            self.placeholder.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            let location = (self.locator)(data, env);
            self.image = None;
            self.location.clone_from(&location);
            if let Some(location) = location {
                ctx.submit_command(REQUEST_DATA.with(location).to(ctx.widget_id()));
            }
        }
        if let Some(image) = self.image.as_mut() {
            image.lifecycle(ctx, event, data, env);
        } else {
            self.placeholder.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        let location = (self.locator)(data, env);
        if location != self.location {
            self.image = None;
            self.location.clone_from(&location);
            if let Some(location) = location {
                ctx.submit_command(REQUEST_DATA.with(location).to(ctx.widget_id()));
            }
            ctx.children_changed();
        }
        if let Some(image) = self.image.as_mut() {
            image.update(ctx, data, env);
        } else {
            self.placeholder.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        if let Some(image) = self.image.as_mut() {
            let size = image.layout(ctx, bc, data, env);
            image.set_origin(ctx, Point::ORIGIN);
            size
        } else {
            let size = self.placeholder.layout(ctx, bc, data, env);
            self.placeholder.set_origin(ctx, Point::ORIGIN);
            size
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if let Some(image) = self.image.as_mut() {
            image.paint(ctx, data, env)
        } else {
            self.placeholder.paint(ctx, data, env)
        }
    }
}
