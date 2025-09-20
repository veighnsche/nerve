# Nerve Program

> Fill the CAPITALIZED fields. Everything else has defaults.  
> Keep it brief—sentences or bullet lists are fine.

## 1) Project

- Name: **YOUR_PROJECT_NAME**
- Language / Stack: **RUST | PYTHON | TS | GO | …**
- Brief: **ONE-LINE outcome (what this app should do)**
- Repo context (optional): **monorepo|service|lib** (default: lib)

## 2) Source of truth (docs you want the AI to follow)

- Doc roots: `.specs/**, docs/**`  <!-- default; keep or edit -->
- Issue roots (optional): `tickets/**.md`

## 3) MVP scope (what “done” means)

- Features (bullets):  
  - **core gameplay loop**  
  - **save/load**  
  - **CLI UI**  
- Non-goals (optional):  
  - **multiplayer**  

## 4) Style & constraints (how to do it)

- Coding style: **idiomatic + tested**
- Test strategy: **unit + property tests**
- Libraries you prefer/ban (optional): **prefer X; avoid Y**
- FS write areas: `src/**, tests/**, .specs/**`  <!-- default safe globs -->

## 5) Guardrails (budgets & loop bounds)

- Quick-fix attempts: **6**       <!-- short loop ceiling -->
- Tokens budget: **200k**         <!-- total ceiling for build -->
- Time budget: **30m**            <!-- wall clock ceiling -->
- Full test timeout: **10m**

## 6) Maintenance (after v1 ships)

- Cadence: **5m**                 <!-- big loop period -->
- Hourly drift threshold: **0.28**<!-- spec↔code semantic drift gate -->
- Daily token cap: **300k**

## 7) Extras (optional)

- Entry command (if a CLI app): **ttt --play**
- License: **GPL-3.0-or-later** (default)
