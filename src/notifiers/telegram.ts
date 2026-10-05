import type { Notifier, UpdateMessage } from './notifier'

const MARKDOWN_V2_SPECIAL_CHARS = /[_*[\]()~`>#+\-=|{}.!]/g

function escapeMarkdownV2(text: string): string {
  return text.replace(MARKDOWN_V2_SPECIAL_CHARS, (char) => `\\${char}`)
}

export class TelegramNotifier implements Notifier {
  constructor(private readonly botToken: string) {}

  async send(chatId: string, message: UpdateMessage): Promise<void> {
    const text = [
      `*${escapeMarkdownV2(message.title)}*`,
      escapeMarkdownV2(message.body),
      escapeMarkdownV2(message.url),
    ].join('\n')

    const response = await fetch(`https://api.telegram.org/bot${this.botToken}/sendMessage`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        chat_id: chatId,
        text,
        parse_mode: 'MarkdownV2',
        disable_web_page_preview: true,
      }),
    })

    if (!response.ok) {
      throw new Error(`Telegram sendMessage failed: ${response.status} ${await response.text()}`)
    }
  }
}
