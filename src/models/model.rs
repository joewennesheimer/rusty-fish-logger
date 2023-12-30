use std::fmt::{Display, self};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fish {
    pub species: Species,
    pub weight: f32,
    pub length: f32,
    pub time_caught: DateTime<Utc>,
    pub location: String,
    pub bait: String,
    pub notes: String,
}

impl Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Species {
    LargemouthBass,
    SmallmouthBass,
    Catfish,
    Crappie,
    Walleye,
    Trout,
    Salmon,
    Muskie,
    Pike,
    Bluegill,
    Perch,
    Carp,
    Gar,
    Bowfin,
    Drum,
    Sturgeon,
}

impl Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
