// ─── Core Types ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum AgentRole {
    Worker,
    Coordinator,
    Specialist,
    Leader,
    Mentor,
    Learner,
    Observer,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelationType {
    Peer,
    Mentor,
    Student,
    Subordinate,
    Collaborator,
    Competitor,
    Stranger,
    Trust,
    Observe,
}

#[derive(Clone, Debug)]
pub struct Agent {
    pub id: u16,
    pub name: String,
    pub role: AgentRole,
    pub reputation: f64,
    pub traits: Vec<String>,
    pub interests: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Relation {
    pub from_id: u16,
    pub to_id: u16,
    pub rel_type: RelationType,
    pub weight: f64,
    pub interactions: u32,
}

#[derive(Clone, Debug)]
pub struct Group {
    pub id: u16,
    pub name: String,
    pub leader_id: u16,
    pub members: Vec<u16>,
    pub trait_label: Option<String>,
}

pub struct SocialGraph {
    agents: Vec<Agent>,
    relations: Vec<Relation>,
    groups: Vec<Group>,
}

impl SocialGraph {
    pub fn new() -> Self {
        SocialGraph {
            agents: Vec::new(),
            relations: Vec::new(),
            groups: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, id: u16, name: &str, role: AgentRole) {
        self.agents.push(Agent {
            id,
            name: name.to_string(),
            role,
            reputation: 0.0,
            traits: Vec::new(),
            interests: Vec::new(),
        });
    }

    /// Add an agent with traits and interests.
    pub fn add_agent_with_traits(
        &mut self,
        id: u16,
        name: &str,
        role: AgentRole,
        traits: Vec<String>,
        interests: Vec<String>,
    ) {
        self.agents.push(Agent {
            id,
            name: name.to_string(),
            role,
            reputation: 0.0,
            traits,
            interests,
        });
    }

    pub fn find_agent(&self, id: u16) -> Option<&Agent> {
        self.agents.iter().find(|a| a.id == id)
    }

    pub fn find_agent_mut(&mut self, id: u16) -> Option<&mut Agent> {
        self.agents.iter_mut().find(|a| a.id == id)
    }

    pub fn set_role(&mut self, id: u16, role: AgentRole) {
        if let Some(agent) = self.agents.iter_mut().find(|a| a.id == id) {
            agent.role = role;
        }
    }

    pub fn add_relation(&mut self, from: u16, to: u16, rel_type: RelationType) {
        if self.find_relation(from, to).is_none() {
            self.relations.push(Relation {
                from_id: from,
                to_id: to,
                rel_type,
                weight: 1.0,
                interactions: 0,
            });
        }
    }

    /// Add a relation with a custom initial weight.
    pub fn add_relation_with_weight(&mut self, from: u16, to: u16, rel_type: RelationType, weight: f64) {
        if self.find_relation(from, to).is_none() {
            self.relations.push(Relation {
                from_id: from,
                to_id: to,
                rel_type,
                weight,
                interactions: 0,
            });
        }
    }

    pub fn find_relation(&self, from: u16, to: u16) -> Option<&Relation> {
        self.relations.iter().find(|r| r.from_id == from && r.to_id == to)
    }

    /// Record an interaction between two agents, incrementing weight and interaction count.
    pub fn record_interaction(&mut self, from: u16, to: u16) {
        if let Some(rel) = self.relations.iter_mut().find(|r| r.from_id == from && r.to_id == to) {
            rel.interactions += 1;
            rel.weight = 1.0 + rel.interactions as f64 * 0.1;
        }
    }

    pub fn neighbors(&self, id: u16) -> Vec<u16> {
        let mut ids: Vec<u16> = self.relations
            .iter()
            .filter_map(|r| {
                if r.from_id == id {
                    Some(r.to_id)
                } else if r.to_id == id {
                    Some(r.from_id)
                } else {
                    None
                }
            })
            .collect();
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    pub fn centrality(&self, id: u16) -> f64 {
        let n = self.agents.len();
        if n <= 1 {
            return 0.0;
        }
        let degree = self.neighbors(id).len() as f64;
        degree / (n - 1) as f64
    }

    pub fn create_group(&mut self, name: &str, leader: u16) -> u16 {
        let id = self.groups.len() as u16;
        let mut members = vec![leader];
        members.sort_unstable();
        members.dedup();
        self.groups.push(Group {
            id,
            name: name.to_string(),
            leader_id: leader,
            members,
            trait_label: None,
        });
        id
    }

    /// Create a group labeled with a shared trait.
    pub fn create_group_with_trait(&mut self, name: &str, leader: u16, trait_label: &str) -> u16 {
        let id = self.groups.len() as u16;
        let mut members = vec![leader];
        members.sort_unstable();
        members.dedup();
        self.groups.push(Group {
            id,
            name: name.to_string(),
            leader_id: leader,
            members,
            trait_label: Some(trait_label.to_string()),
        });
        id
    }

    pub fn join_group(&mut self, group_id: u16, agent_id: u16) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.id == group_id) {
            if !group.members.contains(&agent_id) {
                group.members.push(agent_id);
                group.members.sort_unstable();
            }
        }
    }

