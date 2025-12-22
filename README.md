Pro PainT
=========

Your task is to create a graphical image editor. The editor will support multiple layers. In each layer you can use different tools (pencil, pen, etc.). Create a custom format to which you will save and load the files. The program will allow you to export from your chosen commonly used image format.

A graphical image editor developed as a project for the PV281 course during fall 2025 semester.

Use-case diagram
----------------
![Use-case diagram](./docs/use-case.png)

Development setup (Leptos + Tauri)
----------------------------------

This project uses a Rust + Web stack:

- **Frontend**: [Leptos](https://github.com/leptos-rs/leptos) (WASM, built with `trunk`) in `pro-pain-t-app/`
- **Desktop shell**: [Tauri](https://tauri.app/) in `src-tauri/`

### Prerequisites

Install the following tools (once):

- Rust toolchain (via `rustup`)
- Trunk (for building/serving the Leptos frontend)
- Tauri CLI

On macOS, you can install them with:

- `cargo install trunk`
- `rustup target add wasm32-unknown-unknown`
- `cargo install tauri-cli`

### Development (run app in dev mode)

From the project root:

1. Change into the Tauri directory:
	- `cd src-tauri`
2. Start the dev environment:
	- `cargo tauri dev`

What happens:

- Tauri runs the configured `beforeDevCommand`, which starts `trunk serve` in `pro-pain-t-app/` on port `1420`.
- Tauri opens a desktop window and loads the frontend from `http://localhost:1420` (hot reload).

### Building a release desktop app

From the project root:

1. Change into the Tauri directory:
	- `cd src-tauri`
2. Build the application:
	- `cargo tauri build`

This will:

- run `trunk build` in `pro-pain-t-app/` and output static files into `pro-pain-t-app/dist/`,
- bundle those assets into a Tauri desktop application (e.g. `.app` on macOS).

### Frontend-only development (optional)

If you want to develop only the Leptos UI in a browser, without Tauri:

1. From the project root:
	- `cd pro-pain-t-app`
2. Run the dev server:
	- `trunk serve --open`

This will open the Leptos app in your default browser.
