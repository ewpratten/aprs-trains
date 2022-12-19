use crate::train::TrainInfo;

mod viarail;

pub async fn get_trains() -> Result<Vec<TrainInfo>, reqwest::Error> {
    // Build output
    let mut output = Vec::new();

    // Collect data
    output.extend(viarail::get_trains().await?);

    // Return output
    Ok(output)
}
