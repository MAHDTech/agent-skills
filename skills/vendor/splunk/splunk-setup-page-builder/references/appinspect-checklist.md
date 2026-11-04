# AppInspect findings to expect on a fresh setup-page app, and the fix

These are the exact warnings/failures observed when running
`splunk-appinspect inspect <app> --mode test` (and `--mode precert`)
against a freshly scaffolded app using this skill's pattern, before each
fix was applied. If you see any of these, apply the corresponding fix —
don't suppress or ignore them.

| Finding | Check | Fix |
| --- | --- | --- |
| `No [id] section found in app.conf file.` | `check_version_is_valid_semver` | Add an `[id]` stanza with `name` and `version` to `app.conf`. |
| `No name attribute specified in the [id] stanza` | `check_for_valid_package_id` | `[id]` must include both `name = <app_id>` and `version = <x.y.z>`. |
| `No check_for_updates property found in [package] stanza.` | `check_for_updates_disabled` | Add `check_for_updates = false` to `[package]` (for apps not being uploaded to Splunkbase as auto-updating; keep in mind Splunkbase-listed apps may want this managed differently — ask the user if unsure). |
| `Splunk App packages doesn't contain app.manifest file.` | `check_for_valid_package_id` | Add `app.manifest` at the app root (see SKILL.md step 6). |
| `Stanza [<conf_file>] requires a conf-level reload trigger.` | `check_reload_trigger_for_meta` | Add `reload.<conf_file> = simple` under `[triggers]` in `app.conf` for every custom conf file the setup page writes. |
| `The 'admin' role is not available to Splunk Cloud customers.` | `check_kos_are_accessible` | In `default.meta`, always list `sc_admin` alongside `admin` and `power` in `write` (and `read` if restricted) ACLs. |
| `Do not use default/setup.xml in the Cloud environment.` | `check_setup_xml_in_default` (tag: `cloud`) | Don't create `default/setup.xml` at all. Use a `setup_view` (this skill's whole approach) instead. |
| `Entity 'mdash' not defined` / `Invalid XML File` | `check_that_all_xml_files_are_well_formed` | XML only has 5 built-in entities (`&amp; &lt; &gt; &apos; &quot;`). Use numeric character references instead, e.g. `&#8212;` for an em dash, not HTML entity names like `&mdash;`. |

After applying all of these, a from-scratch setup-page app built with this
skill's pattern passes `splunk-appinspect inspect <app> --mode precert`
with 0 `error`, 0 `failure`, 0 `warning`.

## Running it

```sh
pip install splunk-appinspect
# macOS only, if you hit "ImportError: failed to find libmagic":
brew install libmagic

splunk-appinspect inspect <app_dir_or_tgz> --mode test
splunk-appinspect inspect <app_dir_or_tgz> --mode precert
```

Package as a `.tgz` the same way you'd hand it to Splunkbase:

```sh
tar --exclude='.DS_Store' -czf <app_id>.tgz <app_id>/
```
