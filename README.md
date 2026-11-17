# tempo-types

The small shared value vocabulary for Tempo 2. Distinct newtypes prevent passing
a target ID where an attempt, dataset, submission or event ID is expected.
Content versions, response revisions, miss revisions and configured Study days
are also distinct types. `StudyTime` is supplied by the application; `Grade`
describes Good or Again without deciding its scheduling effect.

This crate has no dependency on other Tempo packages. It owns no attempts,
catalogs, queue candidates, scheduler state, SQL rows or UI wire envelopes.
Those rich records belong to their respective domain/application crates.

The types derive serialization for explicit persistence and transport adapters.
Semantic validation belongs to the owning command/domain boundary. Run
`cargo check -p tempo-types` within the development workspace.
