export type GitHubWebHook = {
	ref: string;
	repository: Repository;
	pusher: Pusher;
	head_commit: Commit;
	commits: Commits[];
};

export type Repository = {
	full_name: string;
};

export type Commit = {
	message: string;
	author: Pusher;
};

export type Pusher = {
	email: string;
	name: string;
	username: string;
};
