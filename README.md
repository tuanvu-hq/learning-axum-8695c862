# PROJECT-8695c862

## Setup

Run `.bash` files.

## Run

### Rust

```bash
cargo watch -x run
```

### Docker Compose

```bash
# turn on
docker-compose up

# -U: username
# -d: database name
docker-compose exec database psql -U postgres -d postgres

# \dt: describe tables. Displays a list of all tables in the public schema.
\dt

# quit
exit

# shut down
docker-compose down

docker volume ls

docker volume rm axum-8695c862_db-data
```

```bash
# -d: deamon mode, runs process in the background, gives access to the terminal
# --wait: wait for checks
docker-compose up -d --wait

docker-compose logs database
```

### Sea ORM

```bash
cargo install sea-orm-cli

sea-orm-cli generate entity -o src/database
```

## Notes

### `layer` vs. `route_layer`

- `layer`: Middleware applied to all requests, runs every time, even if no routes match.
- `route_layer`: Middleware applied only to specific routes, runs only when a route matches.

Key difference: `layer` is global, `route_layer` is route-specific.
