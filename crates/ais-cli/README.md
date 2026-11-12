# AIS CLI (Autonomous Iteration System)

The open-source autonomous code quality, risk triage, universal baseline ratchet, and self-healing CLI.

## Installation
```bash
cargo install --path crates/ais-cli
```

## Commands
- `ais scan`: Run fast incremental quality and architectural scan.
- `ais ratchet`: Universal baseline ratchet preventing technical debt regressions across metrics.
- `ais risk`: Contextual Risk Scoring (CRS) & RICE debt triage.
- `ais heal`: Closed-loop autonomous self-healing agent with multi-model routing (Gemini 3.1 Pro / 3.8).
- `ais telemetry`: Ingest production crash logs (GCP Cloud Run, Sentry) directly into living backlogs.
- `ais metrics`: Measure Cyclomatic Complexity (CCN) and Maintainability Index (MI).

## Configuration (`ais.yaml`)
Define custom rules, threshold boundaries, and model options in your repo root:
```yaml
version: 1.0.0
models:
  default: gemini-3.1-pro
  deep_reasoning: gemini-3.8
ratchet:
  baseline_file: audit/raw/ratchet_baseline.json
  fail_on_regression: true
```
