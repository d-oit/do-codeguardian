/**
 * OpenCode Plugin: CodeGuardian Security Analysis
 *
 * This plugin hooks into file save events to automatically run CodeGuardian's security analysis.
 * It uses the $ shell API to execute 'cargo run -- check' and handles the output accordingly.
 */

export const CodeGuardianSecurityAnalysis = async ({ app, client, $ }) => {
  /**
   * Hook triggered on file save events.
   * Runs CodeGuardian security analysis using 'cargo run -- check'.
   * @param {string} filePath - The path of the saved file.
   */
  const onFileSave = async (filePath) => {
    console.log(`Running CodeGuardian security analysis on file: ${filePath}`);

    try {
      // Execute CodeGuardian's check command
      const result = await $`cargo run -- check`;

      // Log successful analysis results
      console.log('CodeGuardian analysis completed successfully.');
      console.log('Output:', result.stdout);

      // Optionally, you can integrate with OpenCode's UI to display results
      // For example, show a notification or update a panel
      // openCode.showNotification('Security analysis completed', result.stdout);

    } catch (error) {
      // Handle errors from the command execution
      console.error('CodeGuardian analysis failed.');
      console.error('Error details:', error.stderr);

      // Optionally, show an error notification
      // openCode.showErrorNotification('Security analysis failed', error.stderr);
    }
  };

  /**
   * Optional: Hook into tool executions if needed.
   * This can be extended to run analysis on other events.
   */
  const onToolExecution = async (toolName, args) => {
    // Example: Run analysis after certain tools
    if (toolName === 'some-tool') {
      // Similar logic as onFileSave
    }
  };

  return {
    hooks: {
      onFileSave,
      onToolExecution
    }
  };
};