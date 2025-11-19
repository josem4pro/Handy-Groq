# Handy-Groq: Ultra-Fast Cloud Transcription

This fork of Handy adds **Groq Cloud API integration** for lightning-fast transcription speeds.

## What's New

- **New Model Option: "Groq Cloud (Whisper Large v3)"**
  - Speed Score: 0.99 (fastest available)
  - Accuracy Score: 0.95 (highest accuracy)
  - No local model download required
  - Requires internet connection

## Setup

### 1. Get a Groq API Key

1. Go to [https://console.groq.com/keys](https://console.groq.com/keys)
2. Sign up or log in
3. Create a new API key
4. Copy the key (starts with `gsk_`)

### 2. Set Environment Variable

**Linux/macOS:**
```bash
export GROQ_API_KEY="your_api_key_here"
```

To make it permanent, add to your `~/.bashrc` or `~/.zshrc`:
```bash
echo 'export GROQ_API_KEY="gsk_your_key_here"' >> ~/.bashrc
source ~/.bashrc
```

**Windows:**
```cmd
setx GROQ_API_KEY "your_api_key_here"
```

### 3. Select Groq Model in Handy

1. Open Handy settings
2. Go to Model Selection
3. Choose "Groq Cloud (Whisper Large v3)"
4. Start transcribing!

## How It Works

When you select the Groq model and press your transcription shortcut (Ctrl+Alt+Space by default):

1. Handy records your audio locally
2. Sends it to Groq's API via HTTPS
3. Receives transcription in ~1-2 seconds (vs 10-30s for local models)
4. Pastes the text where your cursor is

## Benefits

| Feature | Local Whisper | Groq Cloud |
|---------|--------------|------------|
| **Speed** | 10-30 seconds | 1-2 seconds |
| **Accuracy** | Good (0.85) | Excellent (0.95) |
| **Storage** | 1-2 GB | 0 GB |
| **Internet** | Not required | Required |
| **Privacy** | 100% local | Sent to Groq |

## Original Handy

This is a fork of the excellent [Handy](https://github.com/cjpais/Handy) project by cjpais.

For the original local-only version, visit: https://github.com/cjpais/Handy
