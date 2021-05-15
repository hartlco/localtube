# localtube
Just a hacky rust service to download the latest Youtube videos from RSS-Feeds using youtube-dl

## Setup
Define `config.toml` containing the `download_path` where the videos should be stored and a list of RSS-Feeds containing the videos to download.

```toml
feeds = [
    "<YOUTUBE-RSS-FEED-URL"
]

download_path = "<PATH>"
```

Optionally, define `downloaded.toml` for already downloaded video-ids. The service will append downloaded IDs to the file.

```toml
downloaded = []
```

## Run the service

```bash
cargo run PATH-TO-CONFIG-TOML OPTIONAL-PATH-TO-DOWNLOADED-TOML
```

# License
MIT