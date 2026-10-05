import type { PullRequestWebHook } from '../github_types'
import type { UpdateMessage } from '../notifiers/notifier'

export function formatPullRequestEvent(payload: PullRequestWebHook): UpdateMessage {
  const { pull_request: pullRequest, repository } = payload

  return {
    title: `${repository.full_name}: pull request ${payload.action}`,
    body: `#${payload.number} ${pullRequest.title} (${pullRequest.user.login})`,
    url: pullRequest.html_url,
  }
}
