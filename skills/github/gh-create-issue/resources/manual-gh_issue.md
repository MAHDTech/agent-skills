<div id="main"
class="main-content color-bg-subtle markdown-body col-12 col-lg-10 pt-7 pb-12 px-3 px-md-6"
role="main">

<div class="container-lg">

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

<figure class="highlight">
<div class="sourceCode" id="cb1" data-lang="bash"><pre
class="sourceCode bash"><code class="sourceCode bash"><span id="cb1-1"><a href="#cb1-1" aria-hidden="true" tabindex="-1"></a><span class="ex">$</span> gh issue list</span>
<span id="cb1-2"><a href="#cb1-2" aria-hidden="true" tabindex="-1"></a><span class="ex">$</span> gh issue create <span class="at">--label</span> bug</span>
<span id="cb1-3"><a href="#cb1-3" aria-hidden="true" tabindex="-1"></a><span class="ex">$</span> gh issue view 123 <span class="at">--web</span></span></code></pre></div>
</figure>

### See also

- [gh](./gh)

</div>

<div class="container-xl px-3 px-md-4">

<div class="d-flex flex-wrap py-5 mb-5">

<div class="col-12 col-lg-4 mb-5">

<a href="https://github.com/" class="color-fg-default"
aria-label="GitHub"><img
src="data:image/svg+xml;base64,PHN2ZyB2aWV3Ym94PSIwIDAgMTAzIDI0IiBoZWlnaHQ9IjMwIiBjbGFzcz0ib2N0aWNvbiBvY3RpY29uLWxvY2t1cC1naXRodWIiIGFyaWEtaGlkZGVuPSJ0cnVlIj48cGF0aCBkPSJNMzYuNDcyIDMuMDg4YzQuMjI1IDAgNi43NzcgMS44MDUgNy41OTEgNC44NDFsLTMuNzg0LjkwMmMtLjQ2Mi0xLjY1LTEuNzgzLTIuNTUyLTMuODA3LTIuNTUyLTMuMDE1IDAtNC42NDMgMS45OC00LjY0MyA1Ljc0M3MxLjU4NCA1Ljc4NyA0LjU1NSA1Ljc4N2MyLjUwMiAwIDQuMDc5LTEuNDA2IDQuMzIzLTMuODI5aC00LjYzMXYtMy4xNjhoOC40NXYyLjAwMmMwIDUuMjYtMy4wNTkgOC4xODYtOC4yMDggOC4xODZDMzEuMjEzIDIxIDI4IDE3LjUwMSAyOCAxMi4wMjJjMC01LjQ4IDMuMjc5LTguOTM0IDguNDcyLTguOTM0TTc5LjYyIDE1Ljc0MWMwIDEuNTE4LjY4MiAyLjM1NSAxLjk4IDIuMzU1IDEuNDk3IDAgMi42NC0xLjM4NyAyLjY0LTMuMjhWOC4zMDRoMy41VjIwLjY3aC0zLjV2LTIuMDljLS42NiAxLjM0Mi0yLjI0NCAyLjMxLTMuOTM4IDIuMzEtMi43MDcgMC00LjE4MS0xLjQ5Ni00LjE4MS00LjIyNVY4LjMwM2gzLjQ5OXoiIC8+PHBhdGggZmlsbC1ydWxlPSJldmVub2RkIiBkPSJNOTMuMDYzIDEwLjU3QzkzLjc3NSA5LjA3MyA5NS4zOCA4LjAxNyA5NyA4LjAxN2MzLjM3NSAwIDUuMTU4IDIuMzMzIDUuMDM5IDYuNDQ4LjExOSA0LjA3LTEuNzMgNi40MjUtNS4wMzkgNi40MjUtMS42ODYgMC0zLjIyNi0uOTY4LTMuOTYtMi4zNTV2Mi4xMzVoLTMuNDc3VjMuNDE4aDMuNDk5em0yLjcwNi4zNzRjLTEuNDQ0IDAtMi41ODggMS40MDgtMi43MDcgMy4zNDV2LjE5OGMuMTE5IDIuMDAyIDEuMjYzIDMuNDc2IDIuNzA3IDMuNDc2IDEuODEzIDAgMi44MDMtMS4zNDIgMi42ODUtMy40OTguMTE4LTIuMTc5LS44NzItMy41MjEtMi42ODUtMy41MjEiIC8+PHBhdGggZD0iTTQ5LjMyMyAyMC42N2gtMy40NzdWOC4zMDNoMy40Nzd6bTcuMDA0LTEyLjM2N2gyLjU3NXYyLjkwNWgtMi41NzV2NS42MTFjMCAuNjgyLjMwOC45NDYuOTkuOTQ2aDEuNTg1djIuOTA1aC0yLjgzOWMtMi4zMSAwLTMuMjEyLS45OS0zLjIxMi0zLjA4di02LjM4Mkg1MC42NVY4LjMwM2gyLjJWNS44NGwzLjQ3Ny0uODE0em03Ljc4My00Ljg4NXY2Ljk5OGg2LjU1OFYzLjQxOGgzLjc0VjIwLjY3aC0zLjc0di02LjkxSDY0LjExdjYuOTFoLTMuNzRWMy40MTh6TTQ3LjU4NCAzYzEuMTY3IDAgMi4wNjkuOTAyIDIuMDY5IDIuMDY5YTIuMDQgMi4wNCAwIDAgMS0yLjA2OSAyLjA2OCAyLjA0IDIuMDQgMCAwIDEtMi4wNjgtMi4wNjhjMC0xLjE2Ny45MDItMi4wNjkgMi4wNjgtMi4wNjlNMTAuMjI2IDE3LjI4NGMtMi45NjUtLjM2LTUuMDU0LTIuNDkzLTUuMDU0LTUuMjU2IDAtMS4xMjMuNDA0LTIuMzM2IDEuMDc4LTMuMTQ0LS4yOTItLjc0MS0uMjQ3LTIuMzE0LjA5LTIuOTY1Ljg5OC0uMTEyIDIuMTExLjM2IDIuODMgMS4wMS44NTMtLjI2OSAxLjc1Mi0uNDA0IDIuODUzLS40MDQgMS4xIDAgMS45OTkuMTM1IDIuODA3LjM4Mi42OTYtLjYyOSAxLjkzMi0xLjEgMi44My0uOTg4LjMxNS42MDYuMzYgMi4xNzkuMDY3IDIuOTQyLjcyLjg1NCAxLjEwMSAyIDEuMTAxIDMuMTY3IDAgMi43NjMtMi4wODkgNC44NTItNS4wOTggNS4yMzQuNzYzLjQ5NCAxLjI4IDEuNTcyIDEuMjggMi44MDd2Mi4zMzZjMCAuNjc0LjU2MSAxLjA1NiAxLjIzNS43ODYgNC4wNjYtMS41NSA3LjI1NS01LjYxNSA3LjI1NS0xMC42NDZDMjMuNSA2LjE4OCAxOC4zMzQgMSAxMS45NzggMSA1LjYyIDEgLjUgNi4xODguNSAxMi41NDVjMCA0Ljk4NiAzLjE2NyA5LjEyIDcuNDM1IDEwLjY2OS42MDYuMjI1IDEuMTktLjE4IDEuMTktLjc4NlYyMC42M2EyLjkgMi45IDAgMCAxLTEuMDc4LjIyNGMtMS40ODMgMC0yLjM1OS0uODA4LTIuOTg3LTIuMzEzLS4yNDctLjYwNy0uNTE3LS45NjYtMS4wMzQtMS4wMzMtLjI3LS4wMjMtLjM1OS0uMTM1LS4zNTktLjI3IDAtLjI3LjQ1LS40NzEuODk4LS40NzEuNjUyIDAgMS4yMTMuNDA0IDEuNzk3IDEuMjM1LjQ1LjY1MS45MjEuOTQzIDEuNDgzLjk0My41NjEgMCAuOTItLjIwMiAxLjQzNy0uNzE5LjM4Mi0uMzgxLjY3NC0uNzE4Ljk0NC0uOTQzIiAvPjwvc3ZnPg=="
class="octicon octicon-lockup-github" /></a>

