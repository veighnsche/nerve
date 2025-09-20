# Nerve Program

> Fill the CAPITALIZED fields if you want to tweak defaults. This file alone is enough to build the MVP pipeline.

## 1) Project

- Name: WayLaunch
- Language / Stack: RUST
- Brief: A CLI for Arch that scans installed apps and ensures any app that CAN run under Wayland is actually launched with Wayland (env/flags/wrappers).
- Repo context: cli

## 2) Source of truth (docs you want the AI to follow)

- Doc roots: .specs/**, docs/**
- Issue roots (optional): tickets/**.md

## 3) MVP scope (what “done” means)

- Features:
  - scan .desktop entries in `/usr/share/applications` and `~/.local/share/applications`
  - detect Wayland capability (rules for toolkits: GTK, Qt, SDL, Electron/Chromium, Java SWT)
  - generate safe launch wrappers or user-local `.desktop` overrides to enforce Wayland env/flags
  - verify execution path (dry-run + real launch test with `$WAYLAND_DISPLAY`)
  - per-app report: compliant / can-fix / X11-only / unknown
  - CLI subcommands: `scan`, `enforce`, `report`, `revert`, `doctor`
  - config file in `~/.config/waylaunch/config.toml`
- Non-goals:
  - system-wide package edits
  - modifying global `.desktop` files (root-owned)
  - Wayland compositors management (sway/hyprland etc.)

## 4) Style & constraints (how to do it)

- Coding style: idiomatic + tested
- Test strategy: unit + integration (mock .desktop dirs), property tests for parser
- Libraries:
  - prefer `clap` for CLI, `toml` for config, `serde` for IO, small `.desktop` parser (write or lightweight crate)
  - avoid heavyweight GUI/tooling deps
- FS write areas: src/**, tests/**, .specs/**, scripts/**, packaging/**

## 5) Guardrails (budgets & loop bounds)

- Quick-fix attempts: 6
- Tokens budget: 200k
- Time budget: 30m
- Full test timeout: 10m

## 6) Maintenance (after v1 ships)

- Cadence: 1h
- Hourly drift threshold: 0.28
- Daily token cap: 300k

## 7) Extras (optional)

- Entry command: waylaunch scan --enforce
- License: GPL-3.0-or-later
