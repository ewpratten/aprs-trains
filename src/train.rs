use aprs_encode::{ddm::{DdmLongitude, DegreeMinutes, DdmLatitude, CardinalDirection}, stack_str::PackArrayString};
use arrayvec::ArrayString;

#[derive(Debug)]
pub struct TrainInfo {
    /// Identifier for the train
    pub identifier: String,
    /// Current speed in km/h
    pub speed: f64,
    /// Current latitude
    pub latitude: f32,
    /// Current longitude
    pub longitude: f32,
    /// Current course of the train
    pub course: Option<f32>,
    /// Status message
    pub status: String,
}

impl TrainInfo {
    /// Get the location in DDMM.hhN/DDDMM.hhW format
    pub fn location_ddm(&self) -> String {
        let mut ddm_longitude = ArrayString::<128>::new();
        DdmLongitude {
            ddm: DegreeMinutes::from(self.longitude.abs()),
            direction: if self.longitude >= 0.0 {
                CardinalDirection::East
            } else {
                CardinalDirection::West
            },
        }
        .pack_into(&mut ddm_longitude)
        .unwrap();
        let mut ddm_latitude = ArrayString::<128>::new();
        DdmLatitude {
            ddm: DegreeMinutes::from(self.latitude.abs()),
            direction: if self.latitude >= 0.0 {
                CardinalDirection::North
            } else {
                CardinalDirection::South
            },
        }
        .pack_into(&mut ddm_latitude)
        .unwrap();

        format!("{}/{}", ddm_latitude, ddm_longitude)
    }

    /// Get the identifier padded with spaces to 9 characters
    pub fn identifier_padded(&self) -> String {
        let mut output = self.identifier.clone();
        while output.len() < 9 {
            output.push(' ');
        }
        output
    }
}
