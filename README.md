# Rust WASM Web Application

A modern web application built with Rust and WebAssembly, featuring a component-based architecture, state management, routing, and Protocol Buffers for content delivery.

## Features

- 🦀 Built with Rust and WebAssembly
- 🎨 Theme switching (light/dark mode)
- 🔄 State management
- 📱 Client-side routing
- 📦 Protocol Buffers for efficient content delivery
- ✨ Component-based architecture
- 📝 Todo list functionality
- 🔢 Counter example
- 🎯 Performance monitoring

## Prerequisites

- Rust (latest stable version)
- wasm-pack
- Node.js (for serving static files)
- Protocol Buffers compiler (protoc)

## Project Structure 

.
├── src/
│ ├── components/ # Reusable UI components
│ ├── state/ # Application state management
│ ├── router/ # Client-side routing
│ ├── content_loader/ # Content loading with Protocol Buffers
│ ├── theme/ # Theme management
│ ├── utils/ # Utility functions
│ ├── rsx/ # RSX macros for UI composition
│ ├── performance/ # Performance monitoring
│ └── lib.rs # Main application entry
├── static/
│ ├── index.html
│ ├── styles.css
│ └── content.pb # Protocol Buffer encoded content
├── build.rs # Build script for Protocol Buffers
└── Cargo.toml


## Getting Started

1. Clone the repository:

bash
git clone https://github.com/yourusername/your-project-name.git
cd your-project-name


2. Install dependencies:

bash
cargo install wasm-pack
cargo install basic-http-server


3. Build the project:

wasm-pack build --target web


4. Serve the application:

basic-http-server . or you can use the liveserver from vs code to load index.html which will serve the application.


5. Open your browser and navigate to `http://localhost:4000`

## Development

### Building Protocol Buffers

The project uses Protocol Buffers for content delivery. To update the content:

1. Modify the content definition in `src/content.proto`
2. Generate new content:


### Adding New Components

1. Create a new file in the `components` directory
2. Use the `rsx!` macro for component definition
3. Export the component in `components/mod.rs`

### State Management

The application uses a centralized state management system:

rust
// Dispatch an action
dispatch(Action::IncrementCounter);
// Access state
STATE.with(|state| {
let state = state.borrow();
// Use state here
});



## Features in Detail

### Routing
The application supports client-side routing with the following routes:
- `/` - Home page
- `/articles` - Articles listing
- `/about` - About page
- `/*` - 404 Not Found

### Theme Support
- Light and dark mode support
- Theme persistence across sessions
- Smooth theme transitions

### Performance Monitoring
Built-in performance monitoring for:
- Route rendering
- State updates
- Content loading

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Rust](https://www.rust-lang.org/)
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
