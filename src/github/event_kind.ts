export const EVENT_KINDS = ['PUSH', 'PULL_REQUEST', 'CI_FAILURE'] as const

export type EventKind = (typeof EVENT_KINDS)[number]

export function isEventKind(value: string): value is EventKind {
  return (EVENT_KINDS as readonly string[]).includes(value)
}