</div>

<div class="col-6 col-sm-3 col-lg-2 mb-6 mb-md-2 pr-3 pr-lg-0 pl-lg-4">

### Product

- <a href="https://github.com/features"
  class="color-fg-subtle">Features</a>
- <a href="https://github.com/security"
  class="color-fg-subtle">Security</a>
- <a href="https://github.com/enterprise"
  class="color-fg-subtle">Enterprise</a>
- <a href="https://github.com/customer-stories"
  class="color-fg-subtle">Customer stories</a>
- <a href="https://github.com/pricing" class="color-fg-subtle">Pricing</a>
- <a href="https://resources.github.com"
  class="color-fg-subtle">Resources</a>

</div>

<div class="col-6 col-sm-3 col-lg-2 mb-6 mb-md-2 pr-3 pr-md-0 pl-md-4">

### Platform

- <a href="https://developer.github.com/"
  class="color-fg-subtle">Developer API</a>
- <a href="http://partner.github.com/"
  class="color-fg-subtle">Partners</a>
- <a href="https://desktop.github.com/" class="color-fg-subtle">GitHub
  Desktop</a>
- <a href="https://github.com/mobile" class="color-fg-subtle">GitHub
  Mobile</a>

</div>

<div class="col-6 col-sm-3 col-lg-2 mb-6 mb-md-2 pr-3 pr-md-0 pl-md-4">

