# Rust MCP Server Weather

This repository contains a Rust implementation of a Model Context Protocol (MCP) server that provides weather forecasting capabilities. The server integrates with the Open-Meteo API to fetch real-time weather data based on geographic coordinates.

## Features

- **Weather Forecasting**: Get current weather conditions and hourly forecasts
- **Geographic Coordinates**: Query weather data using latitude and longitude
- **MCP Integration**: Compatible with Model Context Protocol for AI assistant integration
- **Real-time Data**: Fetches live weather data from Open-Meteo API
- **Structured Output**: Returns formatted weather information including temperature, humidity, and wind speed

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/mcp-weather.git
cd mcp-weather
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/mcp-weather`

## Usage

### Running the MCP Server

Start the weather MCP server:

```bash
cargo run
```

The server will start and listen for MCP protocol messages via standard input/output.

### MCP Tool: get_forecast

The server provides a `get_forecast` tool that accepts latitude and longitude coordinates and returns weather information.

**Parameters:**
- `latitude` (f32): Geographic latitude coordinate
- `longitude` (f32): Geographic longitude coordinate

**Example Response:**
```
Current Weather:
Temperature: 22.5°C
Wind Speed: 15.2 km/h

Hourly Forecast:
- 14:00: 23.1°C, 65% humidity, 12.8 km/h wind
- 15:00: 24.2°C, 62% humidity, 14.1 km/h wind
...
```

## API Integration

This MCP server integrates with the [Open-Meteo API](https://open-meteo.com/), a free weather API that provides:

- Current weather conditions
- Hourly forecasts
- Temperature, humidity, and wind speed data
- No API key required

## Development

### Project Structure

```
src/
├── main.rs          # Application entry point
├── service.rs       # MCP service implementation
├── model.rs         # Data structures and API models
├── util.rs          # Utility functions
└── error.rs         # Error handling
```

### Dependencies

- `rmcp`: Rust MCP SDK for server implementation
- `reqwest`: HTTP client for API requests
- `serde`: Serialization/deserialization
- `tokio`: Async runtime
- `tracing`: Logging and diagnostics

### Adding New Features

To extend the weather service:

1. Add new tools in `service.rs` using the `#[tool]` attribute
2. Define request/response models in `model.rs`
3. Implement utility functions in `util.rs`
4. Update error handling in `error.rs`

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Open-Meteo](https://open-meteo.com/) for providing free weather data
- [Model Context Protocol](https://modelcontextprotocol.io/) for the MCP specification
- [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk) for the Rust implementation