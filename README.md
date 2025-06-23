# Dioxus Desktop Starter Kit

A professional, production-ready starter kit (template) for building desktop applications with [Dioxus](https://dioxuslabs.com/) (Rust). This template features a custom window, multi-language localization, dark/light theme support, and modern styling with [Tailwind CSS](https://tailwindcss.com/).

## 🚀 How to Use

Click the green **"Use this template"** button on GitHub to create your own project based on this starter.

## 🛠 Rename the Project

After using this template, update the project name in `Cargo.toml`:

```toml
[package]
name = "your-app-name"
```

---

## 📺 Demo Video

[![Dioxus Desktop Starter Kit Demo](https://img.youtube.com/vi/y5Qv9c9FRzc/0.jpg)](https://www.youtube.com/watch?v=y5Qv9c9FRzc)

Watch the demo: [https://www.youtube.com/watch?v=y5Qv9c9FRzc](https://www.youtube.com/watch?v=y5Qv9c9FRzc)

---

## ✨ Features

- ⚡ **Dioxus Desktop**: Fast, reactive UI in Rust
- 🪟 **Custom Window**: Unique window chrome and controls
- 🌐 **Localization**: Multi-language support (see `src/localization/`)
- 🌗 **Dark/Light Themes**: Seamless theme switching
- 🎨 **Tailwind CSS**: Utility-first styling
- 🔄 **Live CSS Reload**: `npm run dev` for Tailwind watch mode
- 🧩 Modular, scalable architecture

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (for Tailwind CSS)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/safwa1/dioxus_starter.git
   cd dioxus_starter
   ```
2. **Install Rust dependencies:**
   ```bash
   cargo build
   ```
3. **Install Node.js dependencies (for Tailwind):**
   ```bash
   npm install
   ```

### Development Workflow

- **Start Tailwind in watch mode:**
  ```bash
  npm run dev
  ```
  This will watch and rebuild your CSS on changes.

- **Run the Dioxus desktop app:**
  ```bash
  dx serve
  ```
  Runs the app in development mode with hot reload.

- **Build for release:**
  ```bash
  dx build --release
  ```
  Builds the app for production.

- **Bundle for distribution:**
  ```bash
  dx bundle --release
  ```
  Bundles the app for release/distribution.

---

## 🗂️ Project Structure

```
├── src/                # Rust source code
│   ├── app/            # App logic, components, router
│   ├── localization/   # Language files (multi-language)
│   ├── models/         # Data models
│   ├── services/       # Service layer
│   ├── state/          # State management
│   └── utils/          # Utilities and macros
├── assets/             # Static assets (CSS, icons)
├── icons/              # App icons for various platforms
├── tailwind.config.js  # Tailwind CSS config
├── tailwind.css        # Tailwind CSS entry
├── Cargo.toml          # Rust project manifest
├── package.json        # Node.js dependencies/scripts
└── README.md           # Project documentation
```

---

## 🌐 Localization

- Add or edit language files in `src/localization/` to support more languages.
- Language switching is handled in-app.

---

## 🌓 Themes

- Supports both dark and light themes.
- Theme switching is available in the UI and managed in app state.

---

## 🛠️ Development

- **Hot Reload:** Use `dx serve` or your preferred tool for live reload during development.
- **Linting:**
  ```bash
  cargo clippy
  ```
- **Formatting:**
  ```bash
  cargo fmt
  ```

---

## 🤝 Contributing

Contributions are welcome! Please open issues or submit pull requests for improvements or bug fixes.

---

## 📄 License

This project is licensed under the MIT License.
