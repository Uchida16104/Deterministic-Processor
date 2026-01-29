# Deterministic Data Processing Platform

A permanently usable, multi-user beneficial application that guarantees deterministic behavior and zero runtime errors by design.

## Architecture Overview

This application consists of two main components designed for complete determinism and reliability:

**Frontend (Vercel + SvelteKit)**: Stateless UI for data upload, transformation configuration, and results visualization with no client-side persistence or mutable global state.

**Backend (Render + Rust)**: Fully stateless request-response architecture with pure-function-based processing, no external database, and embedded C# processing layer for mathematical transformations and rule evaluation.

## Technology Stack

The frontend leverages SvelteKit with TypeScript, HTMX for dynamic interactions with push-url navigation, Alpine.js for reactive UI components, and TailwindCSS for styling. The backend uses Rust with Actix-web for the HTTP server, integrating embedded C# via compilation and FFI for deterministic data processing logic.

## Design Principles

The application maintains no mutable global state and executes no time-based or environment-dependent logic. All operations are idempotent with no network calls during processing, ensuring deterministic outputs for identical inputs. Strong compile-time guarantees prevent runtime errors through Rust's type system and C# static analysis.

## Deployment Configuration

### Backend Deployment on Render (Rust)

**Service Type**: Web Service

**Region**: Oregon (US West) or Frankfurt (EU Central) recommended for optimal latency

**Root Directory**: `backend`

**Build Command**: `cargo build --release`

**Start Command**: `./target/release/deterministic-processor`

**Environment Variables**:
```
RUST_LOG=info
PORT=8080
ALLOWED_ORIGINS=https://your-frontend-domain.vercel.app
MAX_PAYLOAD_SIZE=10485760
```

The Rust application automatically binds to the PORT environment variable provided by Render. Ensure ALLOWED_ORIGINS matches your exact Vercel deployment URL for proper CORS handling.

### Frontend Deployment on Vercel (SvelteKit)

**Framework Preset**: SvelteKit

**Root Directory**: `frontend`

**Build Command**: `npm run build`

**Output Directory**: `.svelte-kit` (auto-detected)

**Install Command**: `npm install`

**Environment Variables**:
```
PUBLIC_API_URL=https://your-backend.onrender.com
```

Replace the PUBLIC_API_URL value with your actual Render backend URL after deployment. This variable is prefixed with PUBLIC_ to make it accessible in the browser.

## Local Development Setup

### Prerequisites

You must have Node.js version 18 or higher, Rust version 1.70 or higher, and the .NET SDK version 7.0 or higher installed on your system. Additionally, install cargo-watch for development hot-reloading with the command `cargo install cargo-watch`.

### Backend Local Setup

Navigate to the backend directory and build the C# processing modules with `dotnet build csharp/DataProcessor.csproj`. Then build the Rust application using `cargo build`. For development mode with hot reloading, run `cargo watch -x run`. The backend will start on `http://localhost:8080` by default.

Set the following environment variables for local development:
```bash
export RUST_LOG=debug
export PORT=8080
export ALLOWED_ORIGINS=http://localhost:5173
export MAX_PAYLOAD_SIZE=10485760
```

### Frontend Local Setup

Navigate to the frontend directory and install all dependencies with `npm install`. Create a `.env` file containing `PUBLIC_API_URL=http://localhost:8080`. Start the development server using `npm run dev`. The frontend will be accessible at `http://localhost:5173`.

### Testing the Complete Stack Locally

Start the backend server first, then start the frontend development server. Open your browser to `http://localhost:5173`. Upload a CSV or JSON file, select a transformation rule, and submit for processing. The deterministic results will be displayed immediately.

## Features

The platform supports CSV and JSON data uploads with configurable transformation rules including normalization, aggregation, filtering, and mathematical operations. All processing is stateless and deterministic with identical inputs always producing identical outputs. Results can be visualized in-app or downloaded as files.

## Security & Reliability

The application employs strict input validation with type-safe processing, CORS configuration for cross-origin security, and zero external dependencies during computation. No database eliminates an entire class of failure modes, while compile-time guarantees prevent null references and type errors.

## Extending the Platform

To add new C# processing functions, create methods in `backend/csharp/DataProcessor.cs` following the pure function pattern. Add corresponding Rust FFI bindings in `backend/src/processing/csharp_bridge.rs` and register new transformation types in the pipeline module. All additions must maintain deterministic behavior and statelessness.

## Troubleshooting

If CORS errors occur, verify that ALLOWED_ORIGINS in the backend matches your frontend URL exactly. For C# compilation errors, ensure the .NET SDK is properly installed and accessible. If Render build fails, check that the Build Command and Start Command match the paths specified above.

## License

MIT License - Free for commercial and non-commercial use.

## Support

For issues or questions, please open an issue on the repository. This platform is designed for long-term stability and should require minimal maintenance once deployed.
