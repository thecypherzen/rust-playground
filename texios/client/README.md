# Texios Client

A modern web application for text analysis and visualization, built with React, TypeScript, and WebAssembly.

## Features

- **Text Analysis**: Analyze text files using a high-performance Rust-based WebAssembly engine
- **Word Frequency Analysis**: Generate detailed word frequency statistics
- **Interactive Visualization**: View word frequency data through interactive bar charts
- **File Support**: Process various text file formats
- **Real-time Statistics**: Get instant character count and word count metrics

## Technical Stack

- **Frontend**: React + TypeScript + Vite
- **Core Analysis Engine**: Rust + WebAssembly
- **Data Visualization**: Recharts
- **Styling**: Tailwind CSS + shadcn/ui

## Architecture

The application consists of two main parts:

1. **Rust Analysis Engine**:

   - Uses regex for word pattern matching
   - Processes text to generate word frequencies and positions
   - Compiles to WebAssembly for browser execution
   - Provides statistics like word count and character count

2. **React Frontend**:
   - File selection and handling interface
   - Real-time analysis results display
   - Interactive data visualization
   - Custom hooks for file analysis state management

## Getting Started

1. Install dependencies:

   ```bash
   npm install
   ```

2. Start the development server:

   ```bash
   npm run dev
   ```

3. Build for production:
   ```bash
   npm run build
   ```

## Project Structure

```
src/
├── components/        # React components
│   ├── FileSelect    # File input handling
│   └── WordFrequencyPlot  # Data visualization
├── hooks/            # Custom React hooks
│   └── UseFileAnalysis    # File analysis logic
├── pkg/              # WebAssembly modules
└── lib/             # Utility functions
```

## Future Enhancements

- Add support for more text analysis metrics, eg:
  - skip-to-word in file
  - user-based search term support
  - etc.
- Implement advanced visualization options including additional charts
- Add export functionality for analysis results
- Support for batch file processing
- Add natural language processing features
- Cloud file storage
- Personalised interface