### Support

- <a href="https://help.github.com/en" class="color-fg-subtle">Help</a>
- <a href="https://github.community" class="color-fg-subtle">Community
  Forum</a>
- <a href="https://github.com/services/" class="color-fg-subtle">Expert
  Services</a>
- <a href="https://githubstatus.com/" class="color-fg-subtle">Status</a>
- <a href="https://github.com/contact" class="color-fg-subtle">Contact
  GitHub</a>

</div>

<div class="col-6 col-sm-3 col-lg-2 mb-6 mb-md-2 pr-3 pr-md-0 pl-md-4">

### Company

- <a href="https://github.com/about" class="color-fg-subtle">About</a>
- <a href="https://github.blog/" class="color-fg-subtle">Blog</a>
- <a href="https://github.com/about/careers"
  class="color-fg-subtle">Careers</a>
- <a href="https://github.com/about/press"
  class="color-fg-subtle">Press</a>
- <a href="https://shop.github.com" class="color-fg-subtle">Shop</a>

</div>

</div>

</div>

<div class="color-bg-subtle">

<div class="container-xl px-3 px-md-4 f6 py-4 d-sm-flex flex-justify-between flex-row flex-items-center">

- © 2026 GitHub, Inc.
- <a href="https://help.github.com/en/articles/github-terms-of-service/"
  class="color-fg-muted">Terms</a>
- <a href="https://help.github.com/en/articles/github-privacy-statement/"
  class="color-fg-muted">Privacy</a>
- <a
  href="https://docs.github.com/en/github-cli/github-cli/github-cli-telemetry"
  class="color-fg-muted">Telemetry</a>

<!-- -->

