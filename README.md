# diet-tracker

Applicazione fullstack per il monitoraggio della dieta: giornate, pasti, peso e
note, con autenticazione.

Costruita con [Dioxus 0.7](https://dioxuslabs.com/learn/0.7) (target web) e Postgres.

## Avvio

Prerequisiti: [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started) (`dx`)
e Docker.

```bash
cp .env.example .env      # imposta HASH_SECRET e JWT_SECRET
docker compose up -d      # Postgres su 127.0.0.1:5432
dx serve
```

Il server crea il database se manca e applica le migration in `./migrations`
all'avvio. Per fermare il database: `docker compose down` (`-v` per cancellare
anche i dati).
