# Custom Portfolio Site

[![Website](https://img.shields.io/badge/Website-adam.bocktank.com-purple)](https://adam.bocktank.com)

This is a custom-built portfolio site using Rust (with Actix Web and Diesel) for the backend, and React (with Vite, React Router, and Tailwind CSS) for the frontend. Its purpose is to showcase a collection of projects, each with its own chapters and entries to explain or document the project. This site is both a technical showcase and a place where I document my work and ideas.

The code is public, so theoretically, anyone could build this site for themselves and have their own portfolio site.

## Features

- **Project Management:** Create, edit, and delete projects to showcase your work.
- **Chapter Organization:** Organize project content into chapters for better structure.
- **Detailed Entries:** Add detailed entries with descriptions and images to document your projects.
- **User Authentication:** Secure user authentication system to manage your portfolio.
- **Dockerized Environment:** Easy setup and deployment with Docker.

## Software Stack

- **Backend:** Rust, Actix Web, Diesel
- **Frontend:** React, Vite, React Router, Tailwind CSS
- **Database:** PostgreSQL
- **Containerization:** Docker

## Getting Started

### Prerequisites

- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)

### Installation

1. **Setup:**
   - Configure the docker compose file volumes for your environment.
2. **Docker Setup:**
   - Create a `jwt_secret.txt` file in the root directory with a secret key.
   - From the root directory, run:
     ```bash
     docker-compose up -d
     ```

## Project Structure

```
.
├── portfolio_back
│   ├── src
│   └── Cargo.toml
├── portfolio_front
│   ├── src
│   └── package.json
├── portfolio_db
│   └── dockerfile
└── docker-compose.yml
```

## Usage

- The application will be available at `http://localhost:8086`.

## License

This project is licensed under the MIT License.