- <a href="https://twitter.com/github" class="color-fg-muted"
  title="GitHub on Twitter"><img
  src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdib3g9IjAgMCAyNzMuNSAyMjIuMyIgY2xhc3M9ImQtYmxvY2siIGhlaWdodD0iMTgiPgogICAgICAgICAgICAgIDxwYXRoIGQ9Ik0yNzMuNSAyNi4zYTEwOS43NyAxMDkuNzcgMCAwIDEtMzIuMiA4LjggNTYuMDcgNTYuMDcgMCAwIDAgMjQuNy0zMSAxMTMuMzkgMTEzLjM5IDAgMCAxLTM1LjcgMTMuNiA1Ni4xIDU2LjEgMCAwIDAtOTcgMzguNCA1NCA1NCAwIDAgMCAxLjUgMTIuOEExNTkuNjggMTU5LjY4IDAgMCAxIDE5LjEgMTAuM2E1Ni4xMiA1Ni4xMiAwIDAgMCAxNy40IDc0LjkgNTYuMDYgNTYuMDYgMCAwIDEtMjUuNC03di43YTU2LjExIDU2LjExIDAgMCAwIDQ1IDU1IDU1LjY1IDU1LjY1IDAgMCAxLTE0LjggMiA2Mi4zOSA2Mi4zOSAwIDAgMS0xMC42LTEgNTYuMjQgNTYuMjQgMCAwIDAgNTIuNCAzOSAxMTIuODcgMTEyLjg3IDAgMCAxLTY5LjcgMjQgMTE5IDExOSAwIDAgMS0xMy40LS44IDE1OC44MyAxNTguODMgMCAwIDAgODYgMjUuMmMxMDMuMiAwIDE1OS42LTg1LjUgMTU5LjYtMTU5LjYgMC0yLjQtLjEtNC45LS4yLTcuM2ExMTQuMjUgMTE0LjI1IDAgMCAwIDI4LjEtMjkuMSIgZmlsbD0iY3VycmVudENvbG9yIiAvPgogICAgICAgICAgICA8L3N2Zz4="
  class="d-block" /></a>
- <a href="https://www.facebook.com/GitHub" class="color-fg-muted"
  title="GitHub on Facebook"><img
  src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdib3g9IjAgMCAxNS4zIDE1LjQiIGNsYXNzPSJkLWJsb2NrIiBoZWlnaHQ9IjE4Ij4KICAgICAgICAgICAgICA8cGF0aCBkPSJNMTQuNSAwSC44YS44OC44OCAwIDAgMC0uOC45djEzLjZhLjg4Ljg4IDAgMCAwIC44LjloNy4zdi02aC0yVjcuMWgyVjUuNGEyLjg3IDIuODcgMCAwIDEgMi41LTMuMWguNWExMC44NyAxMC44NyAwIDAgMSAxLjguMXYyLjFoLTEuM2MtMSAwLTEuMS41LTEuMSAxLjF2MS41aDIuM2wtLjMgMi4zaC0ydjUuOWgzLjlhLjg4Ljg4IDAgMCAwIC45LS44Vi44YS44Ni44NiAwIDAgMC0uOC0uOHoiIGZpbGw9ImN1cnJlbnRDb2xvciIgLz4KICAgICAgICAgICAgPC9zdmc+"
  class="d-block" /></a>
- <a href="https://www.youtube.com/github" class="color-fg-muted"
  title="GitHub on YouTube"><img
  src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdib3g9IjAgMCAxOS4xNyAxMy42IiBjbGFzcz0iZC1ibG9jayIgaGVpZ2h0PSIxNiI+CiAgICAgICAgICAgICAgPHBhdGggZD0iTTE4Ljc3IDIuMTNBMi40IDIuNCAwIDAgMCAxNy4wOS40MkMxNS41OSAwIDkuNTggMCA5LjU4IDBhNTcuNTUgNTcuNTUgMCAwIDAtNy41LjRBMi40OSAyLjQ5IDAgMCAwIC4zOSAyLjEzIDI2LjI3IDI2LjI3IDAgMCAwIDAgNi44YTI2LjE1IDI2LjE1IDAgMCAwIC4zOSA0LjY3IDIuNDMgMi40MyAwIDAgMCAxLjY5IDEuNzFjMS41Mi40MiA3LjUuNDIgNy41LjQyYTU3LjY5IDU3LjY5IDAgMCAwIDcuNTEtLjQgMi40IDIuNCAwIDAgMCAxLjY4LTEuNzEgMjUuNjMgMjUuNjMgMCAwIDAgLjQtNC42NyAyNCAyNCAwIDAgMC0uNC00LjY5ek03LjY3IDkuNzFWMy44OWw1IDIuOTF6IiBmaWxsPSJjdXJyZW50Q29sb3IiIC8+CiAgICAgICAgICAgIDwvc3ZnPg=="
  class="d-block" /></a>
