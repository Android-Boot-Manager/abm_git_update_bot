export type UpdateMessage = {
  title: string
  body: string
  url: string
}

export interface Notifier {
  send(targetId: string, message: UpdateMessage): Promise<void>
}
