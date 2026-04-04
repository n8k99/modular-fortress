//! Relevance scoring for context results
//!
//! Apply time decay, frequency boost, and source weighting.

use chrono::{NaiveDateTime, Utc};
use super::injection::{ContextResult, ContextSource};

/// Configuration for relevance scoring
#[derive(Debug, Clone)]
pub struct RelevanceScorer {
    /// Days before score starts decaying
    pub decay_start_days: f32,
    /// How quickly relevance decays (higher = faster decay)
    pub decay_rate: f32,
    /// Minimum score after decay
    pub min_decay_score: f32,
    /// Source weights (stagehand > general for gig context)
    pub source_weights: SourceWeights,
}

/// Weights for different sources
#[derive(Debug, Clone)]
pub struct SourceWeights {
    pub daily_log: f32,
    pub memory_entry: f32,
    pub stagehand_note: f32,
    pub memory: f32,
}

impl Default for SourceWeights {
    fn default() -> Self {
        Self {
            daily_log: 1.0,
            memory_entry: 1.1,      // Slightly prefer memories
            stagehand_note: 1.3,   // Prefer stagehand for gig context
            memory: 1.0,
        }
    }
}

impl Default for RelevanceScorer {
    fn default() -> Self {
        Self {
            decay_start_days: 7.0,   // Start decaying after a week
            decay_rate: 0.05,         // Gradual decay
            min_decay_score: 0.3,     // Never go below 30%
            source_weights: SourceWeights::default(),
        }
    }
}

impl RelevanceScorer {
    /// Create scorer with custom weights
    pub fn with_weights(weights: SourceWeights) -> Self {
        Self {
            source_weights: weights,
            ..Default::default()
        }
    }

    /// Create scorer optimized for stagehand/gig context
    pub fn stagehand_focused() -> Self {
        Self {
            source_weights: SourceWeights {
                daily_log: 0.8,
                memory_entry: 1.0,
                stagehand_note: 1.5,
                memory: 0.9,
            },
            decay_start_days: 14.0,  // Gig notes stay relevant longer
            decay_rate: 0.03,
            min_decay_score: 0.4,
        }
    }

    /// Score a context result
    pub fn score(&self, result: &ContextResult) -> f32 {
        let time_score = self.time_decay_score(result.created_at);
        let source_weight = self.source_weight(&result.source);
        
        time_score * source_weight
    }

    /// Calculate time decay score
    fn time_decay_score(&self, created_at: Option<NaiveDateTime>) -> f32 {
        let Some(created) = created_at else {
            return self.min_decay_score; // Unknown age = minimum
        };

        let now = Utc::now().naive_utc();
        let age_days = (now - created).num_days() as f32;

        if age_days <= self.decay_start_days {
            return 1.0; // No decay yet
        }

        let decay_days = age_days - self.decay_start_days;
        let decay = (-self.decay_rate * decay_days).exp();
        
        decay.max(self.min_decay_score)
    }

    /// Get source weight
    fn source_weight(&self, source: &ContextSource) -> f32 {
        match source {
            ContextSource::DailyLog => self.source_weights.daily_log,
            ContextSource::MemoryEntry => self.source_weights.memory_entry,
            ContextSource::StagehandNote => self.source_weights.stagehand_note,
            ContextSource::Memory => self.source_weights.memory,
        }
    }
}

/// Result with relevance score applied
#[derive(Debug, Clone)]
pub struct ScoredResult {
    pub result: ContextResult,
    pub final_score: f32,
}

impl ScoredResult {
    /// Create scored result from context result and scorer
    pub fn new(result: ContextResult, scorer: &RelevanceScorer) -> Self {
        let relevance = scorer.score(&result);
        let final_score = result.similarity * relevance;
        
        Self {
            result,
            final_score,
        }
    }
}

/// Score and rank a batch of results
pub fn rank_results(
    results: Vec<ContextResult>,
    scorer: &RelevanceScorer,
) -> Vec<ScoredResult> {
    let mut scored: Vec<ScoredResult> = results
        .into_iter()
        .map(|r| ScoredResult::new(r, scorer))
        .collect();
    
    scored.sort_by(|a, b| {
        b.final_score
            .partial_cmp(&a.final_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    scored
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_source_weights() {
        let scorer = RelevanceScorer::default();
        assert!(scorer.source_weights.stagehand_note > scorer.source_weights.daily_log);
    }

    #[test]
    fn test_time_decay_recent() {
        let scorer = RelevanceScorer::default();
        let now = Utc::now().naive_utc();
        
        // Recent (within decay_start_days)
        let score = scorer.time_decay_score(Some(now - Duration::days(3)));
        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_time_decay_old() {
        let scorer = RelevanceScorer::default();
        let now = Utc::now().naive_utc();
        
        // Old (well past decay_start_days)
        let score = scorer.time_decay_score(Some(now - Duration::days(100)));
        assert!(score < 1.0);
        assert!(score >= scorer.min_decay_score);
    }

    #[test]
    fn test_stagehand_focused() {
        let scorer = RelevanceScorer::stagehand_focused();
        assert!(scorer.source_weights.stagehand_note > 1.0);
        assert!(scorer.decay_start_days > 7.0);
    }

    #[test]
    fn test_score_combines_factors() {
        let scorer = RelevanceScorer::default();
        let now = Utc::now().naive_utc();
        
        let result = ContextResult {
            id: 1,
            content: "test".to_string(),
            source: ContextSource::StagehandNote,
            similarity: 0.9,
            relevance_score: 1.0,
            created_at: Some(now),
            metadata: None,
        };
        
        let score = scorer.score(&result);
        // Stagehand weight (1.3) * time decay (1.0 for recent) = 1.3
        assert!((score - 1.3).abs() < 0.01);
    }
}
