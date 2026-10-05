import type { WorkflowRunWebHook } from '../github_types'
import type { UpdateMessage } from '../notifiers/notifier'

export function formatCiFailureEvent(payload: WorkflowRunWebHook): UpdateMessage {
  const { workflow_run: workflowRun, repository } = payload

  return {
    title: `${repository.full_name}: CI failure`,
    body: `${workflowRun.name} failed on ${workflowRun.head_branch}`,
    url: workflowRun.html_url,
  }
}
