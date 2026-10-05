import type { GitHubWebHook, PullRequestWebHook, WorkflowRunWebHook } from '../github_types'
import type { UpdateMessage } from '../notifiers/notifier'
import { formatCiFailureEvent } from './ci_failure_event'
import type { EventKind } from './event_kind'
import { formatPullRequestEvent } from './pull_request_event'
import { formatPushEvent } from './push_event'

export function resolveEventKind(githubEvent: string | undefined, payload: unknown): EventKind | undefined {
  switch (githubEvent) {
    case 'push':
      return 'PUSH'
    case 'pull_request':
      return 'PULL_REQUEST'
    case 'workflow_run': {
      const { workflow_run: workflowRun } = payload as WorkflowRunWebHook
      return workflowRun.status === 'completed' && workflowRun.conclusion === 'failure'
        ? 'CI_FAILURE'
        : undefined
    }
    default:
      return undefined
  }
}

export function formatEvent(kind: EventKind, payload: unknown): UpdateMessage {
  switch (kind) {
    case 'PUSH':
      return formatPushEvent(payload as GitHubWebHook)
    case 'PULL_REQUEST':
      return formatPullRequestEvent(payload as PullRequestWebHook)
    case 'CI_FAILURE':
      return formatCiFailureEvent(payload as WorkflowRunWebHook)
  }
}
