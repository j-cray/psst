use xilem::WidgetView;
use xilem::core::Edit;
use crate::data::{AppState, AppEvent};
use xilem::view::{image as xilem_image, label, FlexExt};
use xilem::masonry::peniko::{ImageData, ImageFormat, ImageAlphaType};
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashSet;
use crate::webapi::WebApi;

fn in_flight_images() -> &'static Mutex<HashSet<Arc<str>>> {
    static IN_FLIGHT_IMAGES: OnceLock<Mutex<HashSet<Arc<str>>>> = OnceLock::new();
    IN_FLIGHT_IMAGES.get_or_init(|| Mutex::new(HashSet::new()))
}

pub fn image_view(img: &::image::DynamicImage) -> impl WidgetView<Edit<AppState>> {
    let rgba8 = img.to_rgba8();
    let (width, height) = rgba8.dimensions();
    let image_data = ImageData {
        data: rgba8.to_vec().into(), // masonry::peniko::Blob or similar
        format: ImageFormat::Rgba8,
        alpha_type: ImageAlphaType::Alpha,
        width,
        height,
    };
    
    // We could apply some preferred size via `.flex` or `SizedBox`, but here we just return the raw image
    xilem_image(image_data)
}

pub fn image_widget(state: &AppState, image_link: Option<Arc<str>>) -> impl WidgetView<Edit<AppState>> {
    if let Some(uri) = image_link {
        if let Some(cached) = WebApi::global().get_cached_image(&uri) {
            image_view(&cached).boxed()
        } else {
            let mut in_flight = in_flight_images().lock().unwrap();
            if !in_flight.contains(&uri) {
                in_flight.insert(uri.clone());
                let sender = state.event_sender.clone();
                let uri_clone = uri.clone();
                std::thread::spawn(move || {
                    let _ = WebApi::global().get_image(uri_clone.clone());
                    // Remove from in-flight memory so we don't leak, though cache hits would prevent re-entry anyway
                    if let Ok(mut lock) = in_flight_images().lock() {
                        lock.remove(&uri_clone);
                    }
                    let _ = sender.send(AppEvent::ImageLoaded(uri_clone));
                });
            }
            label("Loading...").boxed()
        }
    } else {
        label("").boxed()
    }
}
