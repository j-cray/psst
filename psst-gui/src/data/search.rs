use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::data::{
    Album, Artist, Playlist, Promise, Show, Track,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchQuery {
    pub input: Arc<str>,
    pub topic: Option<SearchTopic>,
}

#[derive(Clone)]
pub struct Search {
    pub input: Arc<str>,
    pub topic: Option<SearchTopic>,
    pub results: Promise<SearchResults, SearchQuery>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SearchTopic {
    Artist,
    Album,
    Track,
    Playlist,
    Show,
}

impl SearchTopic {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchTopic::Artist => "artist",
            SearchTopic::Album => "album",
            SearchTopic::Track => "track",
            SearchTopic::Playlist => "playlist",
            SearchTopic::Show => "show",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SearchTopic::Artist => "Artists",
            SearchTopic::Album => "Albums",
            SearchTopic::Track => "Tracks",
            SearchTopic::Playlist => "Playlists",
            SearchTopic::Show => "Podcasts",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Artist,
            Self::Album,
            Self::Track,
            Self::Playlist,
            Self::Show,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct SearchResults {
    pub query: Arc<str>,
    pub topic: Option<SearchTopic>,
    pub artists: Vec<Artist>,
    pub albums: Vec<Arc<Album>>,
    pub tracks: Vec<Arc<Track>>,
    pub playlists: Vec<Playlist>,
    pub shows: Vec<Arc<Show>>,
}

impl SearchResults {
    pub fn is_empty(&self) -> bool {
        self.artists.is_empty()
            && self.albums.is_empty()
            && self.tracks.is_empty()
            && self.playlists.is_empty()
            && self.shows.is_empty()
    }
}
