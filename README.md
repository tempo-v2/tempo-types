# tempo-types

Small shared values: **which thing**, **which version/day**, and **which outcome**.
This crate owns no attempts, catalogs, queues, learning rules, database rows, or UI
messages. It has no dependency on another Tempo crate.

## Find the concept you need

Every concrete type has its own file. Open that file for its purpose, who supplies
and consumes it, and a `#[cfg(test)]` usage scenario at the bottom.

| Your question | Type / file | Scenario in that file |
| --- | --- | --- |
| Which independent collection of data? | [`DatasetId`](src/identifiers/dataset_id.rs) | The same target in original and restored datasets has separate record keys |
| Which learnable item across many tries? | [`TargetId`](src/identifiers/target_id.rs) | Two attempts contribute to one target's history |
| Which run through that item? | [`AttemptId`](src/identifiers/attempt_id.rs) | Multiple answers share an attempt; a fresh repeat has another ID |
| Which answer command, including transport retries? | [`SubmissionId`](src/identifiers/submission_id.rs) | A retried command finds its original receipt; a new answer does not |
| Which external observation? | [`EventId`](src/identifiers/event_id.rs) | Repeated delivery of one game miss can be recognized |
| Which semantic definition was used? | [`ContentVersion`](src/counters/content_version.rs) | Historical content can be identified after the response changes |
| Which state version did this command read? | [`Revision`](src/counters/revision.rs) | A second command from an old view no longer matches the current revision |
| Which generation of misses did this attempt see? | [`MissRevision`](src/counters/miss_revision.rs) | An attempt issued before a new game miss is distinguishable from one issued afterward |
| Which configured scheduling day? | [`StudyDay`](src/counters/study_day.rs) | Due-day comparison includes yesterday and today, but excludes tomorrow |
| When was this operation accepted? | [`StudyTime`](src/study_time.rs) | Two outcomes can share a Study day while retaining distinct timestamps |
| What terminal outcome occurred? | [`Grade`](src/grade.rs) | A consumer handles Good/Again without knowing the grader or scheduler |

## What the wrappers do—and what their owners do

`AttemptId`, `TargetId`, and `SubmissionId` all contain strings, but Rust will reject
passing one where another is expected. Likewise, a `Revision` cannot accidentally
stand in for a `ContentVersion`. The compile-fail example in
[identifiers.rs](src/identifiers.rs) demonstrates that distinction.

These are value wrappers, not validators. They do not generate unique IDs, enforce
idempotency, advance counters, or read a clock. The application and owning domain
operations enforce those rules. The small examples show how the values are used;
Study, Learning, and Store contain the tests that enforce the complete behavior.

A seven-decision tactic has **one target**, may have **several attempts**, and has
**one submission per answer command**. Retrying an unchanged command after losing
its reply retains the submission ID. A new attempt retains the target ID but gets
a new attempt ID. A semantic edit changes the content version, independently of
the response revision that advances within an attempt.

`Grade` belongs here because both Study and Learning use the same outcome type.
Study can report an outcome without importing Learning; Learning can interpret
that outcome without importing Study. A grade contains no scheduling policy.

## Imports and navigation

Root imports remain available: `use tempo_types::{AttemptId, Grade, Revision};`.
The `identifiers`, `counters`, `study_time`, and `prelude` module paths are preserved.
The grouping files are short indexes; concrete definitions are explicit, without
macros hiding which methods or derives a type has. `StudyDay` stays in `counters`
for path compatibility, but its documentation identifies it as a day coordinate.

## Run the examples

From the assembled sibling-repository workspace:

```sh
cargo test -p tempo-types
cargo test -p tempo-types submission_id::tests
cargo doc -p tempo-types --no-deps --open
```

The source contract uses the existing transparent scalar serialization, derives,
public tuple fields, string conversions, and display behavior. There are no new
runtime dependencies or wire-format changes.
