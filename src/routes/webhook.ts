import { Hono } from 'hono'
import { findChannelForOrg, loadConfig } from '../config'
import { orgFromRepoFullName } from '../github/repo'
import { formatEvent, resolveEventKind } from '../github/resolve_event'
import { verifySignature } from '../github/signature'
import type { Repository } from '../github_types'
import { TelegramNotifier } from '../notifiers/telegram'

const app = new Hono<{ Bindings: CloudflareBindings }>()

app.post('/', async (c) => {
  const config = loadConfig(c.env)
  const rawBody = await c.req.text()
  const signature = c.req.header('X-Hub-Signature-256')

  if (!(await verifySignature(config.githubWebhookSecret, rawBody, signature))) {
    return c.text('Forbidden', 403)
  }

  const payload = JSON.parse(rawBody) as { repository: Repository }
  const eventKind = resolveEventKind(c.req.header('X-GitHub-Event'), payload)
  if (!eventKind || !config.enabledEvents.has(eventKind)) {
    return c.body(null, 202)
  }

  const org = orgFromRepoFullName(payload.repository.full_name)
  const channel = findChannelForOrg(config, org)
  if (!channel) {
    return c.body(null, 202)
  }

  const notifier = new TelegramNotifier(config.telegramBotToken)
  await notifier.send(channel.chatId, formatEvent(eventKind, payload))

  return c.body(null, 204)
})

export default app
