<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=Zenora%20Miner%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Auto%20Login%20%7C%20Turnstile%20Solver%20%7C%20Auto%20Mining%20%7C%20Auto%20Rewards%20%7C%20Passkey&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Auto+Login+%7C+Cloudflare+Turnstile+Solver+Per+Login;Auto+Passkey+%7C+Enroll+Once+%7C+Cached+In+session.json;Auto+Mining+%7C+Streak+%7C+Start+Block+%7C+Claim+Session;Auto+Rewards+%7C+Airdrop+%7C+Giveaway+%7C+Ad+Gated+Tasks;Proxy+Support+%7C+One+Proxy+Per+Account;Multi+Account+%7C+Sequential+Processing+Per+Cycle"/>

<p>
  <img alt="ruby" src="https://img.shields.io/badge/Ruby-3.0+-CC342D?logo=ruby&logoColor=white"/>
  <img alt="rust" src="https://img.shields.io/badge/Rust-1.70+-000000?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-Zenora%20Miner-111111"/>
  <img alt="multi-account" src="https://img.shields.io/badge/Multi--Account-Supported-111111"/>
  <img alt="proxy" src="https://img.shields.io/badge/Proxy-Supported-111111"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>Zenora Miner Bot</b> is a full automation bot for the Zenora Miner web app.<br/>
  It handles the complete daily cycle opening an Cloudflare Turnstile challenge before each login, enrolling a passkey for each account once and caching it in <code>session.json</code>, counting the daily streak, starting a fresh mining block when the state is idle, claiming every ready session through the rewarded advertisement flow signed with the cached passkey, claiming available airdrop and giveaway rewards, completing ad-gated sponsored tasks, and reporting the final wallet balance and total claimed, all running across multiple accounts with proxy support and a live countdown between cycles.<br/>
  Built and distributed by <b>Yuurisandesu</b>.
</p>

</div>

---

## Table of Contents

