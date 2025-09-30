#!/bin/bash
# Generate architecture diagrams from PlantUML sources

set -e

echo "🎨 Generating Architecture Diagrams"
echo "=================================="

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if PlantUML is installed
if ! command -v plantuml >/dev/null 2>&1; then
    echo "❌ PlantUML is not installed. Please install it first:"
    echo "   sudo apt-get install plantuml"
    exit 1
fi

# Check if diagram source directory exists
if [[ ! -d "docs/diagrams" ]]; then
    echo "❌ Diagram source directory 'docs/diagrams' not found"
    exit 1
fi

# Create images directory if it doesn't exist
mkdir -p docs/images

echo -e "${BLUE}📁 Generating SVG diagrams...${NC}"
plantuml -tsvg -o ../images docs/diagrams/*.puml

echo -e "${BLUE}📁 Generating PNG diagrams...${NC}"
plantuml -tpng -o ../images docs/diagrams/*.puml

echo -e "${GREEN}✅ All diagrams generated successfully!${NC}"
echo ""
echo "Generated files:"
ls -la docs/images/ | grep -E '\.(svg|png)$' | while read -r line; do
    echo "  📊 $line"
done

echo ""
echo "🚀 Diagrams are ready for use in documentation!"