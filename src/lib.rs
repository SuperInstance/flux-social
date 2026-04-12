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
}

#[derive(Clone, Debug)]
pub struct Agent {
    pub id: u16,
    pub name: String,
    pub role: AgentRole,
    pub reputation: f64,
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
        });
    }

    pub fn find_agent(&self, id: u16) -> Option<&Agent> {
        self.agents.iter().find(|a| a.id == id)
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

    pub fn find_relation(&self, from: u16, to: u16) -> Option<&Relation> {
        self.relations.iter().find(|r| r.from_id == from && r.to_id == to)
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
}

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
        // 1->2 exists, no reverse; test dedup path
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
        // 3 agents, agent 1 has degree 2, centrality = 2/(3-1) = 1.0
        assert!((g.centrality(1) - 1.0).abs() < f64::EPSILON);
        // agent 2: neighbors [1,3] -> degree 2 -> 1.0
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
}