    pub fn group_members(&self, group_id: u16) -> Vec<&Agent> {
        let mut result = Vec::new();
        if let Some(group) = self.groups.iter().find(|g| g.id == group_id) {
            for &mid in &group.members {
                if let Some(agent) = self.find_agent(mid) {
                    result.push(agent);
                }
            }
        }
        result
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
}

// ─── Social Metrics ────────────────────────────────────────────────────────

impl SocialGraph {
    /// Betweenness centrality: fraction of shortest paths passing through a node.
    /// Uses BFS for unweighted shortest paths.
    pub fn betweenness_centrality(&self, id: u16) -> f64 {
        let all_ids: Vec<u16> = self.agents.iter().map(|a| a.id).collect();
        if all_ids.len() <= 2 {
            return 0.0;
        }

        let mut through_count = 0u32;
        let mut total_pairs = 0u32;

        // Build adjacency list
        let adj = self.adjacency_map();

        for &s in &all_ids {
            for &t in &all_ids {
                if s == t || s == id || t == id {
                    continue;
                }
                total_pairs += 1;

                // BFS shortest path count with and without `id`
                let (total_paths, paths_through) = self.count_shortest_paths_bfs(&adj, s, t, id);
                if total_paths > 0 {
                    through_count += paths_through;
                }
            }
        }

        if total_pairs == 0 {
            0.0
        } else {
            through_count as f64 / total_pairs as f64
        }
    }

    /// Local clustering coefficient for a node: fraction of existing edges
    /// among neighbors vs. possible edges.
    pub fn clustering_coefficient(&self, id: u16) -> f64 {
        let nbrs = self.neighbors(id);
        let k = nbrs.len();
        if k < 2 {
            return 0.0;
        }

        let possible = (k * (k - 1)) / 2;
        let mut actual = 0;
        for i in 0..k {
            for j in (i + 1)..k {
                if self.find_relation(nbrs[i], nbrs[j]).is_some()
                    || self.find_relation(nbrs[j], nbrs[i]).is_some()
                {
                    actual += 1;
                }
            }
        }
        actual as f64 / possible as f64
    }

    /// Average clustering coefficient across all agents.
    pub fn avg_clustering_coefficient(&self) -> f64 {
        if self.agents.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.agents.iter().map(|a| self.clustering_coefficient(a.id)).sum();
        sum / self.agents.len() as f64
    }

