/// THIS WOULD BE GENERATED FILE #001

# Specification: WayLaunch

Status: Draft  
Owner: @waylaunch-maintainers  
Date: 2025-09-19  
Version: v0.1  

---

## 0) Motivation & Goals

- Provide a CLI for Arch Linux that ensures apps run under Wayland whenever possible.  
- Enforce Wayland launch environments by scanning installed applications and applying safe overrides.  
- MUST give users visibility into whether applications are compliant, fixable, or X11-only.  

## 1) Non-Goals

- MUST NOT modify system-wide `.desktop` files in `/usr/share/applications`.  
- MUST NOT manage Wayland compositors (sway, hyprland, etc.).  
- SHOULD NOT introduce heavyweight GUI or external tooling dependencies.  

## 2) Current State (Brief)

- No existing tooling reliably enforces Wayland launch across arbitrary installed applications.  
- Users must manually edit `.desktop` entries or pass flags; this is error-prone.  
- WayLaunch aims to automate this process with safe overrides in `~/.local/share/applications`.  

## 3) Responsibilities

- **Primary Responsibility:** Detect Wayland compatibility and enforce correct environment variables and flags.  
- **Secondary Responsibilities:**  
  - Provide clear CLI reports of compliance status.  
  - Offer safe revert functionality to undo overrides.  
- **Out-of-Scope:** Managing compositor settings or editing root-owned files.  

## 4) Requirements

- The CLI MUST scan `.desktop` entries from both `/usr/share/applications` and `~/.local/share/applications`.  
- The system MUST detect Wayland capability for toolkits including GTK, Qt, SDL, Electron/Chromium, and Java SWT.  
- The CLI MUST generate safe launch wrappers or user-local `.desktop` overrides.  
- The system MUST verify execution with a dry-run and `$WAYLAND_DISPLAY`.  
- The CLI SHOULD provide subcommands: `scan`, `enforce`, `report`, `revert`, `doctor`.  
- A config file MUST be read from `~/.config/waylaunch/config.toml`.  

## 5) Interfaces & Contracts

- **CLI Surface:**  
  - `waylaunch scan` — detect applications and classify.  
  - `waylaunch enforce` — apply Wayland overrides.  
  - `waylaunch report` — show compliance summary.  
  - `waylaunch revert` — restore original behavior.  
  - `waylaunch doctor` — troubleshoot problems.  

- **Config Surface:**  
  - TOML-based config in `~/.config/waylaunch/config.toml`.  

- **Errors:**  
  - MUST handle missing `.desktop` entries gracefully.  
  - MUST emit clear errors if enforcement fails.  

## 6) Observability

- CLI output MUST include compliance categories: `compliant`, `can-fix`, `X11-only`, `unknown`.  
- The system SHOULD provide per-app detailed reports.  
- Logs MAY be extended with structured JSON output in future versions.  

## 7) Security & Policy

- MUST NOT modify root-owned files.  
- MUST sandbox overrides in user-local directories only.  
- SHOULD minimize privilege escalation risk by avoiding `sudo` paths.  

## 8) Testing & Proofs

- Unit tests MUST cover `.desktop` parsing.  
- Integration tests MUST simulate user-local overrides.  
- Property tests SHOULD validate toolkit detection logic.  
- Golden fixtures MAY be used for representative `.desktop` files.  

## 9) Migration & Compatibility

- WayLaunch MUST interoperate with existing `.desktop` entries without breaking them.  
- Revert functionality MUST restore original state if users opt out.  
- Breaking changes MUST be documented in changelogs.  

## 10) Open Questions

- Should JSON output mode be supported in v1 or deferred?  
- How to extend toolkit detection beyond the initial set (GTK, Qt, SDL, Electron/Chromium, SWT)?  
- Should per-app overrides be tracked in a central manifest for better revert support?  

---
