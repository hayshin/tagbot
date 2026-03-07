# Telegram Tag Bot

A Telegram bot built with Rust and Teloxide that allows users to create and manage tags for group mentions.

## Features

- **Tag Management**: Users can add themselves to custom tags
- **Mute Functionality**: Users can mute themselves to avoid being mentioned in group calls
- **Selective Calling**: Call specific tags or all users in the group
- **Per-Group Storage**: Each group maintains its own set of tags and users

## Commands

- `/mute` - Mute yourself (you won't be called in group mentions unless explicitly tagged)
- `/join [tag_name]` - Join a specific tag (defaults to "all")
- `/left [tag_name]` - Leave a specific tag (defaults to "all")
- `/call [tag_name]` - Mention all users in the specified tag (or all non-muted users if "all" or no tag specified)
- `/list` - List all tags in this group
- `/help` - Show available commands

## Setup

### Prerequisites

1. Create a bot with [@BotFather](https://t.me/botfather) on Telegram
2. Get your bot token
3. Install Docker and Docker Compose

### Running with Docker Compose

1. Create a `.env` file in the project root:

```bash
TELOXIDE_TOKEN=your_bot_token_here
```

2. Build and run:

```bash
docker-compose up -d
```

3. Check logs:

```bash
docker-compose logs -f
```

### Running with Docker

```bash
# Build the image
docker build -t telegram-tag-bot .

# Run the container
docker run -d \
  --name telegram-tag-bot \
  -e TELOXIDE_TOKEN=your_bot_token_here \
  telegram-tag-bot
```

### Running locally (without Docker)

1. Install Rust: https://rustup.rs/

2. Set environment variable:

```bash
export TELOXIDE_TOKEN=your_bot_token_here
```

3. Run the bot:

```bash
cargo run --release
```

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
│   └── main.rs          # Bot logic
├── Cargo.toml           # Rust dependencies
├── Dockerfile           # Docker build instructions
├── docker-compose.yml   # Docker Compose configuration
└── README.md           # This file
```

## Notes

- Data is stored in memory and will be lost when the bot restarts
- Each group maintains its own separate tag system
- The "muted" tag is special and excludes users from general `/call` commands
- Users can only add/remove themselves, not other users
