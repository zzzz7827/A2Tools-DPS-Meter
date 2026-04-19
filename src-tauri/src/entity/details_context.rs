use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailsActorSummary {
    pub actor_id: i32,
    pub nickname: String,
    #[serde(default)]
    pub job: String,
    /// Job class prefix ID (e.g. 11=Gladiator) for language-independent storage
    #[serde(default)]
    pub job_id: i32,
    #[serde(default)]
    pub party_heal: i64,
    #[serde(default)]
    pub regen: i64,
    #[serde(default)]
    pub damage_received: i64,
    #[serde(default)]
    pub hits_received: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailsTargetSummary {
    pub target_id: i32,
    #[serde(default)]
    pub target_name: String,
    #[serde(default)]
    pub max_hp: i32,
    #[serde(default)]
    pub battle_time: i64,
    #[serde(default)]
    pub last_damage_time: i64,
    #[serde(default)]
    pub total_damage: i32,
    #[serde(default)]
    pub actor_damage: std::collections::HashMap<i32, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailsContext {
    #[serde(default)]
    pub current_target_id: i32,
    #[serde(default)]
    pub targets: Vec<DetailsTargetSummary>,
    #[serde(default)]
    pub actors: Vec<DetailsActorSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailSkillEntry {
    #[serde(default)]
    pub actor_id: i32,
    #[serde(default)]
    pub code: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub time: i32,
    #[serde(default)]
    pub dmg: i32,
    #[serde(default)]
    pub multi_hit_count: i32,
    #[serde(default)]
    pub multi_hit_damage: i32,
    #[serde(default)]
    pub multi_hit_hits: i32,
    #[serde(default)]
    pub min_dmg: i32,
    #[serde(default)]
    pub max_dmg: i32,
    #[serde(default)]
    pub crit: i32,
    #[serde(default)]
    pub parry: i32,
    #[serde(default)]
    pub back: i32,
    #[serde(default)]
    pub perfect: i32,
    #[serde(default)]
    pub double: i32,
    #[serde(default)]
    pub smite: i32,
    #[serde(default)]
    pub powershard: i32,
    #[serde(default)]
    pub regen: i32,
    #[serde(default)]
    pub job: String,
    #[serde(default)]
    pub is_dot: bool,
    #[serde(default)]
    pub hit_timestamps: Vec<i64>,
    #[serde(default)]
    pub specs: Vec<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingPoint {
    #[serde(default)]
    pub ts_ms: i64,
    #[serde(default)]
    pub ping_ms: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetDetailsResponse {
    pub target_id: i32,
    #[serde(default)]
    pub max_hp: i32,
    #[serde(default)]
    pub total_target_damage: i32,
    #[serde(default)]
    pub battle_time: i64,
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub skills: Vec<DetailSkillEntry>,
    #[serde(default)]
    pub ping_history: Vec<PingPoint>,
}
