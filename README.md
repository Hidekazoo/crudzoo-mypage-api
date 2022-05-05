## Local Development

```bash
kubectl craete namespace sample # for dev namespace
cd environment
skaffold dev -n sample
```

Add .env to add postgres connection information

```env
# example
DATABASE_URL="postgres://postgres:password@localhost:5432/mypage"
```

Migration With sqlx！

```bash
sqlx migrate run
```

## sqlx offline mode
Run sqlx offline mode for Docker build. Move the created sqlx-data.json file to the root directory
```
cd infra
cargo sqlx prepare -- --lib
```