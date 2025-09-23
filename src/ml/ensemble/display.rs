use super::*;

/// Display implementation for EnsemblePrediction
impl std::fmt::Display for EnsemblePrediction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "🎯 Ensemble Prediction")?;
        writeln!(f, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
        writeln!(f, "Final Prediction: {:.3}", self.prediction)?;
        writeln!(f, "Uncertainty: {:.3}", self.uncertainty)?;
        writeln!(
            f,
            "Confidence: {:.1}%",
            self.explanation.confidence_level * 100.0
        )?;
        writeln!(f)?;

        writeln!(f, "📊 Individual Model Predictions:")?;
        for (i, (&pred, &conf)) in self
            .individual_predictions
            .iter()
            .zip(self.confidence_scores.iter())
            .enumerate()
        {
            let agrees = self
                .explanation
                .model_agreement
                .get(&i)
                .copied()
                .unwrap_or(false);
            let marker = if agrees { "✅" } else { "❌" };
            writeln!(
                f,
                "  {} Model {}: {:.3} (confidence: {:.3})",
                marker,
                i + 1,
                pred,
                conf
            )?;
        }

        if !self.explanation.uncertainty_sources.is_empty() {
            writeln!(f)?;
            writeln!(f, "⚠️  Uncertainty Sources:")?;
            for source in &self.explanation.uncertainty_sources {
                writeln!(f, "   • {}", source)?;
            }
        }

        Ok(())
    }
}

/// Display implementation for EnsembleTrainingHistory
impl std::fmt::Display for EnsembleTrainingHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "📈 Ensemble Training History")?;
        writeln!(f, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")?;

        if !self.model_performances.is_empty() {
            writeln!(f, "Individual Model Performance:")?;
            for perf in &self.model_performances {
                writeln!(
                    f,
                    "  Model {}: Acc={:.3}, F1={:.3}, Time={:.1}ms",
                    perf.model_id + 1,
                    perf.accuracy,
                    perf.f1_score,
                    perf.training_time.as_millis()
                )?;
            }
        }

        if !self.diversity_metrics.is_empty() {
            writeln!(f)?;
            writeln!(f, "Diversity Metrics:")?;
            for (i, metrics) in self.diversity_metrics.iter().enumerate() {
                writeln!(
                    f,
                    "  Iteration {}: Diversity={:.3}, Q-stat={:.3}, Disagreement={:.3}",
                    i + 1,
                    metrics.pairwise_diversity,
                    metrics.q_statistic,
                    metrics.disagreement
                )?;
            }
        }

        if !self.training_times.is_empty() {
            let total_time: std::time::Duration = self.training_times.iter().sum();
            writeln!(f)?;
            writeln!(f, "Total Training Time: {:.2}s", total_time.as_secs_f64())?;
        }

        Ok(())
    }
}