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
│   ├── lib.rs           # Library exports for testing
│   ├── db.rs            # Database connection pool
│   ├── state.rs         # Centralized application state (AppState)
│   ├── cache.rs         # In-memory caching implementation (Moka)
│   ├── error.rs         # Custom error types
│   ├── models.rs        # Data models (SQLx + Serde)
│   ├── middleware.rs    # Authentication middleware
│   └── handlers/        # Route handlers
│       ├── mod.rs
│       ├── about.rs     # About section
│       ├── skills.rs    # Skills CRUD
│       ├── experience.rs# Experience timeline CRUD
│       ├── projects.rs  # Projects CRUD
│       ├── contact.rs   # Contact info, socials & messages
│       ├── blog.rs      # Blog posts, categories & tags
│       ├── admin.rs     # Authentication
│       ├── upload.rs    # Image upload (Cloudinary)
│       └── health.rs    # Health check endpoints
├── tests/               # Integration tests
│   ├── models_tests.rs  # Model serialization tests
│   ├── error_tests.rs   # Error handling tests
│   ├── handlers_tests.rs# Handler payload tests
│   └── handlers/        # Handler-specific tests
│       ├── mod.rs
│       ├── about_tests.rs
│       ├── contact_tests.rs
│       ├── upload_tests.rs
│       ├── experience_tests.rs
│       ├── projects_tests.rs
│       ├── skills_tests.rs
│       ├── blog_tests.rs
│       └── admin_tests.rs
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
| `GET` | `/health` | Health check |
| `GET` | `/health/ready` | Readiness check |

### Admin

All endpoints in this section require HTTP Basic authentication using
`ADMIN_PASSWORD`; without it they return `401`. The Swagger UI at
`/swagger-ui` is protected the same way.

The Next.js frontend never exposes this password to the browser: it verifies
the password once at login, issues a signed session cookie, and its
`/api/admin/proxy/*` route attaches the Basic header server-side.

```bash
curl -u admin:$ADMIN_PASSWORD -X POST http://localhost:8080/api/skills \
  -H 'Content-Type: application/json' \
  -d '{"icon":"code","title":"Rust","description":"..."}'
```

| Method | Endpoint | Description |
|---|---|---|
| `POST` | `/api/admin/login` | Verify the admin password (public; rate limited to 10 attempts per IP per 5 minutes) |
| `GET` | `/api/blog/admin/posts` | List all posts, drafts included |
| `POST` | `/api/about` | Update about info |
| `POST` | `/api/skills` | Create a skill |
| `PUT` | `/api/skills/:id` | Update a skill |
| `DELETE` | `/api/skills/:id` | Delete a skill |
| `POST` | `/api/experience/timeline` | Create timeline entry |
| `PUT` | `/api/experience/timeline/:id` | Update timeline entry |
| `DELETE` | `/api/experience/timeline/:id` | Delete timeline entry |
| `POST` | `/api/projects` | Create a project |
| `PUT` | `/api/projects/:id` | Update a project |
| `DELETE` | `/api/projects/:id` | Delete a project |
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
> 📄 OpenAPI JSON spec available at **`/api-docs/openapi.json`**.

## ⚡ Caching

The application uses **Moka** for high-performance in-memory caching to reduce database load and improve response times.

### Cached Endpoints

| Endpoint | Cache Key | TTL | Invalidation |
|---|---|---|---|
| `GET /api/projects` | `"projects"` | Default | POST/PUT/DELETE /api/projects |
| `GET /api/blog/categories` | `"categories"` | Default | POST/PUT/DELETE /api/blog/categories |
| `GET /api/blog/tags` | `"tags"` | Default | POST/PUT/DELETE /api/blog/tags |
| `GET /api/contact/socials` | `"socials"` | Default | POST/PUT/DELETE /api/contact/socials |
| `GET /api/about` | `"about"` | Default | POST /api/about |
| `GET /api/experience` | `"experience"` | Default | POST/PUT/DELETE /api/experience/timeline |
| `GET /api/skills` | `"skills"` | Default | POST/PUT/DELETE /api/skills |

### How Caching Works

1. **GET Request** → Check cache → Return cached data if available → Otherwise query DB and store in cache
2. **Write Operations** → Execute DB query → Invalidate related cache entries
3. **Next GET Request** → Cache miss → Query DB → Store fresh data in cache

Cache entries automatically expire after their TTL (Time-to-Live) elapses, ensuring data freshness.

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
| `ADMIN_PASSWORD` | Admin password. **Required** — while unset, every admin endpoint returns 401 | — |
| `ALLOWED_ORIGINS` | Comma-separated browser origins allowed by CORS | `http://localhost:3000` |
| `CLOUDINARY_URL` | Cloudinary credentials URL | — |
| `DB_MAX_CONNECTIONS` | Database pool size | `10` |
| `DB_ACQUIRE_TIMEOUT_SECS` | Seconds to wait for a pooled connection | `10` |

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

## 🧪 Testing

The project includes comprehensive unit and integration tests that run **without connecting to a real database**. Tests cover:

- Model serialization/deserialization
- Error handling and display messages
- Handler payload validation
- API response structures

### Run All Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test models_tests
cargo test error_tests
cargo test handlers_tests
```

### Code Quality

```bash
# Check for errors and warnings
cargo build

# Run linter for code quality
cargo clippy --all-targets --all-features

# Fix clippy warnings automatically
cargo clippy --fix --allow-dirty
```

### Test Coverage

| Test Module | Tests | Coverage |
|---|---|---|
| Health handler tests | 4 | Health check endpoints |
| Error handling tests | 13 | Error types & display |
| Model tests | 14 | Data model serialization |
| Handler payload tests | 74 | All handler payloads & validation |
| **Total** | **92** | **Complete coverage** |

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