- [Requirements](#requirements)
- [Installation](#installation)
- [Configuration](#configuration)
- [First-Time Setup](#first-time-setup)
- [Running the Bot](#running-the-bot)
- [Features](#features)
- [File Structure](#file-structure)
- [Disclaimer](#disclaimer)

---

## Requirements

- Rust `1.70+`
- Ruby `3.0+` (only needed to run the downloader `bot.rb`)
- Git
- A running Turnstile solver worker with the URL set in `solver/solver.yaml`

---

## Installation

### Install Rust

#### Linux / macOS / Android (Termux)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### Windows

Download and run the installer from https://rustup.rs

After installing, restart your terminal and verify:

```bash
rustc --version
cargo --version
```

---

### Install Ruby

#### Ubuntu / Debian

```bash
sudo apt install ruby
```

#### macOS

```bash
brew install ruby
```

#### Android (Termux)

```bash
pkg install ruby
```

#### Windows

Download and run the installer from https://rubyinstaller.org

---

**Clone the repository:**

```bash
git clone https://github.com/Yuurisan-N1/Zenora-Miner.git
cd Zenora-Miner
```

**Download the bot binary using the downloader:**

```bash
ruby bot.rb
```

The downloader will show a numbered menu. Enter the number that matches your platform and the binary will be downloaded automatically with a live progress bar showing the percentage, MB downloaded, and total size. For Linux binaries, file permissions are set to executable automatically after the download completes.

**Or build from source:**

```bash
./run.sh build
```

**Or download manually from the releases page:**

| Platform | Filename | Download |
|---|---|---|
| macOS aarch64 | `zenora-bot-macos-aarch64` | [Download](https://github.com/Yuurisan-N1/Zenora-Miner/releases/latest/download/zenora-bot-macos-aarch64) |
| Linux x86_64 | `zenora-bot-linux-x86_64` | [Download](https://github.com/Yuurisan-N1/Zenora-Miner/releases/latest/download/zenora-bot-linux-x86_64) |
| Windows x86_64 | `zenora-windows-x86_64.exe` | [Download](https://github.com/Yuurisan-N1/Zenora-Miner/releases/latest/download/zenora-windows-x86_64.exe) |

If you download the Linux or macOS binary manually, you need to set it as executable before running:

```bash
chmod +x zenora-bot-linux-x86_64
# or
chmod +x zenora-bot-macos-aarch64
```

---

## Configuration

### 1. Accounts (data.txt)

Fill `data.txt` with one account per line in `email|password` format. Blank lines are ignored. Keep the file private:

```
you@example.com|yourpassword
another@example.com|anotherpassword
```

```bash
chmod 600 data.txt
```

### 2. Proxy (proxy.txt) - Optional

Fill `proxy.txt` with proxies, one per line. Proxies are assigned to accounts by index (first proxy to first account, second proxy to second account, and so on). If the number of proxies is fewer than the number of accounts, proxies wrap around cyclically. If `proxy.txt` is missing or empty, the bot runs without a proxy.

```
http://user:pass@ip:port
http://user:pass@ip:port
```

Supported format: `http://user:pass@host:port`

### 3. Solver (solver/solver.yaml)

Fill in the URL of your Turnstile solver worker. The bot reads the `solver.url` field from this file before each login cycle.

```yaml
solver:
  url: https://solver.yuurisan.my.id
```

If `solver/solver.yaml` is missing or the URL is empty, the bot will refuse to start.

### 4. Bot Settings (config.json)

`sleep_seconds` controls how many seconds the bot waits between cycles. If `config.json` is missing or the key is absent, the bot falls back to a default of `3600` seconds.

```json
{
  "settings": {
    "sleep_seconds": 3600
  }
}
```

---

## First-Time Setup

Each account must have a passkey enrolled before the bot can claim mining sessions. The enrollment is done once per account and the passkey is cached in `session.json`. After that, all future runs reuse the cached passkey automatically.

**Step 1 - Add your accounts to `data.txt`.**

**Step 2 - Set the solver URL in `solver/solver.yaml`.**

**Step 3 - Run the bot.**

On the first run, for each account that has no cached passkey, the bot will:

1. Log in and trigger passkey enrollment.
2. Prompt you to enter the 6-digit OTP sent to the account email.
3. Complete the passkey registration and save it to `session.json`.

After the OTP step, the passkey is cached and the bot continues the cycle automatically. Future runs skip the enrollment entirely.

> Keep `session.json` private and back it up. If the file is lost, the enrollment must be repeated.

---

## Running the Bot

All modes are launched through `run.sh`. The script builds the binary automatically if it is missing or outdated before starting.

| Command | Description |
|---|---|
| `./run.sh run` | Run in the foreground |
| `./run.sh bg` | Run in the background with nohup, logs to `zenora-miner-bot.log` |
| `./run.sh screen` | Run inside a screen session |
| `./run.sh probe` | Check account status without running a full cycle |
| `./run.sh selftest` | Run a self-test to verify the setup |
| `./run.sh stop` | Stop the running bot |
| `./run.sh build` | Build the release binary only |

Press `Ctrl+C` at any time to stop the bot cleanly when running in the foreground.

---

## Features

### Encrypted Solver Channel
Before each login the bot opens a secure channel to the solver worker, submits the Turnstile task, and polls for the solved token. If the token cannot be obtained after the polling window, the account is skipped for that cycle.

### Auto Passkey Enrollment
If an account has no cached passkey, the bot handles the full enrollment flow with a one-time OTP and saves the passkey to `session.json`. Existing passkeys are reused automatically on future runs.

### Auto Login
The bot logs in with the account email, password, and solved Turnstile token. Accounts with invalid credentials or two-factor login enabled are skipped.

### Auto Daily Streak
The bot checks and counts the daily streak each cycle, keeping the streak bonus alive without manual taps.

### Auto Mining
The bot reads the current mining state. If idle, it starts a fresh block. If already running, the step is skipped. The state and available balance are logged either way.

### Auto Session Claim
When a mining session is ready, the bot runs the full rewarded advertisement flow and claims the session reward. The ZNR amount credited is logged on success.

### Auto Rewards
Every cycle the bot reads the campaign list and claims available airdrops, giveaways, and ad-gated sponsored tasks. Rewards requiring manual proof are reported. Already claimed rewards are skipped.

### Wallet Report
After all actions, the bot logs the ZNR balance, wallet address, and total ZNR claimed for the cycle.

### Proxy Support
Each account can be assigned its own proxy via `proxy.txt`. Proxy assignment uses round-robin if there are fewer proxies than accounts.

### Multi Account
All accounts in `data.txt` are processed sequentially within every cycle.

### Auto Countdown
After all accounts complete a cycle, the bot displays a live `HH:MM:SS` countdown until the next cycle starts.

---

## File Structure

```text
Zenora-Miner/
├── bot.rb                        # Downloader, select and fetch binary for your platform
├── build.rs                      # Build script
├── Cargo.toml                    # Rust project manifest
├── Cargo.lock                    # Dependency lock file
├── Makefile                      # Build shortcuts
├── run.sh                        # Launcher and process manager
├── config.json                   # Sleep duration between cycles
├── data.txt                      # Account credentials, email|password one per line
├── proxy.txt                     # Proxies, one per line (optional)
├── session.json                  # Passkey cache per account (auto-generated, keep private)
├── HOWTO.txt                     # Quick start reference
├── LICENSE                       # License file
├── assets/
│   ├── icon.ico                  # App icon
│   └── icon.png                  # App icon
├── solver/
│   └── solver.yaml               # Solver worker URL
├── src/
│   ├── main.rs                           # Entry point, dispatches to run, probe, or selftest
│   ├── api/
│   │   ├── endpoint.rs                   # API paths, base URL, site key, and referer helpers
│   │   ├── mod.rs
│   │   └── protocol.rs                   # Handshake, envelope, key derivation, and server signature verification
│   ├── app/
│   │   ├── bootstrap.rs                  # Project root, banner, and display text sanitizer
│   │   └── mod.rs
│   ├── domain/
│   │   ├── account.rs                    # Account and proxy models
│   │   ├── mining.rs                     # Mining state, streak, balance, and ready session parser
│   │   ├── mod.rs
│   │   ├── session.rs                    # Passkey cache and vault encrypt/decrypt
│   │   └── task.rs                       # Campaign and reward list parser
│   ├── modes/
│   │   ├── mod.rs
│   │   ├── probe.rs                      # Checks config, solver, and exit country without running a cycle
│   │   ├── run.rs                        # Loads accounts and hands off to the cycle runner
│   │   └── selftest.rs                   # Offline checks for vault, passkey, masking, and config defaults
│   ├── net/
│   │   ├── client.rs                     # HTTP client with optional proxy, cookie store, and timeout
│   │   ├── mod.rs
│   │   ├── proxy.rs                      # Proxy string parser and normalizer
│   │   └── solver.rs                     # Solver session, Turnstile task submission, and token polling
│   ├── ops/
│   │   ├── mod.rs
│   │   ├── runner.rs                     # Main cycle loop across all accounts with countdown between cycles
│   │   ├── auth/
│   │   │   ├── login.rs                  # Login with email, password, and Turnstile token
│   │   │   ├── mod.rs
│   │   │   └── passkey.rs                # Passkey enrollment with OTP and cached assertion signing
│   │   ├── mining/
│   │   │   ├── claim.rs                  # Ad flow and session claim submission
│   │   │   ├── mod.rs
│   │   │   ├── start.rs                  # Starts a mining block when state is idle
│   │   │   ├── status.rs                 # Fetches current mining state and balance
│   │   │   └── streak.rs                 # Daily streak request
│   │   ├── referral/
│   │   │   ├── mod.rs
│   │   │   └── wallet.rs                 # Wallet balance and address report
│   │   └── task/
│   │       ├── ads.rs                    # Ad flow: start, select, wait, complete, return token
│   │       ├── mod.rs
│   │       └── rewards.rs                # Claims airdrops, giveaways, and ad-gated sponsored tasks
│   └── support/
│       ├── banner.rs                     # ASCII banner with timestamp and community link
│       ├── error.rs                      # Server error code to human-readable message mapper
│       ├── mod.rs
│       ├── cfg/
│       │   ├── loader.rs                 # Loads config.json, data.txt, proxy.txt, and solver.yaml
│       │   ├── mod.rs
│       │   └── schema.rs                 # Default config and sleep_seconds extractor
│       ├── consts/
│       │   ├── app.rs                    # File path constants and passkey retry count
│       │   ├── mod.rs
│       │   └── net.rs                    # Protocol version, solver paths, poll settings, and pinned server key
│       └── log/
│           ├── format.rs                 # Green, yellow, and red log line helpers
│           ├── init.rs                   # Terminal output lock, ANSI colors, and live countdown
│           └── mod.rs
└── utils/
    └── banner.rb                 # Banner display on startup
```

---

## Disclaimer

This tool is built for educational and technical exploration purposes. Use it wisely and at your own responsibility.

---

<div align="center">
<img width="100%" alt="footer" src="https://capsule-render.vercel.app/api?type=waving&height=120&section=footer"/>
</div>