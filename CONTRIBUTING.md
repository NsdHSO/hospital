# Contributing to Hospital Project

Thank you for considering contributing to the Hospital Project! This document outlines the process for contributing to the project and guidelines for commit messages to ensure automated semantic versioning and release management.

## Conventional Commits

We use [Conventional Commits](https://www.conventionalcommits.org/) to automate version management and release notes. This makes it easier to track changes, understand the impact of changes, and generate changelogs.

### Commit Message Format

Each commit message consists of a **header**, a **body** and a **footer**. The header has a special format that includes a **type**, a **scope** and a **subject**:

```
<type>(<scope>): <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
```

The **header** is mandatory and the **scope** of the header is optional. Any line in the commit message cannot be longer than 100 characters.

### Types

The commit type must be one of the following:

- **feat**: A new feature (triggers a minor version bump)
- **fix**: A bug fix (triggers a patch version bump)
- **docs**: Documentation only changes (triggers a patch version bump)
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, etc) (no version bump)
- **refactor**: A code change that neither fixes a bug nor adds a feature (triggers a patch version bump)
- **perf**: A code change that improves performance (triggers a patch version bump)
- **test**: Adding missing tests or correcting existing tests (triggers a patch version bump)
- **build**: Changes that affect the build system or external dependencies (triggers a patch version bump)
- **ci**: Changes to our CI configuration files and scripts (no version bump)
- **chore**: Other changes that don't modify src or test files (no version bump)

### Scope

The scope is optional and should be a noun describing a section of the codebase that is affected by the change. Examples might be:

- **ambulance**
- **patient**
- **hospital**
- **doctor**
- **ui**
- **api**
- **db**

### Subject

The subject is a short description of the change:

- use the imperative, present tense: "change" not "changed" nor "changes"
- don't capitalize the first letter
- no dot (.) at the end

### Examples

```
feat(ambulance): add status tracking for ambulances

fix(patient): resolve issue with patient registration

docs(api): update API documentation for status endpoint

refactor(db): improve database connection handling

test(ambulance): add integration tests for status endpoint
```

## Breaking Changes

Breaking changes should be noted in the footer with a `BREAKING CHANGE:` prefix, followed by a description of what has changed and what users need to do differently.

```
feat(api): change response format for ambulance status

BREAKING CHANGE: The ambulance status endpoint now returns data in a 
different format. Clients need to update their parsing logic.
```

This type of commit will trigger a major version bump (1.0.0 -> 2.0.0).

## Pull Requests

Pull requests should have a title that follows the conventional commit format. This will be used to generate the release notes.

## Local Development

Before committing, please make sure:

1. All tests pass
2. Your code follows our style guidelines
3. You've added tests for new features
4. Your commit message follows the conventional commit format

Thank you for your contributions!
