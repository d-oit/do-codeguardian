# Maintainer Setup for Vulnerability Reporting

1. Enable private vulnerability reporting:
   - Go to Repository Settings > Security & analysis
   - Enable "Private vulnerability reporting"
   - Enable "GitHub Security Advisories"

2. Configure issue labels:
   - Create "security" label in repository
   - Apply to all vulnerability-related issues

3. Triage workflow:
   - Acknowledge reports within 48 hours
   - Create internal tracking issue with "security" label
   - Use GitHub Security Advisories for coordination

4. Disclosure process:
   - Develop patch in private branch
   - Request CVE through GitHub
   - Coordinate release with security fix
   - Publish advisory after patch release
