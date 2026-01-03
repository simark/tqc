# tqc

A command-line tool for downloading episodes from [Télé-Québec](https://video.telequebec.tv/).

## Features

- **List shows** - Browse seasons and episodes with metadata (duration, IDs)
- **Flexible downloads** - Download a single episode, an entire season, or a complete show
- **Progress tracking** - Shows current progress (e.g., `[5/52]`) when downloading multiple episodes
- **Resilient** - Continues downloading remaining episodes if one fails, with a summary at the end

## Requirements

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) must be installed and available in your PATH

## Installation

```bash
cargo install --path .
```

Or build manually:

```bash
cargo build --release
./target/release/tqc --help
```

## Usage

### Finding a show slug

Show slugs can be found in Télé-Québec URLs. For example, the show "Simon" at:
```
https://video.telequebec.tv/tv-show/32951-simon
```
has the slug `32951-simon`.

### List all episodes

```bash
tqc list 32951-simon
```

Output:
```
Show: Simon (ID: 32951)

Saison 1 (ID: 32952) (52 episodes):
  EP01 (ID: 33285): Super lapin (6 min)
  EP02 (ID: 33284): Les petites roues (6 min)
  ...
```

### Download a single episode

```bash
tqc download 32951-simon 1 5    # Season 1, Episode 5
```

### Download an entire season

```bash
tqc download 32951-simon 1      # All of Season 1
```

### Download an entire show

```bash
tqc download 32951-simon        # All seasons
```

## Output format

Downloaded files are named:
```
{Show} - S{season}E{episode} - {Title}.mp4
```

For example: `Simon - S01E01 - Super lapin.mp4`

## How it works

tqc uses Télé-Québec's public Brightcove-based API to enumerate shows, seasons, and episodes. The actual video download is handled by yt-dlp, which supports the Télé-Québec/Brightcove player natively.

## License

MIT
