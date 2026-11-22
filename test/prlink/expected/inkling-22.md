`git push` to an existing PR prints no URL, so the `post-tool` hook watched nothing for the commonest way CI starts. Every PR here so far was opened with `gh pr create`, which is why nobody hit it.

The rule: **a push watches every open pull request whose head is exactly the repository and branch git reports it pushed to.**

## What changed

- [`detect::pushed`](https://github.com/dpep/inkling/blob/be6da7d/src/domain/detect.rs#L302) reads git's own report: the `To <remote>` line (scp, `ssh://`, `https://`, on a configured GitHub host) and each ref line that landed — fast-forward, `+` forced, `* [new branch]`. Tags, deletions, rejections are ignored. GitHub's "Create a pull request for 'x'" hint is tied to the `To` block that follows it. The gate ([`is_push`](https://github.com/dpep/inkling/blob/be6da7d/src/domain/detect.rs#L287)) now also accepts `git -C <dir> push`.
- Per branch, cheapest first ([`post_tool`](https://github.com/dpep/inkling/blob/be6da7d/src/main.rs#L1600)):
  1. this session's live watch has that head: touch it ([`Watches::stir_pushed`](https://github.com/dpep/inkling/blob/be6da7d/src/adapters/watches_json.rs#L288)). Deadline extends, read schedule back to 1 minute. No network.
  2. GitHub offered to create a PR for it: there isn't one, so skip.
  3. otherwise one GraphQL request for all such branches ([`GitHubProvider::pushed_to`](https://github.com/dpep/inkling/blob/be6da7d/src/adapters/github.rs#L64)), asking the pushed repo and its `parent` (a fork's PRs live upstream). A PR is kept only if its `headRepository` is the pushed repo. Found PRs register the way a URL does (`matched="git push"`, same cap of 8), bounded by [`PUSH_LOOKUP_BUDGET`](https://github.com/dpep/inkling/blob/be6da7d/src/main.rs#L1591) (10s, around token + request).
- The head is stored on the watch ([`Watch::head`](https://github.com/dpep/inkling/blob/be6da7d/src/adapters/watches_json.rs#L140), serde-defaulted so an existing `watches.json` loads). The lookup writes it and every reading refreshes it, since the per-PR query now also asks for `headRefName headRepository { nameWithOwner }` (same request). There's no separate branch→PR cache, so nothing goes stale when a branch gets a new PR.

**Several open PRs from one branch** (different bases): all of them are watched. The push changed every one and each runs CI, and the cap still bounds it.

**The failure mode is a miss, never the wrong PR.** This matters in practice: rust-lang/rust has five open PRs whose head branch is `master`, all from strangers' forks, so matching on `headRefName` alone would be wrong. The page size is 100 so your PR doesn't fall off the end.

## Proof

Tests first. These failed against a stub before the parser existed: update of an existing branch, new branch with the hint, forced push, seven remote spellings, non-GitHub remotes, a multi-ref table with tag/delete/reject/up-to-date lines, a hint scoped to its remote, `git -C`, and the cap. Adapter tests use recorded GraphQL responses: the fork→upstream case, strangers' `master` filtered out, two bases, an unreadable repo or deleted fork, and query escaping. Store tests cover: a reading remembers the head, a match needs the same reader + live + exact repo/branch/host, touch vs. ask vs. skip, and an old file loads.

End to end with `target/debug/inkling` against scratch paths. I used real `git push` output from dpep/inkling-ci-probe, and opened the probe PR (#7) with `gh api` so the live hook wouldn't watch it:

```
push has no pull request yet; nothing to watch repo="dpep/inkling-ci-probe" branch="lane-push-e2e"
looked up the pull requests a push updated branches="dpep/inkling-ci-probe:lane-push-e2e" found=1 elapsed_ms=502
watch registered automatically reference=github.com/dpep/inkling-ci-probe#7 owner="lane-push-e2e" tool="Bash" matched="git push" deadline="2026-09-24T04:12:00Z"
watch stirred by a push reference=github.com/dpep/inkling-ci-probe#7 owner="lane-push-e2e" branch="lane-push-e2e"
```

The first line is the new-branch push, made before the PR existed. The second and third are the first update push once the PR was open: `watches.json` got `"head": {"repo": "dpep/inkling-ci-probe", "branch": "lane-push-e2e"}`. The fourth is a second push: it touched the watch with no lookup. I also deleted `head` from the file and polled, and the reading put it back. Probe PR closed and branch deleted.

## Dogfood

This PR can't use its own feature: the installed build predates it, so pushes to this branch aren't auto-watched. That is the gap being fixed. `gh pr create` registered it through the existing URL path (`watch registered automatically … matched="gh pr create"`, baselined 1s later). `inkling wait --until checks` in the background reported `success github.com/dpep/inkling#22` and `met after 1m 8s (5 polls)`. I could act on that without looking anything else up. No mid-turn push arrived in this subagent, because pushes go to the parent session.

🤖 Generated with Claude Code