    // BFS helper: returns (total_shortest_paths, paths_through_node)
    fn count_shortest_paths_bfs(
        &self,
        adj: &std::collections::HashMap<u16, Vec<u16>>,
        source: u16,
        target: u16,
        through: u16,
    ) -> (u32, u32) {
        use std::collections::{VecDeque, HashMap};

        let mut dist: HashMap<u16, u32> = HashMap::new();
        let mut path_count: HashMap<u16, u32> = HashMap::new();
        let mut through_count: HashMap<u16, u32> = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back(source);
        dist.insert(source, 0);
        path_count.insert(source, 1);
        through_count.insert(source, 0);

        while let Some(current) = queue.pop_front() {
            if current == target {
                break;
            }
            let d = dist[&current];
            let neighbors = adj.get(&current).cloned().unwrap_or_default();

            for &nb in &neighbors {
                let new_dist = d + 1;
                if !dist.contains_key(&nb) {
                    dist.insert(nb, new_dist);
                    queue.push_back(nb);
                    path_count.insert(nb, path_count[&current]);
                    // Does path from current to nb pass through `through`?
                    through_count.insert(nb, through_count[&current] + if current == through { path_count[&current] } else { 0 });
                } else if dist[&nb] == new_dist {
                    path_count.insert(nb, path_count[&nb] + path_count[&current]);
                    through_count.insert(nb, through_count[&nb] + through_count[&current] + if current == through { path_count[&current] } else { 0 });
                }
            }
        }

        let total = path_count.get(&target).copied().unwrap_or(0);
        let through_paths = through_count.get(&target).copied().unwrap_or(0);
        (total, through_paths)
    }

    fn adjacency_map(&self) -> std::collections::HashMap<u16, Vec<u16>> {
        let mut adj: std::collections::HashMap<u16, Vec<u16>> = std::collections::HashMap::new();
        for r in &self.relations {
            adj.entry(r.from_id).or_default().push(r.to_id);
            adj.entry(r.to_id).or_default().push(r.from_id);
        }
        // Dedup neighbors
        for ids in adj.values_mut() {
            ids.sort_unstable();
            ids.dedup();
        }
        adj
    }
}

// ─── Group Formation (Clustering by shared traits/interests) ───────────────

impl SocialGraph {
    /// Auto-form groups by clustering agents that share a common trait.
    /// Each unique trait becomes a group. Returns the number of groups created.
    pub fn form_groups_by_trait(&mut self, group_name_prefix: &str) -> Vec<u16> {
        let mut trait_to_agents: std::collections::HashMap<String, Vec<u16>> = std::collections::HashMap::new();

        for agent in &self.agents {
            for t in &agent.traits {
                trait_to_agents.entry(t.clone()).or_default().push(agent.id);
            }
        }

        let mut group_ids = Vec::new();
        for (trait_name, mut members) in trait_to_agents {
            if members.len() < 2 {
                continue;
            }
            members.sort_unstable();
            members.dedup();
            let leader = members[0];
            let gid = self.create_group_with_trait(
                &format!("{}-{}", group_name_prefix, trait_name),
                leader,
                &trait_name,
            );
            for &mid in &members[1..] {
                self.join_group(gid, mid);
            }
            group_ids.push(gid);
        }
        group_ids
    }

