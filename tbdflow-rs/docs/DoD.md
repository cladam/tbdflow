## The philosophy of th DoD Check

The Definition of Done (DoD) check is more than just a pre-commit hook; it's a tool designed to support a specific way of working. 
This document explains the philosophy behind its creation and the problems it aims to solve.

### Background: Moving beyond the internal PR model

The Pull Request (PR) is a brilliant invention for managing contributions from untrusted sources in open-source projects. It provides a necessary quality gate to protect the integrity of a codebase.

However, this model has been widely adopted for internal development within trusted teams, where it can become a form of process waste. As outlined in the principles of Trunk-Based Development (TBD), forcing team members through a formal PR process is like making your family go through airport security to enter your home.
It’s a costly solution to a different problem.

Formal PRs introduce mandatory delays and context switching, disrupting the flow that TBD and Continuous Integration (CI) aim to create. A developer finishes their work, creates a PR, and then waits, breaking their concentration to start another task.  The DoD check within `tbdflow` was born from this observation: How can we get the quality benefits of a review without the friction of a formal PR process?

### Philosophy: A TBD-Native quality gate

The goal of the DoD check is to be a lightweight, developer-centric quality gate that aligns with the principles of TBD and Continuous Integration.

It achieves this by shifting the quality check "left," moving it from a slow, asynchronous review process to an immediate, synchronous step right at the moment of commit.

* **It's a conversation, not a gatekeeper:** The interactive checklist is a conversation the developer has with the team's shared standards. It's a quick "Did you remember to...?" nudge, not a formal approval process.

* **It lives with the code:** By defining the checklist in a version-controlled `.dod.yml` file, the team's Definition of Done becomes a living artifact that evolves with the codebase, not a forgotten page in Confluence.

* **It Prioritises flow:** The entire process is designed to be completed in seconds, directly in the terminal. It provides immediate feedback without the context switch of navigating a UI, waiting for CI, or pinging teammates for a review.

### User Stories

Here are a few examples of how this feature supports the team's workflow:

**As a Developer, I want to commit my work to `main` with confidence, so that I don't have to worry about forgetting a key quality step.**

* _Scenario:_ I've just finished a small bug fix. I run `tbdflow commit`. An interactive checklist appears, reminding me to run the unit tests and update the code comments. I confirm I've done so and my code is committed, secure in the knowledge that I've followed our team's standards.

**As a Tech Lead, I want to ensure our team consistently follows our Definition of Done, so that we can maintain a high-quality, stable main branch.**

* _Scenario:_ Our team agrees to add a new security check to our DoD. I update the `.dod.yml` file and commit it. Now, every developer on the team will automatically see the new item on their pre-commit checklist, ensuring the new standard is adopted immediately and consistently.

**As a New Team Member, I want a simple way to learn and follow the team's quality standards, so that I can contribute high-quality code from day one.**

* _Scenario:_ It's my first week on the team. I'm not yet familiar with all the team's NFRs or documentation standards. The `tbdflow` commit command guides me through the checklist on my very first commit, providing a safety net and an interactive learning tool that helps me get up to speed quickly.

