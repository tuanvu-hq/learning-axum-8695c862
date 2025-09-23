# PROJECT-8695c862

## Setup

Run `.bash` files.

## Run

```bash
cargo watch -x run
```

## Notes

### `layer` vs. `route_layer`

- `layer`: Middleware applied to all requests, runs every time, even if no routes match.
- `route_layer`: Middleware applied only to specific routes, runs only when a route matches.

Key difference: `layer` is global, `route_layer` is route-specific.
