# Simplified GitHub Workflows

This directory contains simplified workflows that can be fully replicated locally.

## Removed Complex Workflows

The following workflows have been removed due to complexity and difficulty in local replication:

1. **Multi-platform releases** - Complex cross-compilation and publishing
2. **Multi-language documentation** - Sphinx, TypeDoc, complex deployment
3. **Matrix testing** - Multiple Python/Node versions
4. **External service integrations** - Codecov, complex caching

## Current Simplified Workflows

1. **ci-simple.yml** - Basic Rust testing and linting
2. **security.yml** - Security audits only

## Local Testing

Use the provided local testing scripts:

- **Windows**: `local-ci.bat`
- **Unix/Linux/macOS**: `local-ci.sh`

These scripts replicate all essential checks locally before pushing.

## Workflow Philosophy

- **Local First**: All checks should pass locally before pushing
- **Essential Only**: Only include checks that catch real issues
- **Fast Feedback**: Quick execution for rapid development
- **Cross-platform**: Works on all development environments

## Restored Workflows (Backup)

Original complex workflows are backed up in `workflows-backup/` directory.