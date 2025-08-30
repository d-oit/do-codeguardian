---
name: Feature Request
about: Suggest a new feature for CodeGuardian
title: "[FEATURE] "
labels: enhancement
assignees: ''

body:
- type: textarea
  id: summary
  attributes:
    label: Feature Summary
    description: A brief summary of the feature.
  validations:
    required: true

- type: textarea
  id: problem
  attributes:
    label: Problem/Use Case
    description: Describe the problem this feature would solve.
  validations:
    required: true

- type: textarea
  id: solution
  attributes:
    label: Proposed Solution
    description: Describe your proposed solution.
  validations:
    required: true

- type: textarea
  id: alternatives
  attributes:
    label: Alternative Solutions
    description: Describe any alternative solutions you've considered.

- type: textarea
  id: additional
  attributes:
    label: Additional Context
    description: Any additional context or screenshots.
---