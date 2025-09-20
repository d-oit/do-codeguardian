use chrono::{DateTime, Duration, Utc};
use codeguardian::release_monitoring::{
    ReleaseData, ReleaseMonitoringConfig, ReleaseMonitoringService,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Benchmark release monitoring performance
fn benchmark_release_monitoring(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("collect_release_data_small", |b| {
        b.to_async(&rt).iter(|| async {
            let config = ReleaseMonitoringConfig {
                repository: "test/repo".to_string(),
                max_releases_to_monitor: 5,
                ..Default::default()
            };

            let service = ReleaseMonitoringService::new(config);

            // Mock the GitHub client to avoid actual API calls
            // In real benchmarks, you'd use a mock or test data

            black_box(service);
        });
    });

    c.bench_function("calculate_metrics_10_releases", |b| {
        let releases = create_test_releases(10);

        b.iter(|| {
            let config = ReleaseMonitoringConfig::default();
            let service = ReleaseMonitoringService::new(config);

            rt.block_on(async {
                let _metrics = service.calculate_metrics(&releases).await.unwrap();
            });
        });
    });

    c.bench_function("calculate_metrics_50_releases", |b| {
        let releases = create_test_releases(50);

        b.iter(|| {
            let config = ReleaseMonitoringConfig::default();
            let service = ReleaseMonitoringService::new(config);

            rt.block_on(async {
                let _metrics = service.calculate_metrics(&releases).await.unwrap();
            });
        });
    });

    c.bench_function("calculate_metrics_100_releases", |b| {
        let releases = create_test_releases(100);

        b.iter(|| {
            let config = ReleaseMonitoringConfig::default();
            let service = ReleaseMonitoringService::new(config);

            rt.block_on(async {
                let _metrics = service.calculate_metrics(&releases).await.unwrap();
            });
        });
    });
}

fn create_test_releases(count: usize) -> Vec<ReleaseData> {
    let mut releases = Vec::with_capacity(count);
    let base_time = Utc::now() - Duration::days(30);

    for i in 0..count {
        let created_at = base_time - Duration::days(i as i64 * 3);
        let published_at = Some(created_at + Duration::hours((i % 24) as i64));

        releases.push(ReleaseData {
            tag_name: format!("v1.{}.0", i),
            name: format!("Version 1.{}.0", i),
            created_at,
            published_at,
            draft: i % 10 == 0,      // Every 10th release is a draft
            prerelease: i % 15 == 0, // Every 15th is prerelease
            body: format!(
                "Release notes for version 1.{}.0 with various improvements and bug fixes.",
                i
            ),
            download_count: (i * 50) as u64, // Increasing download counts
        });
    }

    releases
}

criterion_group!(benches, benchmark_release_monitoring);
criterion_main!(benches);
