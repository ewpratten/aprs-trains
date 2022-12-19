use std::collections::HashMap;

use crate::train::TrainInfo;

/// Data format used by VIA for their train tracker
#[derive(Debug, serde::Deserialize)]
struct ViaTrainInfo {
    lat: Option<f32>,
    lng: Option<f32>,
    speed: Option<f64>,
    direction: Option<f32>,
    from: Option<String>,
    to: Option<String>,
}

/// Get all trains from VIA Rail
pub async fn get_trains() -> Result<Vec<TrainInfo>, reqwest::Error> {
    // Make a request to the tsimobile API
    let response = reqwest::get("https://tsimobile.viarail.ca/data/allData.json").await?;

    // Convert to something workable
    let parsed_data: HashMap<String, ViaTrainInfo> = response.json().await?;

    // Convert to TrainInfo format
    let mut output = Vec::new();

    for (id, train) in parsed_data {
        if let (Some(lat), Some(lng), Some(speed), Some(from), Some(to)) =
            (train.lat, train.lng, train.speed, train.from, train.to)
        {
            // Skip bracketed train names for my sanity
            if id.contains("(") {
                continue;
            }

            output.push(TrainInfo {
                identifier: format!("VIA-{}", id),
                speed,
                latitude: lat,
                longitude: lng,
                course: train.direction,
                status: format!("{} to {}", from, to),
            });
        }
    }

    // Return output
    Ok(output)
}
