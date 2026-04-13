# flux-social

> Social graph engine with agents, relationships, groups, and centrality metrics for the FLUX fleet.

## What This Is

`flux-social` is a Rust crate implementing a **social graph** — it models agents with roles (Worker, Coordinator, Leader, Specialist, Mentor, Learner, Observer), typed relationships (Peer, Mentor, Student, Subordinate, Collaborator, Competitor, Stranger), and groups with leaders.

## Role in the FLUX Ecosystem

Fleet coordination requires understanding who's connected to whom. `flux-social` provides the social layer:

- **`flux-trust`** scores agents based on observations; social tracks the relationships
- **`flux-evolve`** uses social structure to determine which behaviors to evolve
- **`flux-simulator`** models fleet communication via the social graph topology
- **`flux-dream-cycle`** schedules tasks based on agent roles and group membership

## Key Features

| Feature | Description |
|---------|-------------|
| **Agent Roles** | 7 distinct roles: Worker, Coordinator, Specialist, Leader, Mentor, Learner, Observer |
| **Typed Relations** | 7 relationship types including Peer, Collaborator, Competitor, Mentor/Student |
| **Centrality** | Degree centrality: `centrality(id)` returns 0.0–1.0 influence score |
| **Groups** | Create named groups with leaders, add/remove members |
| **Neighbor Query** | `neighbors(id)` returns all connected agents |
| **Role Mutation** | `set_role()` promotes or reassigns agents dynamically |

## Quick Start

```rust
use flux_social::{SocialGraph, AgentRole, RelationType};

let mut graph = SocialGraph::new();

// Add agents with roles
graph.add_agent(1, "Oracle1", AgentRole::Leader);
graph.add_agent(2, "Super Z", AgentRole::Worker);
graph.add_agent(3, "Quill", AgentRole::Specialist);

// Define relationships
graph.add_relation(1, 2, RelationType::Subordinate);
graph.add_relation(1, 3, RelationType::Collaborator);
graph.add_relation(2, 3, RelationType::Peer);

// Query
let neighbors = graph.neighbors(1); // [2, 3]
let influence = graph.centrality(1); // 1.0 (connected to everyone)

// Groups
let gid = graph.create_group("Alpha Team", 1);
graph.join_group(gid, 2);
let members = graph.group_members(gid);
```

## Building & Testing

```bash
cargo build
cargo test
```

## Related Fleet Repos

- [`flux-trust`](https://github.com/SuperInstance/flux-trust) — Bayesian trust scoring for agents
- [`flux-evolve`](https://github.com/SuperInstance/flux-evolve) — Behavioral evolution driven by social context
- [`flux-simulator`](https://github.com/SuperInstance/flux-simulator) — Multi-agent fleet simulation
- [`flux-dream-cycle`](https://github.com/SuperInstance/flux-dream-cycle) — Task scheduling by role
- [`flux-memory`](https://github.com/SuperInstance/flux-memory) — Store social graph snapshots

## License

Part of the [SuperInstance](https://github.com/SuperInstance) FLUX fleet.
