use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::process::Command;

const API_BASE: &str = "https://beacon.playback.api.brightcove.com/telequebec/api";
const PLAYER_BASE: &str = "https://video.telequebec.tv/player";

#[derive(Parser)]
#[command(name = "tqc")]
#[command(about = "Download episodes from Télé-Québec")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all seasons and episodes
    List {
        /// Show slug (e.g., "32951-simon")
        show: String,
    },
    /// Download episode(s). Omit episode to download entire season, omit both for entire show.
    Download {
        /// Show slug (e.g., "32951-simon")
        show: String,
        /// Season number (omit to download entire show)
        season: Option<u32>,
        /// Episode number (omit to download entire season)
        episode: Option<u32>,
    },
}

#[derive(Debug, Deserialize)]
struct Season {
    id: u64,
    name: String,
    seasons_number: u32,
}

#[derive(Debug, Deserialize)]
struct Asset {
    id: u64,
    name: String,
    seasons: Vec<Season>,
}

#[derive(Debug, Deserialize)]
struct AssetData {
    asset: Asset,
}

#[derive(Debug, Deserialize)]
struct AssetResponse {
    data: AssetData,
}

#[derive(Debug, Deserialize)]
struct Episode {
    id: u64,
    original_name: String,
    episode_number: u32,
    season_number: u32,
    length: u32,
}

#[derive(Debug, Deserialize)]
struct PaginationUrls {
    next: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Pagination {
    url: PaginationUrls,
}

#[derive(Debug, Deserialize)]
struct EpisodesResponse {
    pagination: Pagination,
    data: Vec<Episode>,
}

async fn fetch_show(slug: &str) -> Result<Asset, Box<dyn std::error::Error>> {
    let url = format!("{}/assets/{}?device_type=web&device_layout=web", API_BASE, slug);
    let response = reqwest::get(&url).await?.json::<AssetResponse>().await?;
    Ok(response.data.asset)
}

async fn fetch_episodes(show_id: u64, season_id: u64) -> Result<Vec<Episode>, Box<dyn std::error::Error>> {
    let mut episodes = Vec::new();
    let mut url = format!(
        "{}/tvshow/{}/season/{}/episodes?device_type=web&device_layout=web&layout_id=320",
        API_BASE, show_id, season_id
    );

    loop {
        let response: EpisodesResponse = reqwest::get(&url).await?.json().await?;
        episodes.extend(response.data);

        match response.pagination.url.next {
            Some(next_url) => url = next_url,
            None => break,
        }
    }

    Ok(episodes)
}

async fn list_show(show: &Asset) -> Result<(), Box<dyn std::error::Error>> {
    println!("Show: {} (ID: {})", show.name, show.id);

    for season in &show.seasons {
        println!("\n{} (ID: {}) (... episodes):", season.name, season.id);

        let episodes = fetch_episodes(show.id, season.id).await?;

        print!("\x1b[1A\x1b[2K");
        println!("{} (ID: {}) ({} episodes):", season.name, season.id, episodes.len());

        for ep in &episodes {
            println!(
                "  EP{:02} (ID: {}): {} ({} min)",
                ep.episode_number, ep.id, ep.original_name, ep.length
            );
        }
    }

    Ok(())
}

fn download_single_episode(show_name: &str, season_num: u32, episode: &Episode) -> bool {
    let url = format!("{}/{}/stream", PLAYER_BASE, episode.id);
    let output_name = format!(
        "{} - S{:02}E{:02} - {}.%(ext)s",
        show_name, season_num, episode.episode_number, episode.original_name
    );

    let status = Command::new("yt-dlp")
        .arg("-o")
        .arg(&output_name)
        .arg(&url)
        .status();

    match status {
        Ok(s) if s.success() => true,
        Ok(s) => {
            eprintln!("yt-dlp exited with status: {}", s);
            false
        }
        Err(e) => {
            eprintln!("Failed to run yt-dlp: {}", e);
            false
        }
    }
}

struct EpisodeToDownload {
    season_num: u32,
    episode: Episode,
}

async fn download(
    show: &Asset,
    season_num: Option<u32>,
    episode_num: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let seasons: Vec<&Season> = match season_num {
        Some(num) => {
            let season = show
                .seasons
                .iter()
                .find(|s| s.seasons_number == num)
                .ok_or_else(|| format!("Season {} not found", num))?;
            vec![season]
        }
        None => show.seasons.iter().collect(),
    };

    // Collect all episodes to download
    let mut all_episodes: Vec<EpisodeToDownload> = Vec::new();
    for season in seasons {
        let episodes = fetch_episodes(show.id, season.id).await?;

        let episodes_to_download: Vec<Episode> = match episode_num {
            Some(num) => {
                let episode = episodes
                    .into_iter()
                    .find(|e| e.episode_number == num)
                    .ok_or_else(|| {
                        format!("Episode {} not found in season {}", num, season.seasons_number)
                    })?;
                vec![episode]
            }
            None => episodes,
        };

        for episode in episodes_to_download {
            all_episodes.push(EpisodeToDownload {
                season_num: season.seasons_number,
                episode,
            });
        }
    }

    let total = all_episodes.len();
    let mut failed: Vec<String> = Vec::new();

    for (i, ep) in all_episodes.iter().enumerate() {
        println!(
            "[{}/{}] Downloading: {} S{:02}E{:02} - {}",
            i + 1,
            total,
            show.name,
            ep.season_num,
            ep.episode.episode_number,
            ep.episode.original_name
        );

        if !download_single_episode(&show.name, ep.season_num, &ep.episode) {
            failed.push(format!(
                "S{:02}E{:02} - {}",
                ep.season_num, ep.episode.episode_number, ep.episode.original_name
            ));
        }
    }

    println!();
    if failed.is_empty() {
        println!("Download complete! {} episodes downloaded.", total);
    } else {
        println!(
            "Download complete. {}/{} succeeded, {} failed:",
            total - failed.len(),
            total,
            failed.len()
        );
        for f in &failed {
            println!("  - {}", f);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { show } => {
            let show = fetch_show(&show).await?;
            list_show(&show).await?
        }
        Commands::Download { show, season, episode } => {
            let show = fetch_show(&show).await?;
            download(&show, season, episode).await?
        }
    }

    Ok(())
}
