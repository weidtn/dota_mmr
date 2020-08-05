use std::collections::HashMap;
// game_mode: 22 = role queue
// win/loss = +- 20 MMR
// Wins: 1061 Lose: 1125 = 2200 MMR
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // steamid = "60374563" // dota
    // steamid = "76561198020640291" // steam
    let request_url = format!(
        "https://api.opendota.com/api/players/{steamid}/wl?game_mode=22",
        steamid = "60374563"
    );
    // println!("{}", request_url);
    let response = reqwest::get(&request_url)
    .await?
    .json::<HashMap<String, i32>>()
    .await?;

    // println!("response: {:?}", response);
    // println!("lose: {:?}", lose);
    let wins = match response.get("win"){
        Some(x) => *x,
        None => 0 as i32
    };
    let lose = match response.get("lose"){
        Some(x) => *x,
        None => 0 as i32
    };
    let start_mmr = 2200;
    let diff_wins = 1061 - wins;
    let diff_lose = 1125 - lose;
    let curr_mmr = start_mmr + diff_wins * 20 - diff_lose * 20;
    println!("MMR: {}", curr_mmr);
    Ok(())
}
