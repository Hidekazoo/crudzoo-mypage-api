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