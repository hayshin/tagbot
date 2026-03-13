# Telegram Tag Bot

A Telegram bot built with Rust and Teloxide that allows users to create and manage tags for group mentions.

## Features

- **Tag Management**: Users can add themselves to custom tags
- **Selective Calling**: Call specific tags or all users in the group
- **Per-Group Storage**: Each group maintains its own set of tags and users

## Commands

- `/join [tag_name]` - Join a specific tag (defaults to "алл")
- `/leave [tag_name]` - Leave a specific tag (defaults to "алл")
- `/call [tag_name]` - Mention all users in the specified tag (or all non-muted users if "алл" or no tag specified)
- `/list` - List all tags in this group
- `/help` - Show available commands

## Deployment

### GitHub Actions (Automated)

1. Push your code to a GitHub repository.
2. Create a tag: `git tag v1.0.0 && git push origin v1.0.0`.
3. The GitHub Action will automatically:
   - Build a release binary and attach it to a new GitHub Release.
   - Build and push a Docker image to GitHub Container Registry (`ghcr.io`).

### Nix Deployment (Manual)

If you have Nix installed with flakes enabled:

1. **Build and Run Locally:**
   ```bash
   nix run .
   ```

2. **Build Docker Image:**
   ```bash
   # Build the image
   nix build .#dockerImage
   
   # Load it into your local Docker daemon
   docker load < result
   
   # Run the container
   docker run -d \
     --name tagbot \
     -v ./data:/app/data \
     -e TELOXIDE_TOKEN=your_token \
     -e DATABASE_URL=/app/data/tagbot.db \
     tagbot:latest
   ```

### Binary Deployment

1. Download the latest binary from [Releases](../../releases).
2. Create a `.env` file (see `.env.example`):
   - `TELOXIDE_TOKEN=your_token`
   - `DATABASE_URL=./tagbot.db`
3. Run: `./tagbot`.

## Usage Example

1. Add the bot to your Telegram group.
2. Users can join tags:
   - `/join developers`
   - `/join designers`
   - `/join` - join the default "алл" tag.
3. Call specific groups:
   - `/call developers` - mentions all users in the "developers" tag.
   - `/call` - mentions all non-muted users (the default "алл" tag).
4. Leave tags:
   - `/leave developers`
   - `/leave` - leave the "алл" tag.
5. Leave tags:
   - `/leave developers`
   - `/leave` - leave the "алл" tag.

## Project Structure

```
.
├── src/
│   ├── main.rs          # Bot entry point and dispatcher
│   ├── db.rs            # Database operations (SQLite)
│   ├── models.rs        # Data models
│   └── commands/        # Command handlers
│       ├── mod.rs       # Command definition
│       ├── call.rs      # /call handler
│       ├── join.rs      # /join handler
│       ├── leave.rs     # /leave handler
│       ├── list.rs      # /list handler
│       └── responses.rs # Bot responses
├── Cargo.toml           # Rust dependencies
├── flake.nix            # Nix flake configuration
├── .env.example         # Environment variable template
└── README.md           # This file
```

## Notes

- Data is persisted in a SQLite database (`tagbot.db`).
- Each group maintains its own separate tag system.
- Users can only add/remove themselves, not other users.
- **Private Notifications**: To receive direct messages when you're called in a group, you MUST start a private chat with the bot (send any message to it).
