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
