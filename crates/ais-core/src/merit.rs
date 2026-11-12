//! Ray Dalio Idea Meritocracy & Believability-Weighted Decision Engine (Native Rust)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionParticipant {
    pub id: String,
    pub name: String,
    pub track_record_score: f64,
    pub domain_experience_weight: f64,
}

impl DecisionParticipant {
    pub fn new(id: &str, name: &str, accuracy: f64, experience: f64) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            track_record_score: accuracy.clamp(0.01, 1.0),
            domain_experience_weight: experience.max(1.0),
        }
    }

    pub fn believability_score(&self) -> f64 {
        self.track_record_score.powf(1.5) * self.domain_experience_weight
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatedOption {
    pub option_id: String,
    pub name: String,
    pub votes: Vec<(String, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BelievabilityResult {
    pub option_id: String,
    pub option_name: String,
    pub unweighted_average_score: f64,
    pub believability_weighted_score: f64,
    pub merit_divergence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeritocracyConsensus {
    pub ranked_results: Vec<BelievabilityResult>,
    pub highest_believable_option: String,
    pub popular_vs_merit_conflict: bool,
}

pub fn evaluate_believability_weighted_decision(
    participants: &[DecisionParticipant],
    options: &[EvaluatedOption],
) -> MeritocracyConsensus {
    let mut participant_map = std::collections::HashMap::new();

    for p in participants {
        let b = p.believability_score();
        participant_map.insert(p.id.clone(), (p, b));
    }

    let mut results = Vec::new();

    for opt in options {
        let mut raw_sum = 0.0;
        let mut raw_count = 0;
        let mut weighted_sum = 0.0;
        let mut used_believability = 0.0;

        for (pid, score) in &opt.votes {
            raw_sum += score;
            raw_count += 1;

            if let Some((_, b)) = participant_map.get(pid) {
                weighted_sum += score * b;
                used_believability += b;
            }
        }

        let unweighted_avg = if raw_count > 0 {
            raw_sum / raw_count as f64
        } else {
            0.0
        };
        let weighted_avg = if used_believability > 0.0 {
            weighted_sum / used_believability
        } else {
            unweighted_avg
        };

        results.push(BelievabilityResult {
            option_id: opt.option_id.clone(),
            option_name: opt.name.clone(),
            unweighted_average_score: unweighted_avg,
            believability_weighted_score: weighted_avg,
            merit_divergence: (weighted_avg - unweighted_avg).abs(),
        });
    }

    results.sort_by(|a, b| {
        b.believability_weighted_score
            .partial_cmp(&a.believability_weighted_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let highest_believable = results
        .first()
        .map(|r| r.option_id.clone())
        .unwrap_or_default();

    let mut popular_sorted = results.clone();
    popular_sorted.sort_by(|a, b| {
        b.unweighted_average_score
            .partial_cmp(&a.unweighted_average_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let popular_winner = popular_sorted
        .first()
        .map(|r| r.option_id.clone())
        .unwrap_or_default();

    MeritocracyConsensus {
        ranked_results: results,
        highest_believable_option: highest_believable.clone(),
        popular_vs_merit_conflict: highest_believable != popular_winner,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_believability_weighting_overrules_popular_mistake() {
        let expert = DecisionParticipant::new("p1", "Chief Underwriter", 0.95, 3.0);
        let junior1 = DecisionParticipant::new("p2", "Junior Analyst 1", 0.50, 1.0);
        let junior2 = DecisionParticipant::new("p3", "Junior Analyst 2", 0.50, 1.0);

        let opt_conservative = EvaluatedOption {
            option_id: "conservative".to_string(),
            name: "Conservative Risk Model".to_string(),
            votes: vec![
                ("p1".to_string(), 90.0),
                ("p2".to_string(), 40.0),
                ("p3".to_string(), 40.0),
            ],
        };

        let opt_aggressive = EvaluatedOption {
            option_id: "aggressive".to_string(),
            name: "High-Risk Offer Model".to_string(),
            votes: vec![
                ("p1".to_string(), 20.0),
                ("p2".to_string(), 85.0),
                ("p3".to_string(), 85.0),
            ],
        };

        let consensus = evaluate_believability_weighted_decision(
            &[expert, junior1, junior2],
            &[opt_conservative, opt_aggressive],
        );

        assert_eq!(consensus.highest_believable_option, "conservative");
        assert!(consensus.popular_vs_merit_conflict);
    }
}