- <a href="https://www.linkedin.com/company/github" class="color-fg-muted"
  title="GitHub on Linkedin"><img
  src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdib3g9IjAgMCAxOSAxOCIgY2xhc3M9ImQtYmxvY2siIGhlaWdodD0iMTgiPgogICAgICAgICAgICAgIDxwYXRoIGQ9Ik0zLjk0IDJBMiAyIDAgMSAxIDIgMGEyIDIgMCAwIDEgMS45NCAyek00IDUuNDhIMFYxOGg0em02LjMyIDBINi4zNFYxOGgzLjk0di02LjU3YzAtMy42NiA0Ljc3LTQgNC43NyAwVjE4SDE5di03LjkzYzAtNi4xNy03LjA2LTUuOTQtOC43Mi0yLjkxeiIgZmlsbD0iY3VycmVudENvbG9yIiAvPgogICAgICAgICAgICA8L3N2Zz4="
  class="d-block" /></a>
- <a href="https://github.com/github" class="color-fg-muted"
  title="GitHub&#39;s organization"><img
  src="data:image/svg+xml;base64,PHN2ZyB2aWV3Ym94PSIwIDAgMTYgMTYiIHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCIgY2xhc3M9Im9jdGljb24gb2N0aWNvbi1tYXJrLWdpdGh1YiBkLWJsb2NrIiBhbHQ9IkdpdEh1YiIgYXJpYS1oaWRkZW49InRydWUiPgogICAgICAgICAgICAgIDxwYXRoIGQ9Ik02Ljc2NiAxMS4zMjhjLTIuMDYzLS4yNS0zLjUxNi0xLjczNC0zLjUxNi0zLjY1NiAwLS43ODEuMjgxLTEuNjI1Ljc1LTIuMTg4LS4yMDMtLjUxNS0uMTcyLTEuNjA5LjA2My0yLjA2Mi42MjUtLjA3OCAxLjQ2OC4yNSAxLjk2OC43MDMuNTk0LS4xODcgMS4yMTktLjI4MSAxLjk4NS0uMjgxLjc2NSAwIDEuMzkuMDk0IDEuOTUzLjI2NS40ODQtLjQzNyAxLjM0NC0uNzY1IDEuOTY5LS42ODcuMjE4LjQyMi4yNSAxLjUxNS4wNDYgMi4wNDcuNS41OTMuNzY2IDEuMzkuNzY2IDIuMjAzIDAgMS45MjItMS40NTMgMy4zNzUtMy41NDcgMy42NC41MzEuMzQ0Ljg5IDEuMDk0Ljg5IDEuOTU0djEuNjI1YzAgLjQ2OC4zOTEuNzM0Ljg2LjU0N0MxMy43ODEgMTQuMzU5IDE2IDExLjUzIDE2IDguMDMgMTYgMy42MSAxMi40MDYgMCA3Ljk4NCAwIDMuNTYzIDAgMCAzLjYxIDAgOC4wMzFhNy44OCA3Ljg4IDAgMCAwIDUuMTcyIDcuNDIyYy40MjIuMTU2LjgyOC0uMTI1LjgyOC0uNTQ3di0xLjI1Yy0uMjE5LjA5NC0uNS4xNTYtLjc1LjE1Ni0xLjAzMSAwLTEuNjQtLjU2Mi0yLjA3OC0xLjYwOS0uMTcyLS40MjItLjM2LS42NzItLjcxOS0uNzE5LS4xODctLjAxNS0uMjUtLjA5My0uMjUtLjE4NyAwLS4xODguMzEzLS4zMjguNjI1LS4zMjguNDUzIDAgLjg0NC4yODEgMS4yNS44Ni4zMTMuNDUyLjY0LjY1NSAxLjAzMS42NTVzLjY0MS0uMTQgMS0uNWMuMjY2LS4yNjUuNDctLjUuNjU3LS42NTYiIC8+CiAgICAgICAgICAgIDwvc3ZnPg=="
  class="octicon octicon-mark-github d-block" /></a>

</div>

</div>

</div>
