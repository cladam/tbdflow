# What We Can Learn from Martin's NBR Workflows (Trunktopus)

## Architecture Overview

Martin's system uses **6 interconnected workflows** that create a complete "System of Record" for non-blocking reviews:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         TRUNKTOPUS ARCHITECTURE                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   PUSH TO MAIN                                                              │
│        │                                                                    │
│        ▼                                                                    │
│   ┌─────────────────────────────────┐                                       │
│   │ review_issues_for_commit.yaml   │  ◄── Creates AUTHOR + REVIEWER issues │
│   │ • Author issue (assigned to     │      Sets commit status = PENDING     │
│   │   committer)                    │      Adds to GitHub Project           │
│   │ • Reviewer issues (one per      │      Embeds small diffs inline        │
│   │   reviewer)                     │                                       │
│   └─────────────────────────────────┘                                       │
│                                                                             │
│   REVIEWER CLOSES THEIR ISSUE                                               │
│        │                                                                    │
│        ▼                                                                    │
│   ┌─────────────────────────────────┐                                       │
│   │ update-review-issues.yaml       │  ◄── Checks off reviewer in parent    │
│   │ • Updates parent checklist      │      Adds ✅ to title                  │
│   └─────────────────────────────────┘                                       │
│                                                                             │
│   COMMENTS ON COMMIT (GitHub UI)                                            │
│        │                                                                    │
│        ▼                                                                    │
│   ┌─────────────────────────────────┐                                       │
│   │ handle_commit_comments.yaml     │  ◄── Polls for UI comments            │
│   │ • Runs every 2 hours            │      Adds as checklist items          │
│   │ • Syncs comments → issues       │      Creates "review-comment" labels  │
│   └─────────────────────────────────┘                                       │
│                                                                             │
│   ALL TASKS COMPLETE                                                        │
│        │                                                                    │
│        ▼                                                                    │
│   ┌─────────────────────────────────┐                                       │
│   │ auto_close_review_issues.yaml   │  ◄── Consensus logic                  │
│   │ • Checks: all peers reviewed?   │      Auto-closes parent issue         │
│   │ • Checks: all comments handled? │                                       │
│   └─────────────────────────────────┘                                       │
│                                                                             │
│   AUTHOR ISSUE CLOSED                                                       │
│        │                                                                    │
│        ▼                                                                    │
│   ┌─────────────────────────────────┐                                       │
│   │ attach-commit-state-success...  │  ◄── THE GREEN TICK                   │
│   │ • Sets commit status = SUCCESS  │      Enables deploy gating            │
│   └─────────────────────────────────┘                                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Key Learnings

### 1. **Parent-Child Issue Model**

Martin creates TWO types of issues:

- **Author Issue** (`author-issue` label): Assigned to the committer, contains checklist
- **Reviewer Issues** (`peer-review` label): One per reviewer, linked to parent

This enables **multi-reviewer orchestration** - you know when ALL reviewers are done.

### 2. **Commit Status as Gate**

```yaml
gh api repos/$REPO/statuses/$COMMIT \
-f state="pending" \
-f context="peer-review" \
-f description="Awaiting peer review"
```

This is the **compliance killer feature** - CI/CD can require this status before deploying.

### 3. **UI Comment Synchronization**

The `handle_commit_comments.yaml` runs on a schedule and:

- Fetches comments from GitHub's commit comment API
- Adds them as checklist items in the author issue
- Creates a paper trail even for "drive-by" comments

### 4. **Inline Diffs for Small Changes**

```yaml
if [ "$DIFF_LINE_COUNT" -le 60 ]; then
DIFF_SECTION=$(printf "```diff\n%s\n```" "$COMMIT_DIFF")
fi
```

Brilliant UX - small changes show the diff right in the issue!

### 5. **GitHub App Token**

Uses a GitHub App instead of `GITHUB_TOKEN` for:

- Higher rate limits
- Cross-repository actions
- Better audit identity

### 6. **Concurrency Control**

```yaml
concurrency:
  group: create-reviews-for-commits
  cancel-in-progress: false
```

Prevents race conditions when multiple pushes happen quickly.

## What tbdflow Should Adopt

| Feature        | Current tbdflow | Martin's Approach      | Recommendation  |
|----------------|-----------------|------------------------|-----------------|
| Issue Creation | Single issue    | Parent + Child issues  | Consider for v2 |
| Commit Status  | ❌ None          | ✅ Pending → Success    | **Must have**   |
| UI Comments    | ❌ Not synced    | ✅ Polled & synced      | Nice to have    |
| Multi-reviewer | ❌ Single list   | ✅ Separate issues      | Consider for v2 |
| Inline Diff    | ❌ Just link     | ✅ Small diffs embedded | **Easy win**    |
| Deploy Gating  | ❌ Not possible  | ✅ Via commit status    | **Must have**   |

## Immediate Improvements for tbdflow

1. **Add commit status** in our workflow (pending on create, success on close)
2. **Embed small diffs** in issue body when < 60 lines
3. **Add concurrency control** to prevent duplicate issues
4. **Use `author-issue` label** to distinguish from future reviewer issues
