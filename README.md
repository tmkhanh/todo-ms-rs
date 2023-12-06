# Rust Todo microservice

# Database

The service uses sqlx library to asynchronously connect database instance and manipulate data. It also uses
[sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md) to manage data migrations.

- Run container with some default values

```bash
docker run --name todo-db \
  -p 5432:5432 \
  -e POSTGRES_USER=todo \
  -e POSTGRES_PASSWORD=todo \
  -e POSTGRES_DB=todo \
  -d postgres:latest
```

- To migrate use

```bash
sqlx migrate run
```

or following command to revert the change

```bash
sqlx migrate revert
```