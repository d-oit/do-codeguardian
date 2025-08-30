/**
 * OpenCode Plugin: CodeGuardian CI Integration
 *
 * This plugin integrates CodeGuardian with CI pipelines by hooking into events like git push or CI triggers.
 * It uses the $ shell API to execute CodeGuardian commands and handles output reporting for different CI systems.
 *
 * Supported CI systems:
 * - GitHub Actions
 * - GitLab CI
 * - Jenkins
 * - Generic (fallback)
 */

export const CodeGuardianCIIntegration = async ({ app, client, $ }) => {
  /**
   * Configuration options for the plugin.
   * Can be customized when initializing the plugin.
   */
  const config = {
    command: 'cargo run -- check', // Default CodeGuardian command
    ciSystem: null, // Auto-detect if null
    reportFile: 'codeguardian-report.json', // File to save report
    failOnIssues: true, // Fail CI if issues found
  };

  /**
   * Detects the CI system based on environment variables.
   * @returns {string} The detected CI system ('github', 'gitlab', 'jenkins', 'generic')
   */
  const detectCISystem = () => {
    if (process.env.GITHUB_ACTIONS) return 'github';
    if (process.env.GITLAB_CI) return 'gitlab';
    if (process.env.JENKINS_HOME) return 'jenkins';
    return 'generic';
  };

  /**
   * Runs CodeGuardian analysis using the configured command.
   * @returns {Promise<Object>} Result object with stdout, stderr, exitCode
   */
  const runAnalysis = async () => {
    console.log('Running CodeGuardian analysis...');
    try {
      const result = await $`${config.command}`;
      console.log('CodeGuardian analysis completed successfully.');
      return { stdout: result.stdout, stderr: result.stderr, exitCode: 0 };
    } catch (error) {
      console.error('CodeGuardian analysis failed.');
      return { stdout: error.stdout || '', stderr: error.stderr || '', exitCode: error.exitCode || 1 };
    }
  };

  /**
   * Reports the analysis results based on the CI system.
   * @param {Object} result - The result from runAnalysis
   */
  const reportResults = async (result) => {
    const ciSystem = config.ciSystem || detectCISystem();
    console.log(`Reporting results for CI system: ${ciSystem}`);

    // Save report to file
    await $`echo '${JSON.stringify(result)}' > ${config.reportFile}`;

    switch (ciSystem) {
      case 'github':
        if (result.exitCode === 0) {
          await $`echo "status=success" >> $GITHUB_OUTPUT`;
          await $`echo "::notice::CodeGuardian analysis passed"`;
        } else {
          await $`echo "status=failure" >> $GITHUB_OUTPUT`;
          await $`echo "::error::CodeGuardian analysis failed"`;
        }
        break;
      case 'gitlab':
        if (result.exitCode === 0) {
          await $`echo "CodeGuardian analysis passed" >> codeguardian-ci.log`;
        } else {
          await $`echo "CodeGuardian analysis failed" >> codeguardian-ci.log`;
          if (config.failOnIssues) {
            process.exit(1);
          }
        }
        break;
      case 'jenkins':
        if (result.exitCode === 0) {
          await $`echo "SUCCESS: CodeGuardian analysis passed"`;
        } else {
          await $`echo "FAILURE: CodeGuardian analysis failed"`;
          if (config.failOnIssues) {
            process.exit(1);
          }
        }
        break;
      default:
        console.log('Analysis result:', result);
        if (result.exitCode !== 0 && config.failOnIssues) {
          process.exit(1);
        }
    }
  };

  /**
   * Initializes the plugin with custom configuration.
   * @param {Object} options - Configuration options
   */
  const init = (options = {}) => {
    Object.assign(config, options);
    console.log('CodeGuardian CI Integration plugin initialized with config:', config);
  };

  // Initialize with default config
  init();

  return {
    /**
     * Hook triggered on git push events.
     * Runs CodeGuardian analysis and reports results.
     */
    onGitPush: async () => {
      console.log('Git push detected, running CodeGuardian CI integration...');
      const result = await runAnalysis();
      await reportResults(result);
    },

    /**
     * Hook triggered on CI pipeline triggers.
     * Runs CodeGuardian analysis and reports results.
     */
    onCITrigger: async () => {
      console.log('CI trigger detected, running CodeGuardian CI integration...');
      const result = await runAnalysis();
      await reportResults(result);
    },

    /**
     * Hook triggered on pre-commit events (if supported).
     * Runs CodeGuardian analysis before committing.
     */
    onPreCommit: async () => {
      console.log('Pre-commit hook triggered, running CodeGuardian analysis...');
      const result = await runAnalysis();
      if (result.exitCode !== 0 && config.failOnIssues) {
        console.error('CodeGuardian analysis failed, aborting commit.');
        process.exit(1);
      }
    },

    /**
     * Initialize the plugin with custom options.
     */
    init,
  };
};