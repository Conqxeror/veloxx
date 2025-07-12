#!/bin/bash

# Local Documentation Build Script
# This script builds documentation locally for development and testing

set -e

echo "🦀 Building Veloxx Documentation Locally"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo not found. Please install Rust first.${NC}"
    exit 1
fi

echo -e "${BLUE}📚 Building Rust documentation...${NC}"
cargo doc --all-features --no-deps --document-private-items

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Rust documentation built successfully${NC}"
else
    echo -e "${RED}❌ Failed to build Rust documentation${NC}"
    exit 1
fi

echo -e "${BLUE}🏗️ Creating local documentation site...${NC}"

# Create docs-site directory
mkdir -p docs-site

# Copy Rust docs
echo -e "${YELLOW}📁 Copying Rust documentation...${NC}"
mkdir -p docs-site/rust
cp -r target/doc/* docs-site/rust/
echo '<meta http-equiv="refresh" content="0; url=veloxx">' > docs-site/rust/index.html

# Create main index page
echo -e "${YELLOW}📄 Creating main landing page...${NC}"
cat > docs-site/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Veloxx Documentation - Local Build</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 40px 20px;
        }
        .header {
            text-align: center;
            color: white;
            margin-bottom: 60px;
        }
        .header h1 {
            font-size: 3.5em;
            margin: 0;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        .header p {
            font-size: 1.3em;
            margin: 20px 0;
            opacity: 0.9;
        }
        .local-notice {
            background: rgba(255, 255, 255, 0.1);
            border: 2px solid rgba(255, 255, 255, 0.3);
            border-radius: 10px;
            padding: 20px;
            margin-bottom: 40px;
            color: white;
            text-align: center;
        }
        .docs-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
            gap: 30px;
            margin-top: 40px;
        }
        .doc-card {
            background: white;
            border-radius: 15px;
            padding: 30px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
            transition: transform 0.3s ease, box-shadow 0.3s ease;
            text-decoration: none;
            color: inherit;
        }
        .doc-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 15px 40px rgba(0,0,0,0.3);
        }
        .doc-icon {
            font-size: 3em;
            margin-bottom: 20px;
            display: block;
        }
        .doc-title {
            font-size: 1.5em;
            font-weight: bold;
            margin-bottom: 15px;
            color: #333;
        }
        .doc-description {
            color: #666;
            margin-bottom: 20px;
        }
        .doc-link {
            display: inline-block;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 12px 24px;
            border-radius: 25px;
            text-decoration: none;
            font-weight: 500;
            transition: opacity 0.3s ease;
        }
        .doc-link:hover {
            opacity: 0.9;
        }
        .external-link {
            background: #6c757d;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🦀 Veloxx</h1>
            <p>High-performance data processing library with multi-language bindings</p>
        </div>
        
        <div class="local-notice">
            <h3>🏠 Local Documentation Build</h3>
            <p>This is a local build of the Veloxx documentation. Some links may point to external resources.</p>
        </div>
        
        <div class="docs-grid">
            <a href="rust/" class="doc-card">
                <span class="doc-icon">🦀</span>
                <div class="doc-title">Rust Documentation</div>
                <div class="doc-description">
                    Complete API reference for the core Rust library with detailed examples and type information.
                </div>
                <div class="doc-link">View Rust Docs</div>
            </a>
            
            <a href="../README_PYTHON.md" class="doc-card">
                <span class="doc-icon">🐍</span>
                <div class="doc-title">Python Bindings</div>
                <div class="doc-description">
                    Python bindings documentation with installation guide and usage examples.
                </div>
                <div class="doc-link external-link">View Python Docs</div>
            </a>
            
            <a href="../README_WASM.md" class="doc-card">
                <span class="doc-icon">🕸️</span>
                <div class="doc-title">WebAssembly Bindings</div>
                <div class="doc-description">
                    WebAssembly bindings for JavaScript/TypeScript with browser and Node.js support.
                </div>
                <div class="doc-link external-link">View WASM Docs</div>
            </a>
        </div>
        
        <div style="text-align: center; margin-top: 40px;">
            <a href="../README.md" style="color: white; text-decoration: none; font-size: 1.2em;">
                📖 View Main README
            </a>
        </div>
    </div>
</body>
</html>
EOF

echo -e "${GREEN}✅ Documentation site created successfully!${NC}"
echo -e "${BLUE}📂 Documentation location: $(pwd)/docs-site/${NC}"
echo ""
echo -e "${YELLOW}🌐 To view the documentation:${NC}"
echo "1. Open docs-site/index.html in your browser, or"
echo "2. Serve locally with:"
echo "   cd docs-site && python -m http.server 8000"
echo "   Then visit: http://localhost:8000"
echo ""
echo -e "${GREEN}🎉 Local documentation build complete!${NC}"