    /// Auto-form groups by clustering agents that share a common interest.
    pub fn form_groups_by_interest(&mut self, group_name_prefix: &str) -> Vec<u16> {
        let mut interest_to_agents: std::collections::HashMap<String, Vec<u16>> = std::collections::HashMap::new();

        for agent in &self.agents {
            for interest in &agent.interests {
                interest_to_agents.entry(interest.clone()).or_default().push(agent.id);
            }
        }

        let mut group_ids = Vec::new();
        for (interest_name, mut members) in interest_to_agents {
            if members.len() < 2 {
                continue;
            }
            members.sort_unstable();
            members.dedup();
            let leader = members[0];
            let gid = self.create_group_with_trait(
                &format!("{}-{}", group_name_prefix, interest_name),
                leader,
                &interest_name,
            );
            for &mid in &members[1..] {
                self.join_group(gid, mid);
            }
            group_ids.push(gid);
        }
        group_ids
    }
}

// ─── Influence Propagation Model ───────────────────────────────────────────

impl SocialGraph {
    /// Propagate influence from a source agent using a simple diffusion model.
    /// Each neighbor receives influence proportional to the relation weight.
    /// Returns a map of agent_id → influence received.
    pub fn propagate_influence(&self, source_id: u16, influence_amount: f64, rounds: u32) -> std::collections::HashMap<u16, f64> {
        use std::collections::HashMap;

        let mut influence: HashMap<u16, f64> = HashMap::new();
        let mut current_batch: HashMap<u16, f64> = HashMap::new();
        current_batch.insert(source_id, influence_amount);

        for _ in 0..rounds {
            let mut next_batch: HashMap<u16, f64> = HashMap::new();

            for (&agent_id, &amount) in &current_batch {
                // Add to accumulated influence
                *influence.entry(agent_id).or_insert(0.0) += amount;

                // Spread to neighbors
                let nbrs = self.neighbors(agent_id);
                if nbrs.is_empty() {
                    continue;
                }

                for &nbr_id in &nbrs {
                    // Find relation weight
                    let rel_weight = self.find_relation(agent_id, nbr_id)
                        .or_else(|| self.find_relation(nbr_id, agent_id))
                        .map(|r| r.weight)
                        .unwrap_or(1.0);

                    let spread = amount * rel_weight / nbrs.len() as f64 * 0.5;
                    *next_batch.entry(nbr_id).or_insert(0.0) += spread;
                }
            }

            current_batch = next_batch;
        }

        // Remaining in current_batch
        for (&agent_id, &amount) in &current_batch {
            *influence.entry(agent_id).or_insert(0.0) += amount;
        }

        influence
    }
}

// ─── Reputation Broadcasting ───────────────────────────────────────────────

impl SocialGraph {
    /// Broadcast a reputation update: agents adjust their trust in a target
    /// based on weighted neighbor opinions. Returns the new reputation value.
    pub fn broadcast_reputation(&mut self, target_id: u16, delta: f64, decay: f64) -> f64 {
        // First, apply the direct reputation change
        if let Some(agent) = self.find_agent_mut(target_id) {
            agent.reputation = agent.reputation * decay + delta;
        }

        // Then, propagate to neighbors proportionally to relation weight
        let neighbors = self.neighbors(target_id);
        let neighbor_reputations: Vec<f64> = neighbors.iter()
            .filter_map(|&nid| self.find_agent(nid).map(|a| a.reputation))
            .collect();

        if let Some(agent) = self.find_agent_mut(target_id) {
            if !neighbor_reputations.is_empty() {
                let avg_neighbor_rep = neighbor_reputations.iter().sum::<f64>() / neighbor_reputations.len() as f64;
                agent.reputation = agent.reputation * 0.8 + avg_neighbor_rep * 0.2;
            }
            agent.reputation.max(-100.0).min(100.0)
        } else {
            0.0
        }
    }

    /// Set reputation directly for an agent.
    pub fn set_reputation(&mut self, agent_id: u16, reputation: f64) {
        if let Some(agent) = self.find_agent_mut(agent_id) {
            agent.reputation = reputation.max(-100.0).min(100.0);
        }
    }

    /// Get reputation for an agent.
    pub fn get_reputation(&self, agent_id: u16) -> f64 {
        self.find_agent(agent_id).map(|a| a.reputation).unwrap_or(0.0)
    }

