use do_codeguardian::cache::memory_pool::{FindingPool, MemoryPoolManager, StringPool};
use do_codeguardian::cache::regex_cache::{RegexCache, SharedRegexCache};
use do_codeguardian::types::{Finding, Severity};
use std::path::PathBuf;
use std::time::Instant;

#[test]
fn test_regex_cache_performance() {
    let mut cache = RegexCache::new(100, 3600);
    let patterns = vec![
        r"test\d+",
        r"error\s+\w+",
        r"warning\s+\w+",
        r"function\s+\w+\s*\(",
        r"class\s+\w+",
    ];

    // First compilation (cache miss)
    let start = Instant::now();
    for pattern in &patterns {
        cache.get_or_compile(pattern).unwrap();
    }
    let first_run = start.elapsed();

    // Second compilation (cache hit)
    let start = Instant::now();
    for pattern in &patterns {
        cache.get_or_compile(pattern).unwrap();
    }
    let second_run = start.elapsed();

    println!("Regex Cache Performance:");
    println!("  First run (compilation): {:.2}ms", first_run.as_millis());
    println!("  Second run (cached): {:.2}ms", second_run.as_millis());
    println!(
        "  Speedup: {:.1}x",
        first_run.as_millis() as f64 / second_run.as_millis() as f64
    );
    println!("  Cache stats: {}", cache.stats().report());

    // Verify cache is working
    assert!(cache.stats().cache_hits > 0);
    assert!(cache.stats().cache_misses > 0);
    assert!(second_run < first_run);
}

#[test]
fn test_memory_pool_performance() {
    let mut finding_pool = FindingPool::new(1000);
    let mut string_pool = StringPool::new(1000);

    // Test finding pool
    let mut findings = Vec::new();
    let start = Instant::now();
    for i in 0..100 {
        let mut finding = finding_pool.get();
        finding.id = format!("test-finding-{}", i);
        finding.analyzer = "test".to_string();
        finding.rule = "test-rule".to_string();
        finding.severity = Severity::Info;
        finding.file = PathBuf::from(format!("/test/file{}.rs", i));
        finding.line = i as u32;
        finding.message = format!("Test message {}", i);
        findings.push(finding);
    }
    let allocation_time = start.elapsed();

    // Return to pool
    let start = Instant::now();
    for finding in findings {
        finding_pool.put(finding);
    }
    let return_time = start.elapsed();

    // Reuse from pool
    let start = Instant::now();
    let mut reused_findings = Vec::new();
    for _ in 0..100 {
        reused_findings.push(finding_pool.get());
    }
    let reuse_time = start.elapsed();

    println!("Memory Pool Performance:");
    println!(
        "  Finding pool allocation: {:.2}ms",
        allocation_time.as_millis()
    );
    println!("  Finding pool return: {:.2}ms", return_time.as_millis());
    println!("  Finding pool reuse: {:.2}ms", reuse_time.as_millis());
    println!("  Finding pool stats: {}", finding_pool.stats().report());

    // Test string pool
    let start = Instant::now();
    for i in 0..100 {
        string_pool.get(&format!("test-string-{}", i));
    }
    let string_time = start.elapsed();

    println!("  String pool intern: {:.2}ms", string_time.as_millis());
    println!("  String pool stats: {}", string_pool.stats().report());

    // Test memory pool manager
    let manager = MemoryPoolManager::new();
    let savings = manager.memory_savings_estimate();
    println!("  Memory savings estimate: {}", savings.report());

    // Verify pools are working
    assert!(finding_pool.stats().reused > 0);
    assert!(finding_pool.stats().returned > 0);
    assert!(string_pool.stats().allocated > 0);
}

#[test]
fn test_shared_regex_cache_thread_safety() {
    use std::thread;

    let cache = SharedRegexCache::new(10, 3600);

    let cache_clone = cache.clone();
    let handle = thread::spawn(move || cache_clone.get_or_compile(r"thread\d+").unwrap());

    let regex = cache.get_or_compile(r"main\d+").unwrap();
    let thread_regex = handle.join().unwrap();

    assert!(regex.is_match("main123"));
    assert!(thread_regex.is_match("thread456"));
}
