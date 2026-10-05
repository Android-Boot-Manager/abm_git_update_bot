export function orgFromRepoFullName(fullName: string): string {
  return fullName.split('/')[0]
}
