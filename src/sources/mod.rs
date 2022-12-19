use crate::train::TrainInfo;

mod viarail;
mod gotransit;

pub async fn get_trains() -> Result<Vec<TrainInfo>, reqwest::Error> {
    // Build output
    let mut output = Vec::new();

    // Collect data
    output.extend(viarail::get_trains().await?);
    output.extend(gotransit::get_trains().await?);

    // Return output
    Ok(output)
}
