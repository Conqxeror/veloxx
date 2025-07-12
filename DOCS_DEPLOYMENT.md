# Documentation Deployment Guide

This guide explains how to deploy documentation for the Veloxx project using GitHub Pages.

## Overview

The documentation deployment system creates a comprehensive documentation site that includes:
- **Rust API Documentation**: Generated using `cargo doc`
- **Python Bindings Documentation**: Links to README_PYTHON.md
- **WebAssembly Documentation**: Links to README_WASM.md
- **Main Landing Page**: Beautiful overview with feature highlights

## Automatic Deployment

### GitHub Actions Workflow

The project includes a GitHub Actions workflow (`.github/workflows/docs-deploy.yml`) that automatically:

1. **Builds Rust Documentation**: Uses `cargo doc --all-features --no-deps --document-private-items`
2. **Creates Documentation Site**: Generates a beautiful landing page with navigation
3. **Deploys to GitHub Pages**: Automatically publishes to your repository's GitHub Pages

### Triggering Deployment

The documentation is automatically deployed when:
- Code is pushed to the `main` branch
- Manual trigger via GitHub Actions interface

## Manual Deployment

### Prerequisites

1. **GitHub Pages Setup**: Ensure GitHub Pages is enabled in your repository settings
2. **Permissions**: The workflow needs `pages: write` and `id-token: write` permissions

### Local Documentation Build

To build documentation locally:

```bash
# Build Rust documentation
cargo doc --all-features --no-deps --document-private-items

# Open documentation in browser
cargo doc --open
```

### Repository Settings

1. Go to your repository on GitHub
2. Navigate to **Settings** → **Pages**
3. Under **Source**, select **GitHub Actions**
4. The workflow will handle the rest automatically

## Documentation Structure

The deployed documentation site includes:

```
docs-site/
├── index.html              # Main landing page
├── rust/                   # Rust API documentation
│   ├── veloxx/            # Main crate documentation
│   └── index.html         # Redirects to veloxx
└── (links to Python/WASM docs in GitHub)
```

## Features of the Documentation Site

### Main Landing Page
- **Modern Design**: Gradient background with card-based navigation
- **Feature Highlights**: Key capabilities of Veloxx
- **Multi-Language Support**: Links to all binding documentations
- **Responsive Layout**: Works on desktop and mobile devices

### Rust Documentation
- **Complete API Reference**: All public and private items documented
- **Code Examples**: Inline examples with syntax highlighting
- **Type Information**: Detailed type signatures and relationships
- **Feature Flags**: Documentation for optional features (python, wasm)

## Customization

### Modifying the Landing Page

To customize the main documentation page, edit the HTML content in the workflow file:
`.github/workflows/docs-deploy.yml` around line 50-200.

### Adding New Documentation Sections

1. Create additional build steps in the workflow
2. Add new cards to the landing page grid
3. Update the navigation structure

## Troubleshooting

### Common Issues

1. **Workflow Fails**: Check that GitHub Pages is enabled and permissions are correct
2. **Documentation Not Updating**: Ensure the workflow completed successfully
3. **Missing Content**: Verify all documentation files are committed to the repository

### Debugging Steps

1. Check the Actions tab in your GitHub repository
2. Review workflow logs for any errors
3. Ensure all required files are present in the repository
4. Verify GitHub Pages settings in repository settings

## Access Your Documentation

Once deployed, your documentation will be available at:
`https://[username].github.io/[repository-name]/`

For the Veloxx project, this would be:
`https://conqxeror.github.io/veloxx/`

## Local Development

To test the documentation site locally:

1. Build the documentation:
   ```bash
   cargo doc --all-features --no-deps --document-private-items
   ```

2. Create a local docs-site directory and copy the generated files
3. Serve using a local HTTP server:
   ```bash
   # Using Python
   python -m http.server 8000
   
   # Using Node.js
   npx serve .
   ```

## Maintenance

### Regular Updates

- Documentation is automatically rebuilt on every push to main
- No manual intervention required for routine updates
- The workflow caches dependencies for faster builds

### Version Updates

When releasing new versions:
1. Update version numbers in Cargo.toml
2. Update CHANGELOG.md
3. Push to main branch
4. Documentation will automatically reflect the new version

## Security

The workflow uses:
- **Minimal Permissions**: Only necessary permissions for Pages deployment
- **Official Actions**: Uses verified GitHub Actions
- **No Secrets Required**: Uses built-in GitHub tokens
- **Restricted Scope**: Only builds and deploys documentation