# The Case Against Pull Requests in TBD

In many modern development teams, the Pull Request (PR) has become the default "gatekeeper." While intended to ensure
quality, in practice, it often becomes a bottleneck that erodes the core benefits of Trunk-Based Development.

`tbdflow` is designed to replace the PR-centric model with a high-trust, high-throughput workflow based on **Pairing**
and
**Non-blocking Reviews (NBR)**.

## 1. The Hidden Cost of PRs

The PR model introduces several "invisible" costs that NBR specifically solves:

- **The Wait Cost:** The time between "Code Finished" and "Code Integrated." In a PR model, this is often measured in
  hours or days. In TBD, it is measured in seconds.

- **Context Switching:** Developers push a PR and then start something new while waiting for feedback. When feedback
  finally arrives, they must drop their current work to revisit the old code.

- **Batching (Quantity over Quality):** Because the "transaction cost" of opening a PR is high, developers tend to batch
  more changes into a single PR. Larger PRs are statistically harder to review and more likely to contain bugs.

- **False Sense of Security:** Many PRs are "rubber-stamped" (LGTM!) because the reviewer is busy or lacks context. NBR
  encourages a "Fix-Forward" culture where the code is already live, making the stakes for clear understanding higher.

## 2. When do PRs actually make sense?

If we are building a "trusted environment," PRs are almost never the optimal choice. However, `tbdflow` recognises a few
specific scenarios where the PR model might still be used as an exception, not the rule:

### A. Untrusted Contributors (Open Source)

If someone outside the core team (an external contributor or a new hire on their first day) wants to contribute, you
cannot afford them direct access to `main`. In this case, the PR acts as a traditional security boundary.

### B. "Deep-Sea" Experiments

If a developer is working on a high-risk architectural experiment that might take days to even be "runnable," a
short-lived branch with a PR might be used as a "discussion board." However, even here, TBD practitioners would argue
for **Feature Toggles** on the trunk.

### C. Regulatory "Hard Gates"

Some specific industries (Medical, Aerospace, Finance) have legacy compliance requirements that mandate a "Stop/Go" gate
before code hits a production-connected branch. While NBR with a solid audit trail usually satisfies the "4-eyes
principle," some organisations aren't culturally ready to move that gate post-commit.

## 3. The `tbdflow` Alternative: Pairing & NBR

Instead of PRs, `tbdflow` encourages:

- **Pair/Mob Programming:** The ultimate "Real-time Review." If two people wrote the code, the "4-eyes principle" is
  satisfied at the moment of creation. No review issue is needed.

- **Non-blocking Reviews (NBR):** The code hits `main immediately. CI/CD begins. The "Review Issue" is created in
  parallel. If an issue is found, we Fix-Forward.

- **Atomic Commits:** Because integrating is easy, commits are small (<200 lines). These are effortless to review
  compared to a 50-file PR.

## Summary

`tbdflow` stays away from PRs because **throughput beats stability** when stability is redefined as "the ability to fix
things
instantly." By removing the PR trap, we empower developers to take ownership of the trunk and treat every commit as a
production-ready investment.
