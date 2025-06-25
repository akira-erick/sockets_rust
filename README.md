# Sockets in Rust

A simple implementation of socket programming in rust.

## Tech Stack
- **Server:** Rust
- **Client** Rust
- **Containerization:** Docker

## Getting Started

### Prerequisites
- Docker
- Docker Compose

### Running the Application

   Clone this repository to your local machine and run it using the following command:

   ```bash
   docker compose up --no-attach client --no-attach server
   ```

   Open anoter teminal and use this command:

   ```bash
   docker attach sockets-client-1
   ```

   Now you are inside the client, to end type exit.

   If you want to see the server logs, go to another terminal and use this command:

   ```bash
   docker attach sockets-server-1
   ```
   To end the process just Ctrl+C in the terminal where you were running the containers.