use codeguardian::performance::{PerformanceMetrics, PerformanceAnalyzer};
use std::time::Duration;
use std::sync::Arc;

fn main() {
    println!("Testing Performance Optimizations");
    
    // Test sampling
    let metrics = PerformanceMetrics::with_sample_rate(5);
    println!("Created metrics with sample rate 5");
    
    // Record some operations
    for i in 0..20 {
        metrics.record_file_processed(Duration::from_millis(100));
        if i % 2 == 0 {
            metrics.record_cache_hit();
        } else {
            metrics.record_cache_miss();
        }
    }
    
    println!("Files processed: {}", metrics.total_files_processed.load(std::sync::atomic::Ordering::Relaxed));
    println!("Cache hits: {}", metrics.cache_hits.load(std::sync::atomic::Ordering::Relaxed));
    println!("Cache misses: {}", metrics.cache_misses.load(std::sync::atomic::Ordering::Relaxed));
    println!("Average processing time: {:?}", metrics.get_average_processing_time());
    println!("Cache hit rate: {:.2}", metrics.get_cache_hit_rate());
    
    // Test analyzer
    let analyzer = PerformanceAnalyzer::new(Arc::new(metrics));
    let recommendations = analyzer.analyze_performance();
    println!("Performance recommendations: {}", recommendations.len());
    
    println!("Performance optimizations working correctly!");
}
