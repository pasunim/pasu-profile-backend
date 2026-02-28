# Pasu Profile Backend

A high-performance RESTful API backend for [Pasu Nimsuwan's](https://github.com/pasunim) personal profile website, built with **Rust** using the **Axum** web framework.

## ✨ Features

- **Blazing Fast** — Built with Rust and Axum for maximum performance
- **RESTful API** — Clean, well-structured API endpoints
- **PostgreSQL** — Reliable data persistence with SQLx
- **Swagger UI** — Interactive API documentation at `/swagger-ui`
- **Docker Ready** — Multi-stage Dockerfile for optimized container builds
- **CORS Enabled** — Cross-origin resource sharing out of the box
- **Cloudinary Integration** — Image upload support via Cloudinary
- **Admin CRUD** — Full admin endpoints for content management

## 🛠 Tech Stack

| Technology | Purpose |
|---|---|
| [Rust](https://www.rust-lang.org/) | Programming language |
| [Axum](https://github.com/tokio-rs/axum) | Web framework |
| [Tokio](https://tokio.rs/) | Async runtime |
| [SQLx](https://github.com/launchbadge/sqlx) | PostgreSQL driver |
| [Utoipa](https://github.com/juhaku/utoipa) | OpenAPI / Swagger UI |
| [Tower-HTTP](https://github.com/tower-rs/tower-http) | Middleware (CORS, tracing) |
| [Docker](https://www.docker.com/) | Containerization |

## 📁 Project Structure

```
pasu-profile-backend/
├── src/
│   ├── main.rs          # Application entry point & route definitions
│   ├── db.rs            # Database connection pool
│   ├── error.rs         # Custom error types
│   ├── models.rs        # Data models (SQLx + Serde)
│   └── handlers/        # Route handlers
│       ├── mod.rs
│       ├── about.rs     # About section
│       ├── skills.rs    # Skills CRUD
│       ├── experience.rs# Experience timeline CRUD
│       ├── projects.rs  # Projects CRUD
│       ├── contact.rs   # Contact info, socials & messages
│       ├── blog.rs      # Blog posts, categories & tags
│       ├── admin.rs     # Authentication
│       └── upload.rs    # Image upload (Cloudinary)
├── Cargo.toml           # Dependencies
├── Dockerfile           # Multi-stage Docker build
├── .env.example         # Environment variable template
└── .dockerignore
```

## 📡 API Endpoints

### Public

| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/about` | Get profile bio |
| `GET` | `/api/skills` | List all skills |
| `GET` | `/api/experience` | Get experience timeline |
| `GET` | `/api/projects` | List all projects |
| `GET` | `/api/contact` | Get contact information |
| `POST` | `/api/contact` | Submit a contact message |
| `GET` | `/api/contact/socials` | Get social media links |
| `GET` | `/api/blog/posts` | List blog posts |
| `GET` | `/api/blog/posts/:slug` | Get a blog post by slug |
| `GET` | `/api/blog/categories` | List blog categories |
| `GET` | `/api/blog/tags` | List blog tags |

### Admin

| Method | Endpoint | Description |
|---|---|---|
| `POST` | `/api/admin/login` | Admin authentication |
| `POST` | `/api/about` | Update about info |
| `POST` | `/api/skills` | Create a skill |
| `PUT` | `/api/skills/:id` | Update a skill |
| `DELETE` | `/api/skills/:id` | Delete a skill |
| `POST` | `/api/experience/timeline` | Create timeline entry |
| `PUT` | `/api/experience/timeline/:id` | Update timeline entry |
| `DELETE` | `/api/experience/timeline/:id` | Delete timeline entry |
| `POST` | `/api/projects` | Create a project |
| `PUT` | `/api/experience/projects/:id` | Update a project |
| `DELETE` | `/api/experience/projects/:id` | Delete a project |
| `POST` | `/api/contact/info` | Update contact info |
| `POST` | `/api/contact/socials` | Create social link |
| `PUT` | `/api/contact/socials/:id` | Update social link |
| `DELETE` | `/api/contact/socials/:id` | Delete social link |
| `GET` | `/api/contact/messages` | List contact messages |
| `DELETE` | `/api/contact/messages` | Delete a contact message |
| `POST` | `/api/blog/posts` | Create blog post |
| `GET` | `/api/blog/admin/posts/:id` | Get post by ID (admin) |
| `PUT` | `/api/blog/admin/posts/:id` | Update blog post |
| `DELETE` | `/api/blog/admin/posts/:id` | Delete blog post |
| `POST` | `/api/blog/categories` | Create category |
| `PUT` | `/api/blog/categories/:id` | Update category |
| `DELETE` | `/api/blog/categories/:id` | Delete category |
| `POST` | `/api/blog/tags` | Create tag |
| `PUT` | `/api/blog/tags/:id` | Update tag |
| `DELETE` | `/api/blog/tags/:id` | Delete tag |
| `POST` | `/api/upload` | Upload image (Cloudinary) |

> 📖 Full interactive documentation available at **`/swagger-ui`** when the server is running.

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [PostgreSQL](https://www.postgresql.org/) 14+
- [Docker](https://www.docker.com/) (optional)

### Environment Variables

Copy the example file and fill in your values:

```bash
cp .env.example .env
```

| Variable | Description | Default |
|---|---|---|
| `DATABASE_URL` | PostgreSQL connection string | — |
| `PORT` | Server port | `8080` |
| `ADMIN_PASSWORD` | Admin login password | — |
| `CLOUDINARY_URL` | Cloudinary credentials URL | — |

### Run Locally

```bash
# Install dependencies & build
cargo build

# Run in development mode
cargo run

# Run in release mode
cargo run --release
```

The server will start at `http://localhost:8080`.

### Run with Docker

```bash
# Build the image
docker build -t pasu-backend .

# Run the container
docker run -d \
  --name pasu-backend \
  -p 8080:8080 \
  --env-file .env \
  pasu-backend
```

### Useful Docker Commands

```bash
# View logs
docker logs -f pasu-backend

# Stop the container
docker stop pasu-backend

# Remove the container
docker rm pasu-backend

# Rebuild & restart
docker stop pasu-backend && docker rm pasu-backend
docker build -t pasu-backend . && docker run -d --name pasu-backend -p 8080:8080 --env-file .env pasu-backend
```

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

## 👤 Author

**Pasu Nimsuwan**

- GitHub: [@pasunim](https://github.com/pasunim)
- LinkedIn: [pasunim](https://www.linkedin.com/in/pasunim/)
- Facebook: [pasu.nimsuwan](https://www.facebook.com/pasu.nimsuwan/)
