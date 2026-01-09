# Open Computing Club Recruitment CLI

A Rust-based terminal user interface application that generates recruitment campaign layouts with QR codes for the Open Computing Club. The application displays a visually appealing recruitment page directly in the terminal that can be screenshot for promotional materials.

## Features

- Beautiful TUI layout with club branding
- Dynamic QR code generation from URLs
- Terminal-based interface perfect for screenshots
- Command-line URL input support
- Interactive controls

## Installation

### Prerequisites

- Rust programming language (1.70 or later)
- Cargo package manager

### Setup

1. Clone or download this repository
2. Navigate to the project directory
3. Build the application:

```bash
cargo build --release
```

## Usage

### Basic Usage

```bash
# Run and be prompted for a URL
cargo run

# Run with custom URL directly
cargo run "https://your-link-here.com"

# Build and run separately
cargo build --release
./target/release/occ-recruitment-cli "https://your-link-here.com"
```

When you run without providing a URL, the application will prompt you to enter one. You can optionally provide a default suggestion that the user can accept or change.

### Theme

The application features a Zenless Zone Zero inspired "cassette futurism" aesthetic with:
- Heavy box-drawing characters (█, ║, ═, ╔, ╗)
- Asymmetric layout (70% main view, 30% sidebar)
- Industrial and gritty styling
- Noise patterns using unicode blocks (░, ▒, ▓)
- Inverted headers with sky blue background and charcoal text
- All uppercase text for the industrial vibe

### Controls

- `q` or `Q`: Quit the application
- `r` or `R`: Regenerate the QR code

## How It Works

The application creates a recruitment campaign layout in your terminal with:

1. A header with the Open Computing Club branding
2. A visually represented QR code that can be scanned
3. Recruitment information and messaging
4. The URL that the QR code points to
5. A footer with club information

The entire interface is designed to be visually appealing and suitable for taking screenshots that can be used as promotional materials.

## Customization

You can customize the application by modifying the source code:

- Update the ASCII art in the `create_ascii_art()` function
- Modify colors in the styling sections
- Change the layout in the `ui()` function
- Adjust the information displayed in the recruitment section

## Dependencies

- `ratatui`: Terminal user interface library
- `crossterm`: Cross-platform terminal manipulation
- `qrcode`: QR code generation
- `clap`: Command-line argument parsing

## Screenshot Tips

For the best promotional screenshots:

1. Use a terminal with good contrast (dark theme often works well)
2. Maximize your terminal window for better resolution
3. Adjust your terminal font size for optimal readability
4. Consider the aspect ratio that works best for your social media platform
5. The QR code will be scannable from the screenshot if displayed properly

## License

This project is licensed under the MIT License - see the LICENSE file for details.