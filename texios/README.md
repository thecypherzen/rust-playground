# Texios

A high-performance text analysis tool combining Rust's speed with web accessibility through WebAssembly.

## Overview

Texios is a modern text analysis platform that combines the performance of Rust with the accessibility of web technologies. It processes text documents to provide detailed analysis and interactive visualizations, making it useful for content analysis, document processing, and text mining tasks.

## Project Structure

```
texios/
├── analyser/        # Rust-based text analysis engine
│   └── src/
│       └── lib.rs   # Core analysis implementation
└── client/          # Web frontend
    └── src/         # React + TypeScript implementation
```

## Components

### Analyzer (Rust)

The core analysis engine written in Rust provides:

- Regex-based word pattern matching
- Word frequency calculation
- Word position tracking
- Document statistics (word count, character count)
- WebAssembly compilation for browser execution

Key features of the analyzer:

- Efficient text processing using Rust's performance capabilities
- Custom regex patterns for word identification
- Comprehensive word statistics and positioning
- WebAssembly-compatible architecture

### Client (React + TypeScript)

The web interface provides:

- Interactive file selection and processing
- Real-time analysis results
- Data visualization using charts
- Modern, responsive UI

## Technology Stack

- **Backend Engine**: Rust
- **WebAssembly Bridge**: wasm-bindgen
- **Frontend**: React + TypeScript
- **Build Tool**: Vite
- **UI Framework**: shadcn/ui + Tailwind CSS
- **Visualization**: Recharts

## Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/texios.git
   cd texios
   ```

2. Build the Rust analyzer:

   ```bash
   cd analyser
   wasm-pack build
   ```

3. Set up the client:
   ```bash
   cd ../client
   npm install
   npm run dev
   ```

## Development

### Building the Analyzer

The Rust analyzer needs to be compiled to WebAssembly:

```bash
cd analyser
wasm-pack build
```

### Running the Client

The client development server can be started with:

```bash
cd client
npm run dev
```

## Future Roadmap

1. Analysis Features

   - Support for more text formats
   - Advanced statistical metrics
   - Natural language processing capabilities
   - Pattern recognition and text classification

2. Performance Improvements

   - Parallel processing for large documents
   - Streaming analysis for real-time processing
   - Memory optimization for large files

3. User Interface
   - Additional visualization options
   - Batch processing interface
   - Analysis result export features
   - Custom analysis configuration

## Contributing

Contributions are welcome! Please feel free to submit pull requests, create issues, or suggest improvements.
