---
name: Bug Report
about: Report a bug in CodeGuardian
title: "[BUG] "
labels: bug
assignees: ''

body:
- type: textarea
  id: description
  attributes:
    label: Description
    description: A clear and concise description of the bug.
  validations:
    required: true

- type: textarea
  id: reproduction
  attributes:
    label: Steps to Reproduce
    description: Steps to reproduce the behavior.
    placeholder: |
      1. Go to '...'
      2. Click on '...'
      3. Scroll down to '...'
      4. See error
  validations:
    required: true

- type: textarea
  id: expected
  attributes:
    label: Expected Behavior
    description: What you expected to happen.
  validations:
    required: true

- type: textarea
  id: actual
  attributes:
    label: Actual Behavior
    description: What actually happened.
  validations:
    required: true

- type: input
  id: version
  attributes:
    label: CodeGuardian Version
    description: The version of CodeGuardian you are using.
    placeholder: e.g., v1.0.0
  validations:
    required: true

- type: input
  id: rust_version
  attributes:
    label: Rust Version
    description: The version of Rust you are using.
    placeholder: e.g., rustc 1.70.0
  validations:
    required: true

- type: input
  id: os
  attributes:
    label: Operating System
    description: Your operating system and version.
    placeholder: e.g., Ubuntu 22.04
  validations:
    required: true

- type: textarea
  id: additional
  attributes:
    label: Additional Context
    description: Any additional context about the problem.
---
