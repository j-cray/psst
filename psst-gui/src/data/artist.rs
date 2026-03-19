use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::data::{Album, Cached, Image, Promise, Track};

#[derive(Clone)]
pub struct ArtistDetail {
    pub artist: Promise<Artist, ArtistLink>,
    pub albums: Promise<ArtistAlbums, ArtistLink>,
    pub top_tracks: Promise<ArtistTracks, ArtistLink>,
    pub related_artists: Promise<Cached<Vec<Artist>>, ArtistLink>,
    pub artist_info: Promise<ArtistInfo, ArtistLink>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Artist {
    pub id: Arc<str>,
    pub name: Arc<str>,
    pub images: Vec<Image>,
}

impl Artist {
    pub fn image(&self, width: f64, height: f64) -> Option<&Image> {
        Image::at_least_of_size(&self.images, width, height)
    }

    pub fn link(&self) -> ArtistLink {
        ArtistLink {
            id: self.id.clone(),
            name: self.name.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ArtistAlbums {
    pub albums: Vec<Arc<Album>>,
    pub singles: Vec<Arc<Album>>,
    pub compilations: Vec<Arc<Album>>,
    pub appears_on: Vec<Arc<Album>>,
}
#[derive(Clone)]
pub struct ArtistInfo {
    pub main_image: Arc<str>,
    pub stats: ArtistStats,
    pub bio: String,
    pub artist_links: Vec<String>,
}

#[derive(Clone)]
pub struct ArtistStats {
    pub followers: i64,
    pub monthly_listeners: i64,
    pub world_rank: i64,
}

#[derive(Clone)]
pub struct ArtistTracks {
    pub id: Arc<str>,
    pub name: Arc<str>,
    pub tracks: Vec<Arc<Track>>,
}

impl ArtistTracks {
    pub fn link(&self) -> ArtistLink {
        ArtistLink {
            id: self.id.clone(),
            name: self.name.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct ArtistLink {
    pub id: Arc<str>,
    pub name: Arc<str>,
}

impl ArtistLink {
    pub fn url(&self) -> String {
        format!("https://open.spotify.com/artist/{id}", id = self.id)
    }
}
