# INTENT — meta-signal-mind

*The meta policy wire contract for PersonaMind policy and configuration. Defines
the typed request/reply channel that PersonaSpirit (Mind's owner) uses to
configure Mind's policy state and read privileged policy.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this meta `meta-signal-mind`
contract. Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `mind/INTENT.md`. Ordinary mind graph/work/query/
subscription traffic stays in `signal-mind/INTENT.md`.

## Why this repo exists

`meta-signal-mind` is the **meta policy signal** for PersonaMind.
PersonaSpirit owns PersonaMind, so this contract is the typed authority surface
Spirit uses to configure Mind's policy state. Ordinary mind graph, work graph,
query, and subscription traffic stays in `signal-mind`; runtime actors, policy
evaluation, store tables, socket binding, and command lowering live in `mind`.

## The channel shape

The meta channel carries a deliberately small initial surface:

- **Requests:** `Configure(Configuration)` mutates meta-controlled policy
  state; `Inspect(Inspection)` reads privileged policy state.
- **Replies:** `Configured` (a new policy revision was recorded),
  `PolicySnapshot` (requested policy state returned), `ConfigurationRejected`
  (understood but rejected by domain policy), `RequestUnimplemented` (in the
  contract but not yet implemented by the runtime).

`Configuration` is the policy bundle: `AuthorityMode` (Mind observes, proposes,
or issues authority orders), `ChoreographyMode` (Mind records, recommends, or
decides channel choreography), and `IntentSynchronizationMode` (how much
Spirit-derived intent state Mind receives). These names are intentionally
policy-shaped — not working graph records, not thoughts, not channel grants.

## Constraints

- The contract exposes meta policy operations, not ordinary mind graph
  operations.
- Every operation root is a contract-local verb in verb form. The wire shape
  contains no public Sema wrapper such as `Mutate` or `Match`; the Sema class is
  a daemon-side projection, derived internally.
- Wire enums are closed. No `Unknown` escape hatch.
- This crate carries only typed wire vocabulary, rkyv codecs, optional NOTA
  codecs, and round-trip witnesses — no runtime, no actors, no `tokio`.
- Every operation and reply round-trips through both rkyv frames and NOTA text.

## Non-ownership

This crate does not own:

- `mind` daemon actors or `mind.redb`;
- `bootstrap-policy.nota`;
- ordinary `signal-mind` graph/work/query/subscription traffic;
- router channel grant execution;
- Spirit's runtime logic for deciding what policy to issue;
- CLI argv parsing or socket permissions.

## See also

- `ARCHITECTURE.md` — contract surface, policy types, and closed-enum discipline.
- `../mind/INTENT.md` — daemon-side intent (graph, work, choreography, state).
- `../signal-mind/INTENT.md` — ordinary peer-callable graph/work/query contract.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
