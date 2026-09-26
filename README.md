# Diet Tracker

A self-hosted diet tracker you share with the people you diet with, built to be
used first and foremost through AI agents.

## Why

Most diet trackers are built around a form you have to open and fill in after
every meal. That friction is why people stop logging. This project assumes you'd
rather tell an agent "I had a plate of pasta for lunch" and let it do the
bookkeeping. Hermes works, and so does any other assistant that can make HTTP
calls.

So the API is the product. Every rule lives in the backend: calorie targets
carrying over from one day to the next, whether a day stayed within its target,
diary search, weight progress, trends. An agent asking a question gets the same
answer the web app shows, without reimplementing anything. The web app is there
for glancing at progress, not as the main way in.

It is **self-hosted** because what you eat and what you weigh is personal. The
data sits in your own Postgres, next to an app you control.

It is **shared** because dieting alone is harder. Everyone on an instance can see
everyone else's diary, weight trend and streak. That transparency is deliberate:
it is what keeps a group motivated. Each person can only ever change their own
data.

## For agents

Everything below is plain HTTP and JSON, served from the instance's origin
(e.g. `https://diet.example.com`).

### Authentication

Sessions are cookies. Log in with a user's name and password, keep the cookies
it sets, and send them with every request:

```bash
curl -c jar.txt -H 'Content-Type: application/json' \
  -d '{"payload":{"name":"ivan","password":"…"}}' \
  https://diet.example.com/api/auth/login

curl -b jar.txt https://diet.example.com/api/stats/summary
```

- `session` lasts **10 minutes**. When a request answers `401`, call
  `POST /api/auth/refresh` (it reads the `refresh_token` cookie, which lasts
  7 days and is rotated on every refresh), store the new cookies, and retry.
  If refresh also answers `401`, log in again.
- Outside local development the cookies are `Secure`, so the instance must be
  served over HTTPS.
- `POST /api/auth/register` takes the same payload as login and creates the
  user. `GET /api/auth/me` returns who you are.

### Conventions

- **Reads** are `GET` requests with filters in the query string. Every filter
  is optional.
- **Writes** are `POST` requests whose JSON body wraps the arguments in
  `payload`: `{"payload": {...}}`.
- **Dates** are `YYYY-MM-DD`.
- **Users are identified by name**, never by id. On reads, `user_name` is
  optional and defaults to the signed-in user. Any signed-in user can read
  anyone's data. On writes, `user_name` (or `name`) is required and must be the
  signed-in user; anything else answers `401`.
- **"Today" is the server's date** unless you pass one. If the user's timezone
  differs from the server's, always send `date` (or `from`/`to`) explicitly.
- **Errors** carry a status code and a human-readable message in Italian, meant
  to be shown to the user as-is. The message is in `data.ServerError.message`,
  and validation errors list the offending fields in
  `data.ServerError.details`:

  ```json
  {"code": 400, "data": {"ServerError": {"message": "Dati non validi", "code": 400,
    "details": {"name": ["Inserisci un nome per la voce"]}}}}
  ```

  `400` invalid input, `401` not signed in or not allowed, `404` unknown user,
  `409` user name taken. A malformed query parameter (e.g. `outcome=maybe`)
  currently answers `500`.

### The model

- An **entry** is something eaten: a name, calories and optional notes, on a date.
- A **day** holds what applies to a date as a whole: the calorie target, the
  weight and notes. Adding an entry on a date that has no day yet creates it,
  carrying the calorie target over from the latest earlier day. A user's very
  first day needs an explicit `target_calories`.
- A day is **within** its target when the calories eaten are at most the
  target, **over** otherwise.
- The **streak** is computed by the server and can't be set. A date counts once
  it holds a weight or at least one entry (a day with only the calorie target
  doesn't), logged by the end of the second day after that date: Monday can
  still be filled in on Wednesday, not on Thursday. What matters is when the
  first weight or entry was logged: filling a date in later is fine for the
  diary but never counts, and neither does removing the weight and setting it
  again, or deleting the day and recreating it, after the deadline.
  - `streak` is the number of consecutive counted dates ending at the most
    recent one. A missing date that can no longer be filled in ends the run
    for good: `streak` restarts from the dates after it, or is `0` if there are
    none.
  - `streak_at_risk` is `true` while a missing date in the last two days can
    still be filled in and would join the run before it. Until it is filled
    in, `streak` counts only the most recent run: the one before the gap if
    nothing has been logged after it, otherwise the dates after the gap.
  - Streaks always follow the server's clock; the `date` parameter doesn't
    affect them.

### Reading

| Endpoint | Returns | Query parameters |
|---|---|---|
| `GET /api/stats/summary` | The user (with `streak` and `streak_at_risk`), calories eaten and target for a date, weight progress (current, starting, delta, progress towards the goal weight), number of days recorded | `user_name`, `date` |
| `GET /api/stats/community` | The same summary for every user | `date` |
| `GET /api/stats/trend` | One point per date: calories, target and weight (the last two carried over from earlier days). At most one year | `user_name`, `from` (default: 29 days before `to`), `to` (default: today) |
| `GET /api/diary` | Days, newest first, each with its entries and calorie total | `user_name`, `from`, `to`, `outcome` (`within` \| `over`), `q` |
| `GET /api/days` | Days, newest first | `user_name`, `from`, `to`, `outcome`, `q` |
| `GET /api/entries` | Entries, newest first | `user_name`, `from`, `to`, `q` |
| `GET /api/users` | All users, with goal weight, `streak` and `streak_at_risk` | none |

`from` and `to` are inclusive. `q` is a case-insensitive search in entry names
and notes. On `/api/diary` and `/api/days` it keeps the days with at least one
matching entry; `/api/diary` then returns each of those days with all of its
entries, not just the matching ones.

### Writing

| Endpoint | `payload` |
|---|---|
| `POST /api/entries` | `user_name`, `date`, `name`, `calories`, `notes?` |
| `POST /api/entries/notes` | `id`, `user_name`, `notes?` |
| `POST /api/entries/delete` | `id`, `user_name` |
| `POST /api/days` | `user_name`, `date`, `target_calories?`, `weight_kg?`, `notes?` |
| `POST /api/days/target-calories` | `user_name`, `date`, `target_calories` |
| `POST /api/days/weight` | `user_name`, `date`, `weight_kg?` |
| `POST /api/days/notes` | `user_name`, `date`, `notes?` |
| `POST /api/days/delete` | `user_name`, `date` |
| `POST /api/users/target-weight` | `name`, `target_weight_kg` |

The `/api/days/*` updates need the day to exist: create it with `POST /api/days`
first, or by adding an entry on that date.

### Recipes

- **"I had pasta for lunch, about 600 kcal"** → `POST /api/entries` with today's
  date. Confirm with the day's new total from `GET /api/stats/summary?date=…`.
- **"How am I doing today?"** → `GET /api/stats/summary?date=…`.
- **"When did I last eat pizza?"** → `GET /api/entries?q=pizza`; the first
  result is the most recent.
- **"How many days did I go over this month?"** →
  `GET /api/days?from=…&to=…&outcome=over` and count the results.
- **"How is Marco doing this week?"** →
  `GET /api/stats/trend?user_name=Marco&from=…&to=…`, or
  `GET /api/diary?user_name=Marco&from=…` for what he actually ate.
- **"Who's keeping up best?"** → `GET /api/stats/community?date=…`.
- **"Is my streak safe?"** → `GET /api/auth/me`: if `streak_at_risk` is `true`,
  remind the user to fill in the day they missed: it still counts until the end
  of the second day after it.
