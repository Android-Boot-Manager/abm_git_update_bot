```
npm install
npm run dev
```

```
npm run deploy
```

## Configuration

GitHub events are posted to Telegram via `POST /webhook`. Configure it with:

- `GITHUB_WEBHOOK_SECRET` (secret) — must match the secret configured on the
  GitHub webhook; requests with an invalid `X-Hub-Signature-256` are rejected.
- `TELEGRAM_BOT_TOKEN` (secret) — bot token used to call the Telegram Bot API.
- `TELEGRAM_ORG_CHANNELS` (var, in `wrangler.jsonc`) — JSON array mapping a
  GitHub organisation to the Telegram chat it should notify, e.g.:

  ```json
  [
    { "org": "Android-Boot-Manager", "chatId": "-1001234567890" },
    { "org": "ABM-Community-Ports", "chatId": "-1009876543210" }
  ]
  ```

  Events from repositories in orgs not listed here are ignored. Adding a new
  org only requires editing this list — no code changes needed.
- `ENABLED_EVENTS` (var, in `wrangler.jsonc`) — comma-separated list of event
  kinds to report. Supported kinds:

  - `PUSH` — a push to any branch.
  - `PULL_REQUEST` — a pull request opened, closed, reopened, etc.
  - `CI_FAILURE` — a GitHub Actions workflow run that completed with a
    `failure` conclusion.

  Example: `ENABLED_EVENTS=PUSH,PULL_REQUEST,CI_FAILURE`. Events of a kind not
  listed here are ignored.

Set the secrets locally by copying `.dev.vars.example` to `.dev.vars`, and in
production via `wrangler secret put GITHUB_WEBHOOK_SECRET` / `wrangler secret
put TELEGRAM_BOT_TOKEN`.
