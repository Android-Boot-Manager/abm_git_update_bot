export type OrgChannel = {
  org: string
  chatId: string
}

export type AppConfig = {
  githubWebhookSecret: string
  telegramBotToken: string
  orgChannels: OrgChannel[]
}

export class ConfigError extends Error {}

export function loadConfig(env: CloudflareBindings): AppConfig {
  const { GITHUB_WEBHOOK_SECRET, TELEGRAM_BOT_TOKEN, TELEGRAM_ORG_CHANNELS } = env

  if (!GITHUB_WEBHOOK_SECRET) {
    throw new ConfigError('GITHUB_WEBHOOK_SECRET is not configured')
  }
  if (!TELEGRAM_BOT_TOKEN) {
    throw new ConfigError('TELEGRAM_BOT_TOKEN is not configured')
  }
  if (!TELEGRAM_ORG_CHANNELS) {
    throw new ConfigError('TELEGRAM_ORG_CHANNELS is not configured')
  }

  let orgChannels: OrgChannel[]
  try {
    orgChannels = JSON.parse(TELEGRAM_ORG_CHANNELS)
  } catch {
    throw new ConfigError('TELEGRAM_ORG_CHANNELS is not valid JSON')
  }

  return {
    githubWebhookSecret: GITHUB_WEBHOOK_SECRET,
    telegramBotToken: TELEGRAM_BOT_TOKEN,
    orgChannels,
  }
}

export function findChannelForOrg(config: AppConfig, org: string): OrgChannel | undefined {
  return config.orgChannels.find((channel) => channel.org === org)
}
