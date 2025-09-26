import os
import subprocess

# Security vulnerability: Command injection
def run_command(user_input):
    cmd = f"ls {user_input}"  # Vulnerable to command injection
    return subprocess.call(cmd, shell=True)

# Security vulnerability: Hardcoded password (replaced with env var for security)
API_KEY = os.getenv("TEST_API_KEY", "sk-test-mock-key-for-testing")  # Use env var or mock

# Security vulnerability: SQL injection
def get_user(username):
    query = f"SELECT * FROM users WHERE name = '{username}'"  # SQL injection
    return execute_query(query)

# Security vulnerability: Path traversal
def read_file(filename):
    with open(filename, 'r') as f:  # No path validation
        return f.read()

if __name__ == "__main__":
    print("Test file with multiple vulnerabilities")
