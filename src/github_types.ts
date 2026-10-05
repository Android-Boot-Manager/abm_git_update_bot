export type GitHubWebHook = {
  ref: string
  compare: string
  repository: Repository
  pusher: Pusher
  head_commit: Commit
  commits: Commit[]
}

export type Repository = {
  full_name: string
  html_url: string
}

export type Commit = {
  id: string
  url: string
  message: string
  author: CommitAuthor
}

export type CommitAuthor = {
  email: string
  name: string
  username: string
}

export type Pusher = {
  email: string
  name: string
}

export type PullRequestWebHook = {
  action: string
  number: number
  pull_request: PullRequest
  repository: Repository
}

export type PullRequest = {
  html_url: string
  title: string
  merged: boolean
  user: PullRequestUser
}

export type PullRequestUser = {
  login: string
}

export type WorkflowRunWebHook = {
  action: string
  workflow_run: WorkflowRun
  repository: Repository
}

export type WorkflowRun = {
  name: string
  html_url: string
  status: string
  conclusion: string | null
  head_branch: string
}
