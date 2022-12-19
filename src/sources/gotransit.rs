//! Data source: http://gotracker.ca/gotracker/web/

use regex::Regex;

use crate::train::TrainInfo;

const CORRIDOR_IDS: [u8; 7] = [
    65, // Barrie
    31, // Kitchener
    09, // Lakeshore East
    01, // Lakeshore West
    21, // Milton
    61, // Richmond Hill
    71, // Stouffville
];

#[derive(Debug, serde::Deserialize)]
struct InServiceTripPublic {
    #[serde(rename = "EquipmentCode")]
    equipment_code: String,
    #[serde(rename = "Latitude")]
    latitude: f32,
    #[serde(rename = "Longitude")]
    longitude: f32,
    #[serde(rename = "StartStation")]
    start_station: String,
    #[serde(rename = "EndStation")]
    end_station: String,
    #[serde(rename = "TripNumber")]
    trip_number: String,
    #[serde(rename = "TripName")]
    trip_name: String,
    #[serde(rename = "CorridorCode")]
    corridor_code: String,
}

/// Get all trains from GO Transit
pub async fn get_trains() -> Result<Vec<TrainInfo>, reqwest::Error> {
    // Allocate output
    let mut output = Vec::new();

    // Handle each corridor
    for corridor_id in CORRIDOR_IDS.iter() {
        // Make a request to the gotracker API
        println!("Requesting data for GO transit corridor {}", corridor_id);
        let response = reqwest::get(&format!("http://gotracker.ca/GOTracker/web/GODataAPIProxy.svc/TripLocation/Service/Lang/{:02}/en", corridor_id)).await?;
        let response_text = response.text().await?;

        // Get only the data portion
        if let Some(data_xml) = Regex::new(r"<Data>(.*)</Data>")
            .unwrap()
            .captures(&response_text)
        {
            let data_xml = data_xml.get(1).unwrap().as_str();

            // Convert to something workable
            let parsed_data: Vec<InServiceTripPublic> = serde_xml_rs::from_str(data_xml).unwrap();

            // Convert to TrainInfo format
            for train in parsed_data {
                output.push(TrainInfo {
                    identifier: format!("GO-{}{}", train.corridor_code, train.equipment_code),
                    speed: 0.0,
                    latitude: train.latitude,
                    longitude: train.longitude,
                    course: None,
                    status: format!("{} to {}", train.start_station, train.end_station),
                });
            }
        }
    }

    // Return output
    Ok(output)
}
