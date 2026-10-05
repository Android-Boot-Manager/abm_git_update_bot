import type { GitHubWebHook } from '../github_types'
import type { UpdateMessage } from '../notifiers/notifier'

export function formatPushEvent(payload: GitHubWebHook): UpdateMessage {
  const branch = payload.ref.replace('refs/heads/', '')
  const commitLines = payload.commits.map(
    (commit) => `- ${commit.message.split('\n')[0]} (${commit.author.username})`,
  )

  return {
    title: `${payload.repository.full_name}: push to ${branch}`,
    body: commitLines.join('\n'),
    url: payload.compare,
  }
}