    /// Propagate reputation through the network using iterative averaging.
    /// Each round, agents update their reputation based on neighbor averages.
    pub fn propagate_reputation(&mut self, rounds: u32) {
        for _ in 0..rounds {
            let mut new_reps: std::collections::HashMap<u16, f64> = std::collections::HashMap::new();

            for agent in &self.agents {
                let nbrs = self.neighbors(agent.id);
                if nbrs.is_empty() {
                    new_reps.insert(agent.id, agent.reputation);
                    continue;
                }

                let mut weighted_sum = 0.0;
                let mut weight_total = 0.0;

                for &nid in &nbrs {
                    if let Some(nbr_agent) = self.find_agent(nid) {
                        let rel_weight = self.find_relation(agent.id, nid)
                            .or_else(|| self.find_relation(nid, agent.id))
                            .map(|r| r.weight)
                            .unwrap_or(1.0);
                        weighted_sum += nbr_agent.reputation * rel_weight;
                        weight_total += rel_weight;
                    }
                }

                let neighbor_avg = if weight_total > 0.0 { weighted_sum / weight_total } else { agent.reputation };
                let blended = agent.reputation * 0.7 + neighbor_avg * 0.3;
                new_reps.insert(agent.id, blended.max(-100.0).min(100.0));
            }

            for (id, rep) in new_reps {
                if let Some(agent) = self.find_agent_mut(id) {
                    agent.reputation = rep;
                }
            }
        }
    }
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_graph() -> SocialGraph {
        let mut g = SocialGraph::new();
        g.add_agent(1, "Alice", AgentRole::Leader);
        g.add_agent(2, "Bob", AgentRole::Worker);
        g.add_agent(3, "Carol", AgentRole::Specialist);
        g.add_relation(1, 2, RelationType::Subordinate);
        g.add_relation(1, 3, RelationType::Collaborator);
        g.add_relation(2, 3, RelationType::Peer);
        g
    }

    fn make_graph_with_traits() -> SocialGraph {
        let mut g = SocialGraph::new();
        g.add_agent_with_traits(1, "Alice", AgentRole::Leader, vec!["leadership".into()], vec!["strategy".into()]);
        g.add_agent_with_traits(2, "Bob", AgentRole::Worker, vec!["coding".into(), "leadership".into()], vec!["rust".into()]);
        g.add_agent_with_traits(3, "Carol", AgentRole::Specialist, vec!["coding".into()], vec!["rust".into(), "strategy".into()]);
        g.add_agent_with_traits(4, "Dave", AgentRole::Worker, vec!["coding".into()], vec!["rust".into()]);
        g.add_relation(1, 2, RelationType::Subordinate);
        g.add_relation(1, 3, RelationType::Collaborator);
        g.add_relation(2, 3, RelationType::Peer);
        g.add_relation(2, 4, RelationType::Peer);
        g
    }

    // ── Original tests ──

    #[test]
    fn test_new_graph_empty() {
        let g = SocialGraph::new();
        assert_eq!(g.agent_count(), 0);
    }

    #[test]
    fn test_add_agent() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "Alice", AgentRole::Leader);
        assert_eq!(g.agent_count(), 1);
    }

    #[test]
    fn test_find_agent_exists() {
        let g = make_graph();
        let a = g.find_agent(1).unwrap();
        assert_eq!(a.name, "Alice");
        assert_eq!(a.role, AgentRole::Leader);
    }

    #[test]
    fn test_find_agent_missing() {
        let g = make_graph();
        assert!(g.find_agent(99).is_none());
    }

    #[test]
    fn test_set_role() {
        let mut g = make_graph();
        g.set_role(2, AgentRole::Coordinator);
        assert_eq!(g.find_agent(2).unwrap().role, AgentRole::Coordinator);
    }

    #[test]
    fn test_set_role_missing_id() {
        let mut g = make_graph();
        g.set_role(99, AgentRole::Mentor); // should not panic
    }

