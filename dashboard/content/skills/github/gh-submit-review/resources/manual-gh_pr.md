+++
title = "manual-gh_pr"
[extra]
skill = false
category = "github"
mermaid = false
skill_name = "gh-submit-review"
+++

## gh pr

Work with GitHub pull requests.

### General commands

- [gh pr create](./gh_pr_create)
- [gh pr list](./gh_pr_list)
- [gh pr status](./gh_pr_status)

### Targeted commands

- [gh pr checkout](./gh_pr_checkout)
- [gh pr checks](./gh_pr_checks)
- [gh pr close](./gh_pr_close)
- [gh pr comment](./gh_pr_comment)
- [gh pr diff](./gh_pr_diff)
- [gh pr edit](./gh_pr_edit)
- [gh pr lock](./gh_pr_lock)
- [gh pr merge](./gh_pr_merge)
- [gh pr ready](./gh_pr_ready)
- [gh pr reopen](./gh_pr_reopen)
- [gh pr revert](./gh_pr_revert)
- [gh pr review](./gh_pr_review)
- [gh pr unlock](./gh_pr_unlock)
- [gh pr update-branch](./gh_pr_update-branch)
- [gh pr view](./gh_pr_view)

### Options

`-R`, `--repo <[HOST/]OWNER/REPO>`  
Select another repository using the \[HOST/\]OWNER/REPO format

### Examples

``` bash
$ gh pr checkout 353
$ gh pr create --fill
$ gh pr view --web
```

### See also

- [gh](./gh)

