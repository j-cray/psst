use xilem::WidgetView;
use xilem::core::Edit;
use crate::data::{AppState, AppEvent};
use xilem::view::{image as xilem_image, label};
use xilem::masonry::peniko::{ImageData, ImageFormat, ImageAlphaType};
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashSet;
use crate::webapi::WebApi;

fn in_flight_images() -> &'static Mutex<HashSet<Arc<str>>> {
    static IN_FLIGHT_IMAGES: OnceLock<Mutex<HashSet<Arc<str>>>> = OnceLock::new();
    IN_FLIGHT_IMAGES.get_or_init(|| Mutex::new(HashSet::new()))
}

fn failed_images() -> &'static Mutex<HashSet<Arc<str>>> {
    static FAILED_IMAGES: OnceLock<Mutex<HashSet<Arc<str>>>> = OnceLock::new();
    FAILED_IMAGES.get_or_init(|| Mutex::new(HashSet::new()))
}

fn decoded_images() -> &'static Mutex<lru::LruCache<Arc<str>, ImageData>> {
    static DECODED_IMAGES: OnceLock<Mutex<lru::LruCache<Arc<str>, ImageData>>> = OnceLock::new();
    DECODED_IMAGES.get_or_init(|| Mutex::new(lru::LruCache::new(std::num::NonZeroUsize::new(200).unwrap())))
}



pub fn image_widget(state: &AppState, image_link: Option<Arc<str>>) -> impl WidgetView<Edit<AppState>> {
    if let Some(uri) = image_link {
        {
            let mut decoded = decoded_images().lock().unwrap();
            if let Some(img) = decoded.get(&uri) {
                return xilem_image(img.clone()).boxed();
            }
        }
        
        {
            let failed = failed_images().lock().unwrap();
            if failed.contains(&uri) {
                return label("").boxed();
            }
        }

        if let Some(cached) = WebApi::global().get_cached_image(&uri) {
            let rgba8 = cached.to_rgba8();
            let (width, height) = rgba8.dimensions();
            let image_data = ImageData {
                data: rgba8.to_vec().into(), // masonry::peniko::Blob or similar
                format: ImageFormat::Rgba8,
                alpha_type: ImageAlphaType::Alpha,
                width,
                height,
            };
            decoded_images().lock().unwrap().put(uri.clone(), image_data.clone());
            xilem_image(image_data).boxed()
        } else {
            let mut in_flight = in_flight_images().lock().unwrap();
            if !in_flight.contains(&uri) {
                in_flight.insert(uri.clone());
                let sender = state.event_sender.clone();
                let uri_clone = uri.clone();
                std::thread::spawn(move || {
                    let result = WebApi::global().get_image(uri_clone.clone());
                    if result.is_err() {
                        if let Ok(mut lock) = failed_images().lock() {
                            lock.insert(uri_clone.clone());
                        }
                    }

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
