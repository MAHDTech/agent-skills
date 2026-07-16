+++
title = "manual-gh_issue"
[extra]
skill = false
category = "github"
mermaid = false
skill_name = "gh-triage"
+++

## gh issue

Work with GitHub issues.

### General commands

- [gh issue create](./gh_issue_create)
- [gh issue list](./gh_issue_list)
- [gh issue status](./gh_issue_status)

### Targeted commands

- [gh issue close](./gh_issue_close)
- [gh issue comment](./gh_issue_comment)
- [gh issue delete](./gh_issue_delete)
- [gh issue develop](./gh_issue_develop)
- [gh issue edit](./gh_issue_edit)
- [gh issue lock](./gh_issue_lock)
- [gh issue pin](./gh_issue_pin)
- [gh issue reopen](./gh_issue_reopen)
- [gh issue transfer](./gh_issue_transfer)
- [gh issue unlock](./gh_issue_unlock)
- [gh issue unpin](./gh_issue_unpin)
- [gh issue view](./gh_issue_view)

### Options

`-R`, `--repo <[HOST/]OWNER/REPO>`  
Select another repository using the \[HOST/\]OWNER/REPO format

### Examples

``` bash
$ gh issue list
$ gh issue create --label bug
$ gh issue view 123 --web
```

### See also

- [gh](./gh)

