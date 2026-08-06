# Working in this repo

**This repo is public.** It is the shareable slice of a larger private setup
(`dpep/myclaude`), and everything committed here is world-readable forever —
including anything a later force-push tries to take back.

## Before committing, check that nothing here

- names an employer, client, colleague, or internal system
- carries a real ticket key, hostname, endpoint, or account id — use `PROJ-123`,
  `example.com`, placeholders
- embeds an absolute path (`/Users/…`) or an email address
- contains a credential, token, or key, even an expired one
- depends on a plugin that isn't public. A skill or agent that says "see the
  `foo` plugin's reference" is a dead end for everyone who can't install `foo`.
  Summarise inline, or cite the source literature instead.
- links outside the repo with a relative path (`](../foo)` 404s here even
  though it resolved in the private repo)

Personal *conventions* are fine — branch prefixes, where repos live — as long as
they're labelled as such so a reader knows to adapt them. Personal *data* is
not.

## What belongs here

Skills and agent prompts that are useful to someone who isn't Daniel, and that
carry no data. Anything holding memory, goals, or org vocabulary stays in the
private repo.

If a plugin here needs an external binary (`rq`, `gqls`), it must be publicly
installable and the README must say so — a skill whose CLI nobody can get is
worse than no skill.

## Sibling repos

- `dpep/myclaude` — the private superset. Plugins live in one place or the
  other, never both: duplication drifts.
- `dpep/rq`, `dpep/gqls` — the CLIs behind two of the `code` skills. Each ships
  its own copy of its skill, so a behaviour change and its documentation land
  in the same commit. When you change one of those skills here, check whether
  the tool repo needs the same edit.