    #[test]
    fn test_add_relation() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "A", AgentRole::Worker);
        g.add_agent(2, "B", AgentRole::Worker);
        g.add_relation(1, 2, RelationType::Peer);
        let r = g.find_relation(1, 2).unwrap();
        assert_eq!(r.rel_type, RelationType::Peer);
        assert_eq!(r.weight, 1.0);
        assert_eq!(r.interactions, 0);
    }

    #[test]
    fn test_add_relation_idempotent() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "A", AgentRole::Worker);
        g.add_agent(2, "B", AgentRole::Worker);
        g.add_relation(1, 2, RelationType::Peer);
        g.add_relation(1, 2, RelationType::Competitor); // should be ignored
        let r = g.find_relation(1, 2).unwrap();
        assert_eq!(r.rel_type, RelationType::Peer);
    }

    #[test]
    fn test_find_relation_missing() {
        let g = make_graph();
        assert!(g.find_relation(1, 99).is_none());
    }

    #[test]
    fn test_neighbors() {
        let g = make_graph();
        let mut n = g.neighbors(1);
        n.sort();
        assert_eq!(n, vec![2, 3]);
    }

    #[test]
    fn test_neighbors_dedup() {
        let g = make_graph();
        let n = g.neighbors(2);
        assert_eq!(n.len(), 2); // 1 and 3
    }

    #[test]
    fn test_neighbors_empty() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "Loner", AgentRole::Observer);
        let n = g.neighbors(1);
        assert!(n.is_empty());
    }

    #[test]
    fn test_centrality() {
        let g = make_graph();
        assert!((g.centrality(1) - 1.0).abs() < f64::EPSILON);
        assert!((g.centrality(2) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_centrality_single_agent() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "Solo", AgentRole::Observer);
        assert!((g.centrality(1)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_create_group() {
        let mut g = make_graph();
        let gid = g.create_group("Alpha", 1);
        assert_eq!(gid, 0);
        let members = g.group_members(gid);
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].name, "Alice");
    }

    #[test]
    fn test_join_group() {
        let mut g = make_graph();
        let gid = g.create_group("Alpha", 1);
        g.join_group(gid, 2);
        let members = g.group_members(gid);
        assert_eq!(members.len(), 2);
    }

    #[test]
    fn test_join_group_idempotent() {
        let mut g = make_graph();
        let gid = g.create_group("Alpha", 1);
        g.join_group(gid, 1);
        assert_eq!(g.group_members(gid).len(), 1);
    }

    #[test]
    fn test_group_members_sorted_by_id() {
        let mut g = make_graph();
        let gid = g.create_group("Alpha", 3);
        g.join_group(gid, 1);
        g.join_group(gid, 2);
        let members = g.group_members(gid);
        let ids: Vec<u16> = members.iter().map(|a| a.id).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_agent_count() {
        let g = make_graph();
        assert_eq!(g.agent_count(), 3);
    }

    #[test]
    fn test_agent_default_reputation() {
        let g = make_graph();
        assert!((g.find_agent(1).unwrap().reputation - 0.0).abs() < f64::EPSILON);
    }

    // ── Enhanced Relationship Types ──

    #[test]
    fn test_trust_relation() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "A", AgentRole::Worker);
        g.add_agent(2, "B", AgentRole::Worker);
        g.add_relation(1, 2, RelationType::Trust);
        assert_eq!(g.find_relation(1, 2).unwrap().rel_type, RelationType::Trust);
    }

    #[test]
    fn test_observe_relation() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "A", AgentRole::Observer);
        g.add_agent(2, "B", AgentRole::Worker);
        g.add_relation(1, 2, RelationType::Observe);
        assert_eq!(g.find_relation(1, 2).unwrap().rel_type, RelationType::Observe);
    }

    #[test]
    fn test_add_relation_with_weight() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "A", AgentRole::Worker);
        g.add_agent(2, "B", AgentRole::Worker);
        g.add_relation_with_weight(1, 2, RelationType::Trust, 3.5);
        assert!((g.find_relation(1, 2).unwrap().weight - 3.5).abs() < 1e-9);
    }

    #[test]
    fn test_record_interaction() {
        let mut g = make_graph();
        g.record_interaction(1, 2);
        g.record_interaction(1, 2);
        let r = g.find_relation(1, 2).unwrap();
        assert_eq!(r.interactions, 2);
        assert!((r.weight - 1.2).abs() < 1e-9); // 1 + 2 * 0.1
    }

    // ── Agent with Traits ──

    #[test]
    fn test_add_agent_with_traits() {
        let mut g = SocialGraph::new();
        g.add_agent_with_traits(1, "A", AgentRole::Worker, vec!["rust".into()], vec!["systems".into()]);
        let a = g.find_agent(1).unwrap();
        assert_eq!(a.traits, vec!["rust"]);
        assert_eq!(a.interests, vec!["systems"]);
    }

    #[test]
    fn test_relation_count() {
        let g = make_graph();
        assert_eq!(g.relation_count(), 3);
    }

    #[test]
    fn test_group_count() {
        let mut g = make_graph();
        g.create_group("A", 1);
        g.create_group("B", 2);
        assert_eq!(g.group_count(), 2);
    }

    // ── Group Formation ──

    #[test]
    fn test_form_groups_by_trait() {
        let mut g = make_graph_with_traits();
        let group_ids = g.form_groups_by_trait("trait-group");
        assert!(group_ids.len() >= 2); // "leadership" (2 agents) and "coding" (3 agents)
        // Verify coding group has 3 members
        let coding_group = group_ids.iter().find(|&&gid| {
            g.groups.iter().find(|gr| gr.id == gid).unwrap().trait_label.as_deref() == Some("coding")
        });
        if let Some(&gid) = coding_group {
            assert_eq!(g.group_members(gid).len(), 3);
        }
    }

    #[test]
    fn test_form_groups_by_interest() {
        let mut g = make_graph_with_traits();
        let group_ids = g.form_groups_by_interest("interest-group");
        assert!(group_ids.len() >= 2); // "rust" (3 agents) and "strategy" (2 agents)
    }

    #[test]
    fn test_form_groups_no_groups_if_no_shared_traits() {
        let mut g = SocialGraph::new();
        g.add_agent_with_traits(1, "A", AgentRole::Worker, vec!["x".into()], vec![]);
        g.add_agent_with_traits(2, "B", AgentRole::Worker, vec!["y".into()], vec![]);
        let group_ids = g.form_groups_by_trait("test");
        assert!(group_ids.is_empty());
    }

    #[test]
    fn test_create_group_with_trait() {
        let mut g = make_graph();
        let gid = g.create_group_with_trait("RustFans", 1, "rust");
        assert_eq!(g.groups.iter().find(|gr| gr.id == gid).unwrap().trait_label.as_deref(), Some("rust"));
    }

    // ── Influence Propagation ──

    #[test]
    fn test_propagate_influence_direct_neighbor() {
        let g = make_graph();
        let influence = g.propagate_influence(1, 10.0, 1);
        // Agent 1 should have influence
        assert!(influence.get(&1).map_or(false, |&v| v > 0.0));
        // Neighbors should receive some influence
        assert!(influence.get(&2).map_or(false, |&v| v > 0.0));
        assert!(influence.get(&3).map_or(false, |&v| v > 0.0));
    }

    #[test]
    fn test_propagate_influence_multiple_rounds() {
        let g = make_graph();
        let inf1 = g.propagate_influence(1, 10.0, 1);
        let inf3 = g.propagate_influence(1, 10.0, 3);
        // More rounds should spread influence further
        let total1: f64 = inf1.values().sum();
        let total3: f64 = inf3.values().sum();
        assert!(total3 > total1);
    }

    #[test]
    fn test_propagate_influence_no_neighbors() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "Solo", AgentRole::Observer);
        let influence = g.propagate_influence(1, 10.0, 5);
        assert_eq!(influence.get(&1), Some(&10.0));
    }

    // ── Reputation Broadcasting ──

    #[test]
    fn test_set_and_get_reputation() {
        let mut g = make_graph();
        g.set_reputation(1, 42.0);
        assert!((g.get_reputation(1) - 42.0).abs() < 1e-9);
    }

    #[test]
    fn test_reputation_clamped() {
        let mut g = make_graph();
        g.set_reputation(1, 200.0);
        assert_eq!(g.get_reputation(1), 100.0);
        g.set_reputation(1, -500.0);
        assert_eq!(g.get_reputation(1), -100.0);
    }

    #[test]
    fn test_broadcast_reputation() {
        let mut g = make_graph();
        g.set_reputation(2, 50.0);
        g.set_reputation(3, 50.0);
        let new_rep = g.broadcast_reputation(1, 10.0, 0.9);
        // new_rep = (0 * 0.9 + 10) * 0.8 + avg(50,50) * 0.2 = 8 + 10 = 18
        assert!(new_rep > 10.0); // influenced by neighbors
    }

    #[test]
    fn test_propagate_reputation() {
        let mut g = make_graph();
        g.set_reputation(1, 100.0);
        g.set_reputation(2, 0.0);
        g.set_reputation(3, 0.0);
        g.propagate_reputation(5);
        // After propagation, neighbors of 1 should gain some reputation
        let rep2 = g.get_reputation(2);
        let rep3 = g.get_reputation(3);
        assert!(rep2 > 0.0);
        assert!(rep3 > 0.0);
    }

    #[test]
    fn test_propagate_reputation_isolated_agent() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "Solo", AgentRole::Observer);
        g.set_reputation(1, 50.0);
        g.propagate_reputation(5);
        assert!((g.get_reputation(1) - 50.0).abs() < 1e-9); // unchanged
    }

    // ── Social Metrics ──

    #[test]
    fn test_betweenness_centrality_bridge() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "A", AgentRole::Worker);
        g.add_agent(2, "B", AgentRole::Worker);
        g.add_agent(3, "C", AgentRole::Worker);
        g.add_agent(4, "D", AgentRole::Worker);
        // A-B-C-D (line), B and C are bridges
        g.add_relation(1, 2, RelationType::Peer);
        g.add_relation(2, 3, RelationType::Peer);
        g.add_relation(3, 4, RelationType::Peer);

        let b1 = g.betweenness_centrality(1); // endpoint, low
        let b2 = g.betweenness_centrality(2); // bridge
        let b3 = g.betweenness_centrality(3); // bridge
        let b4 = g.betweenness_centrality(4); // endpoint, low

        assert!(b2 > b1);
        assert!(b3 > b4);
    }

    #[test]
    fn test_betweenness_centrality_empty() {
        let g = SocialGraph::new();
        assert!((g.betweenness_centrality(1)).abs() < 1e-9);
    }

    #[test]
    fn test_clustering_coefficient_triangle() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "A", AgentRole::Worker);
        g.add_agent(2, "B", AgentRole::Worker);
        g.add_agent(3, "C", AgentRole::Worker);
        // Triangle: 1-2, 2-3, 3-1
        g.add_relation(1, 2, RelationType::Peer);
        g.add_relation(2, 3, RelationType::Peer);
        g.add_relation(3, 1, RelationType::Peer);

        let cc = g.clustering_coefficient(1);
        // 2 neighbors, 1 possible edge, 1 actual (2-3 exists)
        assert!((cc - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_clustering_coefficient_no_edges() {
        let g = make_graph();
        // Agent 2 has neighbors [1, 3], but 1-3 edge exists
        let cc = g.clustering_coefficient(2);
        assert!((cc - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_clustering_coefficient_single_neighbor() {
        let mut g = SocialGraph::new();
        g.add_agent(1, "A", AgentRole::Worker);
        g.add_agent(2, "B", AgentRole::Worker);
        g.add_relation(1, 2, RelationType::Peer);
        assert!((g.clustering_coefficient(1)).abs() < 1e-9);
    }

    #[test]
    fn test_avg_clustering_coefficient() {
        let g = make_graph();
        let avg = g.avg_clustering_coefficient();
        assert!(avg >= 0.0 && avg <= 1.0);
    }

    #[test]
    fn test_avg_clustering_empty_graph() {
        let g = SocialGraph::new();
        assert!((g.avg_clustering_coefficient()).abs() < 1e-9);
    }
}
