# diet-tracker

Applicazione fullstack per il monitoraggio della dieta: giornate, pasti, peso e
note, con autenticazione.

Costruita con [Dioxus 0.7](https://dioxuslabs.com/learn/0.7) (target web) e Postgres.

## Avvio con Docker

Unico prerequisito: Docker.

```bash
cp .env.example .env          # imposta HASH_SECRET e JWT_SECRET
docker compose up -d --build  # Postgres + app su http://localhost:8080
```

Il servizio `web` legge i segreti da `.env` e `.env.local` (entrambi opzionali,
`.env.local` ha la precedenza). Le variabili usate nell'interpolazione del
compose file — `WEB_PORT` e le `POSTGRES_*` — arrivano invece solo da `.env` o
da `--env-file`, ma hanno tutte un default.

La build compila il bundle web con `dx bundle --web --release` e avvia un
Postgres effimero dentro l'immagine di build, perché le macro `sqlx::query_as!`
validano le query a compile-time. La prima build è lunga (compilazione Rust +
wasm); le successive riusano la cache di cargo.

## Sviluppo

Con [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started) (`dx`)
installato, conviene tenere in Docker solo il database:

```bash
docker compose up -d db       # Postgres su 127.0.0.1:5432
dx serve
```

Il server crea il database se manca e applica le migration in `./migrations`
all'avvio. Per fermare tutto: `docker compose down` (`-v` per cancellare anche i
dati).

Variabili in `.env`: `DATABASE_URL` (usata solo da `dx serve`; in compose l'app
punta a `db:5432`), `HASH_SECRET`, `JWT_SECRET`, `WEB_PORT` e le
`POSTGRES_USER` / `POSTGRES_PASSWORD` / `POSTGRES_DB` / `POSTGRES_PORT`
opzionali.
