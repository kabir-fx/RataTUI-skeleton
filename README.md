# 📝 Ratatui Skeleton

A lightweight and elegant skeleton application built with [Ratatui](https://ratatui.rs/) showcasing fundamental concepts of building modern Terminal User Interfaces (TUIs) in Rust.

---

## ✨ Features

- **Asynchronous Event Handling**: Demonstrates how to handle terminal input events and background processes concurrently without blocking the main UI thread using `std::sync::mpsc` channels.
- **Custom Widget Implementation**: Showcases the implementation of the `ratatui::widgets::Widget` trait for a custom application state (`App`).
- **Reactive UI**: The UI reacts to background progress updates in real-time, displaying a progress bar that fills up smoothly over time.
- **Interactive Controls**: Users can interact with the application to modify the UI state dynamically.
- **Layout & Styling**: Uses Ratatui's layout engine to split the screen and applying bold colors, centered text, borders, and gauges.

## 🎮 Controls

- <kbd>q</kbd> : Quit the application
- <kbd>c</kbd> : Toggle the color of the progress bar (between `Green` and `Light Yellow`)

## 🚀 Getting Started

### Prerequisites

You need Rust and Cargo installed on your system. If you don't have them, you can install them from [rustup.rs](https://rustup.rs/).

### Running the Application

Clone the repository and run the application using Cargo:

```bash
cargo run
```

## 🏗️ Architecture Overview

The application is structured into three main concurrent components, connected via multi-producer, single-consumer (MPSC) channels:

1.  **Main Thread (UI Rendering)**:
    The central loop that continuously draws the UI based on the current state of the `App` struct. It listens for `CustomEvents` from the `mpsc::Receiver` and routes them to mutate the state or exit the application.
2.  **Input Thread**:
    A dedicated spawned thread that listens for `crossterm` terminal key events and sends them as `CustomEvents::Input(KeyEvent)` through the `mpsc::Sender`. This ensures that waiting for user input never blocks the rendering of background progress.

3.  **Background Process Thread**:
    A simulated task thread that periodically sends progress updates as `CustomEvents::Progress(f64)` through the `mpsc::Sender` to seamlessly render a non-blocking progress bar on the UI.

## 📦 Dependencies

This skeleton uses the standard TUI stack in the Rust ecosystem:

- [`ratatui`](https://crates.io/crates/ratatui): A Rust library for cooking up terminal user interfaces.
- [`crossterm`](https://crates.io/crates/crossterm): A cross-platform terminal library for reading input events and manipulating the terminal.
