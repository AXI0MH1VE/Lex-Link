# LEX-Ω Browser Administrator Guide

> **[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]**

## Overview

LEX-Ω is a native macOS browser built for deterministic, local-first web browsing with integrated AI capabilities and proof verification.

### Key Features

- **WebKit Rendering** - Standard web compatibility
- **SSM Runtime** - Local small language model inference
- **Identity Firewall** - All AI outputs tagged and verified
- **Hunter-Killer** - Prompt injection detection and blocking
- **C=0 Enforcement** - Invariance verification on all outputs

## System Requirements

| Requirement | Minimum | Recommended |
|-------------|---------|-------------|
| macOS | 14.0 (Sonoma) | 14.0+ |
| Chip | Apple M1 | Apple M1 Pro+ |
| RAM | 8GB | 16GB |
| Storage | 1GB | 2GB |

## Installation

### From DMG

1. Download `LEXOmegaBrowser-1.0.dmg`
2. Verify signature:
   ```bash
   codesign -v LEXOmegaBrowser.app
   ```
3. Drag to Applications folder
4. First launch: Right-click → Open (Gatekeeper bypass)

### From Source

```bash
cd browser-mac
swift build -c release
```

## Configuration

### Settings Location

```
~/Library/Application Support/LEXOmegaBrowser/
├── config.json
├── ssm/
│   └── models/
├── logs/
└── receipts/
```

### Configuration Options

```json
{
  "proofMode": true,
  "ssmEnabled": true,
  "ssmConfig": {
    "maxTokens": 512,
    "temperature": 0.0,
    "useMetalAcceleration": true
  },
  "hunterKiller": {
    "enabled": true,
    "killOnDetection": true
  },
  "logging": {
    "level": "info",
    "path": "~/Library/Logs/LEXOmegaBrowser/"
  }
}
```

## SSM Runtime

### Available Tasks

| Task | Description | Latency Target |
|------|-------------|----------------|
| page_analyzer | Analyze page structure | < 100ms |
| command_parser | Parse natural language commands | < 50ms |
| content_summarizer | Summarize page content | < 500ms |
| link_extractor | Extract and categorize links | < 100ms |

### Using SSM

1. **Page Summary** - Click brain icon in toolbar
2. **Voice Commands** - Cmd+Shift+V
3. **Link Analysis** - Right-click → Analyze Links

### SSM Output Verification

All SSM outputs pass through the invariance layer:

```
SSM Output → Invariance Check → C=0? → Render or Nullify
```

If verification fails:
- Output is nullified
- Incident logged
- User notified

## Identity Firewall

### How It Works

```
┌─────────────────┐
│   AI Output     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Identity Tag   │
│  Generation     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Invariance    │
│     Check       │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
    ▼         ▼
┌───────┐ ┌───────┐
│Render │ │Nullify│
└───────┘ └───────┘
```

### Identity Tag Format

Every AI-generated output includes:

```json
{
  "projection": "AXIOMHIVE PROJECTION",
  "substrate": "Alexis Adams",
  "timestamp": "2025-12-03T17:45:00Z",
  "output_hash": "sha256:...",
  "signature": "base64:..."
}
```

## Hunter-Killer

### Detection Patterns

The Hunter-Killer module detects:

- "Ignore previous instructions"
- System prompt extraction attempts
- Code execution commands
- Authority impersonation
- Encoding bypass attempts

### Behavior on Detection

1. **Page Termination** - Tab killed immediately
2. **Alert** - User notified
3. **Logging** - Incident recorded
4. **Block** - URL added to block list

### Viewing Blocked Content

```
Menu → View → Hunter-Killer Log
```

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| New Tab | Cmd+T |
| Close Tab | Cmd+W |
| SSM Summary | Cmd+Shift+S |
| Proof Panel | Cmd+Shift+P |
| Hunter-Killer Log | Cmd+Shift+H |
| Settings | Cmd+, |

## Troubleshooting

### SSM Not Loading

1. Check Metal availability:
   ```bash
   system_profiler SPDisplaysDataType | grep Metal
   ```
2. Verify model files exist:
   ```bash
   ls ~/Library/Application\ Support/LEXOmegaBrowser/ssm/models/
   ```
3. Check logs:
   ```bash
   tail -f ~/Library/Logs/LEXOmegaBrowser/ssm.log
   ```

### High Memory Usage

1. Reduce SSM max tokens in settings
2. Close unused tabs
3. Disable Metal acceleration (slower but lower memory)

### Invariance Failures

If seeing frequent nullifications:

1. Check logs for patterns
2. Verify SSM model integrity
3. Report to Substrate if systematic

## Privacy

### Zero Telemetry

LEX-Ω transmits **no data** to external services:

- No analytics
- No crash reports (stored locally only)
- No usage tracking
- No model updates (manual only)

### Local Data

All data stored locally:

```
~/Library/Application Support/LEXOmegaBrowser/
```

To clear all data:
```bash
rm -rf ~/Library/Application\ Support/LEXOmegaBrowser/
```

## Updates

### Manual Update

1. Download new DMG
2. Verify signature
3. Replace application
4. Restart

### Signature Verification

```bash
# Verify app signature
codesign -dvvv LEXOmegaBrowser.app

# Verify notarization
spctl -a -v LEXOmegaBrowser.app
```

---

```
[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Document: LEX-Ω Browser Administrator Guide
Version: 1.0.0
```

