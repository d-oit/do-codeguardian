---
name: General Question
about: Ask a general question about CodeGuardian
title: "[QUESTION] "
labels: question
assignees: ''

body:
- type: markdown
  attributes:
    value: |
      ## Note: General Questions in Discussions

      For general questions about CodeGuardian, please use [GitHub Discussions](https://github.com/d-oit/do-codeguardian/discussions) instead of creating an issue. Discussions are better suited for questions, brainstorming, and community interaction.

      If you have a specific bug report, feature request, or security issue, feel free to proceed with this issue template.

- type: textarea
  id: question
  attributes:
    label: Your Question
    description: What is your question?
  validations:
    required: false

- type: textarea
  id: context
  attributes:
    label: Context
    description: Provide any relevant context or background.

- type: dropdown
  id: category
  attributes:
    label: Category
    description: What category does your question fall into?
    options:
      - Installation
      - Usage
      - Configuration
      - Development
      - Security
      - Performance
      - Other
  validations:
    required: false

- type: textarea
  id: additional
  attributes:
    label: Additional Information
    description: Any additional information that might help.
---
