# Settings Management System

A full-stack application for managing schemaless JSON configuration data.

## Project Structure
```
project/
├── backend/          # Rust/Axum REST API
├── frontend/         # SvelteKit UI
├── docker-compose.yml
└── README.md
```

## Quick Start with Docker
```bash
# Build and start all services
docker compose up --build
```

Once running:
- Frontend: http://localhost:5173
- Backend API: http://localhost:3000

### Configuring Data Persistence
edit the compose file to change `./data` on line 14 to a directory of your choice

## Stopping the Application
```bash
# Stop services
docker compose down
```

## Configuration

### Backend Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_PATH` | SQLite database file path | `/app/data/settings.db` |
| `DEFAULT_LIMIT` | Default pagination limit | `10` |
| `MAX_LIMIT` | Maximum pagination limit | `100` |
| `PORT` | Server port | `3000` |

### Frontend Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PUBLIC_API_URL` | Backend API URL | `http://localhost:3000` |
| `PORT` | Server port | `5173` |

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /settings | Create new settings |
| GET | /settings | List all settings (paginated) |
| GET | /settings/{id} | Get settings by ID |
| PUT | /settings/{id} | Update settings |
| DELETE | /settings/{id} | Delete settings |
