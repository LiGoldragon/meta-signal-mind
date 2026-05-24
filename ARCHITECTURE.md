# owner-signal-mind — architecture

*Owner-only Signal contract for PersonaMind policy and configuration.*

---

## 0 · TL;DR

`owner-signal-mind` is the policy signal for PersonaMind.
PersonaSpirit owns PersonaMind, so this contract is the typed
authority surface Spirit uses to configure Mind's policy state.

Ordinary mind graph, work graph, query, and subscription traffic stays
in `signal-mind`. Runtime actors, policy evaluation, store
tables, socket binding, and command lowering live in `mind`.
This repository owns only the owner-only wire vocabulary.

The initial surface is deliberately small:

- `Configure(Configuration)` mutates owner-controlled policy state.
- `Inspect(Inspection)` reads owner-only policy state.

## 1 · Contract Surface

| Operation | Projected Sema class | Meaning |
|---|---|---|
| `Configure` | `Mutate` | Apply a Spirit-issued policy configuration to PersonaMind. |
| `Inspect` | `Match` | Read owner-only policy state. |

| Reply | Meaning |
|---|---|
| `Configured` | The daemon accepted and recorded a new policy revision. |
| `PolicySnapshot` | The daemon returned the requested policy state. |
| `ConfigurationRejected` | The configuration was understood but rejected by domain policy. |
| `RequestUnimplemented` | The request is in the contract but not implemented by the current runtime. |

The Sema classes above are daemon-side projections. The wire carries
contract-local operation roots only; there is no public `Mutate` or
`Match` wrapper.

## 2 · Policy Types

`Configuration` is the current policy bundle:

- `AuthorityMode` decides whether Mind only observes, proposes orders,
  or issues authority orders.
- `ChoreographyMode` decides whether Mind records, recommends, or
  decides channel choreography.
- `IntentSynchronizationMode` decides how much Spirit-derived intent
  state Mind receives.

These names are intentionally policy-shaped. They are not working
graph records, not thoughts, and not channel grants.

## 3 · Boundaries

This repo owns:

- owner-only operation roots and payload records;
- owner-only replies and rejection reasons;
- rkyv and NOTA round-trip shape for the policy signal;
- the contract-local `OperationKind` witness emitted by
  `signal_channel!`.

This repo does not own:

- `mind` daemon actors;
- `mind.redb`;
- `bootstrap-policy.nota`;
- ordinary `signal-mind` graph/work/query/subscription traffic;
- router channel grant execution;
- Spirit's runtime logic for deciding what policy to issue;
- CLI argv parsing or socket permissions.

## 4 · Constraints

- The contract exposes owner-only policy operations, not ordinary mind
  graph operations.
- Every operation root is a contract-local verb in verb form.
- The wire shape contains no public Sema wrapper such as `Mutate` or
  `Match`.
- Policy revisions are daemon-minted reply data, not request payloads.
- The contract crate contains no runtime actors, database handles,
  sockets, command execution, or policy evaluation logic.

## 5 · Witness Tests

`tests/round_trip.rs` proves:

- request operations round-trip through `Frame`;
- replies round-trip through `Frame`;
- NOTA request heads are contract-local verbs;
- the public operation exposes a contract-owned `OperationKind` through
  the generated `kind()` method.

## Code Map

```text
src/lib.rs            owner policy types and signal_channel! declaration
tests/round_trip.rs   frame round trips and contract-local operation witnesses
```

## See Also

- `../signal-mind/ARCHITECTURE.md`
- `../mind/ARCHITECTURE.md`
- `../persona-spirit/ARCHITECTURE.md`
- `../signal-frame/ARCHITECTURE.md`
- `../signal-sema/ARCHITECTURE.md`
- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
