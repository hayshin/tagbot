# Telegram Tag Bot

A Telegram bot built with Rust and Teloxide that allows users to create and manage tags for group mentions.

## Features

- **Tag Management**: Users can add themselves to custom tags
- **Mute Functionality**: Users can mute themselves to avoid being mentioned in group calls
- **Selective Calling**: Call specific tags or all users in the group
- **Per-Group Storage**: Each group maintains its own set of tags and users

## Commands

- `/mute` - Mute yourself (you won't be called in group mentions)
- `/unmute` - Unmute yourself (you will be called in group mentions again)
- `/join [tag_name]` - Join a specific tag (defaults to "all")
- `/left [tag_name]` - Leave a specific tag (defaults to "all")
- `/call [tag_name]` - Mention all users in the specified tag (or all non-muted users if "all" or no tag specified)
- `/ask [tag_name] [question]` - Pick a random user from the tag and respond with the question
- `/list` - List all tags in this group
- `/help` - Show available commands

## Deployment

### GitHub Actions (Recommended)

1. Push your code to a GitHub repository.
2. Create a tag: `git tag v1.0.0 && git push origin v1.0.0`.
3. The GitHub Action will automatically:
   - Build a release binary and attach it to a new GitHub Release.
   - Build and push a Docker image to GitHub Container Registry (`ghcr.io`).

### Manual Deployment (VPS)

1. Install Docker and Docker Compose on your server.
2. Copy `docker-compose.yml` to your server.
3. Create a `.env` file with `TELOXIDE_TOKEN`.
4. Run: `docker compose up -d`.

### Binary Deployment

1. Download the latest binary from [Releases](../../releases).
2. Set environment variables:
   - `TELOXIDE_TOKEN=your_token`
   - `DATABASE_URL=./data/tagbot.db`
3. Run: `./telegram-tag-bot`.

## Usage Example

1. Add the bot to your Telegram group
2. Users can join tags:
   - `/join developers`
   - `/join designers`
   - `/join` - join the default "all" tag
3. Call specific groups:
   - `/call developers` - mentions all users in the "developers" tag
   - `/call` - mentions all non-muted users (the default "all" tag)
4. Leave tags:
   - `/left developers`
   - `/left` - leave the "all" tag
5. Mute yourself to avoid group mentions:
   - `/mute`

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
│       ├── leave.rs     # /left handler
│       ├── list.rs      # /list handler
│       ├── mute.rs      # /mute handler
│       └── unmute.rs    # /unmute handler
├── Cargo.toml           # Rust dependencies
├── Dockerfile           # Docker build instructions
├── docker-compose.yml   # Docker Compose configuration
└── README.md           # This file
```

## Notes

- Data is persisted in a SQLite database (`tagbot.db`)
- Each group maintains its own separate tag system
- Muted users are excluded from `/call` results
- Users can only add/remove themselves, not other users
- **Private Notifications**: To receive direct messages when you're called in a group, you MUST start a private chat with the bot (send any message to it).
