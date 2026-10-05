import type { GitHubWebHook } from '../github_types'
import type { UpdateMessage } from '../notifiers/notifier'

export function orgFromRepoFullName(fullName: string): string {
  return fullName.split('/')[0]
}

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
