@echo off
REM Local Documentation Build Script for Windows
REM This script builds documentation locally for development and testing

setlocal enabledelayedexpansion

echo 🦀 Building Veloxx Documentation Locally
echo ========================================

REM Check if cargo is available
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ Cargo not found. Please install Rust first.
    exit /b 1
)

echo 📚 Building Rust documentation...
cargo doc --all-features --no-deps --document-private-items

if %errorlevel% neq 0 (
    echo ❌ Failed to build Rust documentation
    exit /b 1
)

echo ✅ Rust documentation built successfully

echo 🏗️ Creating local documentation site...

REM Create docs-site directory
if not exist "docs-site" mkdir docs-site

REM Copy Rust docs
echo 📁 Copying Rust documentation...
if not exist "docs-site\rust" mkdir docs-site\rust
xcopy "target\doc\*" "docs-site\rust\" /E /I /Y >nul
echo ^<meta http-equiv="refresh" content="0; url=veloxx"^> > docs-site\rust\index.html

REM Create main index page
echo 📄 Creating main landing page...
(
echo ^<!DOCTYPE html^>
echo ^<html lang="en"^>
echo ^<head^>
echo     ^<meta charset="UTF-8"^>
echo     ^<meta name="viewport" content="width=device-width, initial-scale=1.0"^>
echo     ^<title^>Veloxx Documentation - Local Build^</title^>
echo     ^<style^>
echo         body {
echo             font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
echo             line-height: 1.6;
echo             margin: 0;
echo             padding: 0;
echo             background: linear-gradient^(135deg, #667eea 0%%, #764ba2 100%%^);
echo             min-height: 100vh;
echo         }
echo         .container {
echo             max-width: 1200px;
echo             margin: 0 auto;
echo             padding: 40px 20px;
echo         }
echo         .header {
echo             text-align: center;
echo             color: white;
echo             margin-bottom: 60px;
echo         }
echo         .header h1 {
echo             font-size: 3.5em;
echo             margin: 0;
echo             text-shadow: 2px 2px 4px rgba^(0,0,0,0.3^);
echo         }
echo         .header p {
echo             font-size: 1.3em;
echo             margin: 20px 0;
echo             opacity: 0.9;
echo         }
echo         .local-notice {
echo             background: rgba^(255, 255, 255, 0.1^);
echo             border: 2px solid rgba^(255, 255, 255, 0.3^);
echo             border-radius: 10px;
echo             padding: 20px;
echo             margin-bottom: 40px;
echo             color: white;
echo             text-align: center;
echo         }
echo         .docs-grid {
echo             display: grid;
echo             grid-template-columns: repeat^(auto-fit, minmax^(350px, 1fr^)^);
echo             gap: 30px;
echo             margin-top: 40px;
echo         }
echo         .doc-card {
echo             background: white;
echo             border-radius: 15px;
echo             padding: 30px;
echo             box-shadow: 0 10px 30px rgba^(0,0,0,0.2^);
echo             transition: transform 0.3s ease, box-shadow 0.3s ease;
echo             text-decoration: none;
echo             color: inherit;
echo         }
echo         .doc-card:hover {
echo             transform: translateY^(-5px^);
echo             box-shadow: 0 15px 40px rgba^(0,0,0,0.3^);
echo         }
echo         .doc-icon {
echo             font-size: 3em;
echo             margin-bottom: 20px;
echo             display: block;
echo         }
echo         .doc-title {
echo             font-size: 1.5em;
echo             font-weight: bold;
echo             margin-bottom: 15px;
echo             color: #333;
echo         }
echo         .doc-description {
echo             color: #666;
echo             margin-bottom: 20px;
echo         }
echo         .doc-link {
echo             display: inline-block;
echo             background: linear-gradient^(135deg, #667eea 0%%, #764ba2 100%%^);
echo             color: white;
echo             padding: 12px 24px;
echo             border-radius: 25px;
echo             text-decoration: none;
echo             font-weight: 500;
echo             transition: opacity 0.3s ease;
echo         }
echo         .doc-link:hover {
echo             opacity: 0.9;
echo         }
echo         .external-link {
echo             background: #6c757d;
echo         }
echo     ^</style^>
echo ^</head^>
echo ^<body^>
echo     ^<div class="container"^>
echo         ^<div class="header"^>
echo             ^<h1^>🦀 Veloxx^</h1^>
echo             ^<p^>High-performance data processing library with multi-language bindings^</p^>
echo         ^</div^>
echo         
echo         ^<div class="local-notice"^>
echo             ^<h3^>🏠 Local Documentation Build^</h3^>
echo             ^<p^>This is a local build of the Veloxx documentation. Some links may point to external resources.^</p^>
echo         ^</div^>
echo         
echo         ^<div class="docs-grid"^>
echo             ^<a href="rust/" class="doc-card"^>
echo                 ^<span class="doc-icon"^>🦀^</span^>
echo                 ^<div class="doc-title"^>Rust Documentation^</div^>
echo                 ^<div class="doc-description"^>
echo                     Complete API reference for the core Rust library with detailed examples and type information.
echo                 ^</div^>
echo                 ^<div class="doc-link"^>View Rust Docs^</div^>
echo             ^</a^>
echo             
echo             ^<a href="../README_PYTHON.md" class="doc-card"^>
echo                 ^<span class="doc-icon"^>🐍^</span^>
echo                 ^<div class="doc-title"^>Python Bindings^</div^>
echo                 ^<div class="doc-description"^>
echo                     Python bindings documentation with installation guide and usage examples.
echo                 ^</div^>
echo                 ^<div class="doc-link external-link"^>View Python Docs^</div^>
echo             ^</a^>
echo             
echo             ^<a href="../README_WASM.md" class="doc-card"^>
echo                 ^<span class="doc-icon"^>🕸️^</span^>
echo                 ^<div class="doc-title"^>WebAssembly Bindings^</div^>
echo                 ^<div class="doc-description"^>
echo                     WebAssembly bindings for JavaScript/TypeScript with browser and Node.js support.
echo                 ^</div^>
echo                 ^<div class="doc-link external-link"^>View WASM Docs^</div^>
echo             ^</a^>
echo         ^</div^>
echo         
echo         ^<div style="text-align: center; margin-top: 40px;"^>
echo             ^<a href="../README.md" style="color: white; text-decoration: none; font-size: 1.2em;"^>
echo                 📖 View Main README
echo             ^</a^>
echo         ^</div^>
echo     ^</div^>
echo ^</body^>
echo ^</html^>
) > docs-site\index.html

echo ✅ Documentation site created successfully!
echo 📂 Documentation location: %CD%\docs-site\
echo.
echo 🌐 To view the documentation:
echo 1. Open docs-site\index.html in your browser, or
echo 2. Serve locally with Python:
echo    cd docs-site ^&^& python -m http.server 8000
echo    Then visit: http://localhost:8000
echo.
echo 🎉 Local documentation build complete!