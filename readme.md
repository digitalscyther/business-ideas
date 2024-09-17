### run local
```shell
docker compose build && NGINX_PORT=80 POSTGRES_PORT=5432 REDIS_PORT=6379 docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d
```

### .env
```text
RUST_LOG=info

HOST=0.0.0.0
PORT=3000

POSTGRES_URL=postgres://postgres:postgres@postgres/ideas
```