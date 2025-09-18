#!/bin/bash

# Copy Documentation to .cursor/rules Script
# Copies all .md files from root to .cursor/rules/ with .mdc formatting
# Excludes README.md, ROADMAP.md, CLAUDE.md, and PROJECT_HISTORY.md
# Use --check-unexpected flag to prompt for unexpected files

set -e

# Parse command line arguments
CHECK_UNEXPECTED=false
if [[ "$1" == "--check-unexpected" ]]; then
    CHECK_UNEXPECTED=true
fi

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RULES_DIR="$PROJECT_ROOT/.cursor/rules"

# Files to exclude
EXCLUDE_FILES=("README.md" "ROADMAP.md" "CLAUDE.md" "PROJECT_HISTORY.md")

# Expected documentation files to copy
EXPECTED_FILES=(
    "ARCHITECTURE.md"
    "CODING-RULES.md" 
    "DEVELOPMENT-WORKFLOW.md"
    "DOCUMENTATION-GUIDELINES.md"
    "IMPLEMENTATION-AUTH.md"
    "IMPLEMENTATION-BACKEND.md"
    "IMPLEMENTATION-DATA-CONTRACTS.md"
    "IMPLEMENTATION-DATABASE.md"
    "IMPLEMENTATION-FRONTEND.md"
    "IMPLEMENTATION-NGINX.md"
    "IMPLEMENTATION-SCRIPTS.md"
    "IMPLEMENTATION-TESTING.md"
    "IMPLEMENTATION-UTILS.md"
    "UX-LAYOUT.md"
)

echo -e "${BLUE}📚 Copying documentation files to .cursor/rules/ with .mdc formatting${NC}"
echo "Project root: $PROJECT_ROOT"
echo "Rules directory: $RULES_DIR"
if [[ "$CHECK_UNEXPECTED" == true ]]; then
    echo "Mode: Check unexpected files (will prompt for unexpected files)"
else
    echo "Mode: Auto-skip unexpected files (use --check-unexpected to prompt)"
fi
echo
echo -e "${YELLOW}📋 Expected documentation files to copy:${NC}"
for file in "${EXPECTED_FILES[@]}"; do
    echo "   • $file"
done
echo

# Ensure rules directory exists
if [ ! -d "$RULES_DIR" ]; then
    echo -e "${YELLOW}Creating .cursor/rules directory...${NC}"
    mkdir -p "$RULES_DIR"
fi

# Function to convert filename to description
filename_to_description() {
    local filename="$1"
    # Remove .md extension
    local basename="${filename%.md}"
    # Convert to lowercase and replace hyphens with spaces
    local description="${basename,,}"
    description="${description//-/ }"
    # Capitalize first letter of each word
    description=$(echo "$description" | sed 's/\b\w/\U&/g')
    echo "$description"
}

# Function to convert filename to .mdc filename
filename_to_mdc() {
    local filename="$1"
    # Remove .md extension and convert to lowercase
    local basename="${filename%.md}"
    echo "${basename,,}.mdc"
}

# Counter for processed files
processed=0
skipped=0
unexpected=0

# Process all .md files in project root
for md_file in "$PROJECT_ROOT"/*.md; do
    # Check if file exists (in case no .md files match the pattern)
    if [ ! -f "$md_file" ]; then
        continue
    fi
    
    # Get just the filename
    filename=$(basename "$md_file")
    
    # Check if file should be excluded
    if [[ " ${EXCLUDE_FILES[@]} " =~ " ${filename} " ]]; then
        echo -e "${YELLOW}⏭️  Skipping excluded file: $filename${NC}"
        skipped=$((skipped + 1))
        continue
    fi
    
    # Check if file is in expected list
    if [[ ! " ${EXPECTED_FILES[@]} " =~ " ${filename} " ]]; then
        if [[ "$CHECK_UNEXPECTED" == true ]]; then
            echo -e "${RED}⚠️  Unexpected file found: $filename${NC}"
            echo -e "${YELLOW}   This file wasn't in the expected documentation list.${NC}"
            echo -n "   Do you want to process this file? (y/N): "
            read -r response
            if [[ ! "$response" =~ ^[Yy]$ ]]; then
                echo -e "${YELLOW}⏭️  Skipping unexpected file: $filename${NC}"
                skipped=$((skipped + 1))
                continue
            fi
            echo -e "${GREEN}✅ Processing unexpected file: $filename${NC}"
            unexpected=$((unexpected + 1))
        else
            echo -e "${YELLOW}⏭️  Auto-skipping unexpected file: $filename${NC}"
            skipped=$((skipped + 1))
            continue
        fi
    fi
    
    # Generate .mdc filename and description
    mdc_filename=$(filename_to_mdc "$filename")
    description=$(filename_to_description "$filename")
    mdc_path="$RULES_DIR/$mdc_filename"
    
    echo -e "${BLUE}📄 Processing: $filename → $mdc_filename${NC}"
    echo "   Description: $description"
    
    # Create the .mdc file with proper formatting
    {
        echo "---"
        echo "description: $description"
        echo "globs: [\"**/*\"]"
        echo "alwaysApply: true"
        echo "---"
        echo ""
        cat "$md_file"
    } > "$mdc_path"
    
    echo -e "${GREEN}✅ Created: $mdc_path${NC}"
    echo
    processed=$((processed + 1))
done

echo -e "${GREEN}🎉 Documentation copy completed!${NC}"
echo "📊 Summary:"
echo "   • Processed: $processed files"
echo "   • Skipped: $skipped files"
if [ $unexpected -gt 0 ]; then
    echo -e "   • ${RED}Unexpected files: $unexpected${NC}"
fi
echo "   • Output directory: $RULES_DIR"
echo

# List all .mdc files in rules directory
echo -e "${BLUE}📋 Current .mdc files in .cursor/rules/:${NC}"
ls -la "$RULES_DIR"/*.mdc 2>/dev/null | while read -r line; do
    filename=$(echo "$line" | awk '{print $NF}' | xargs basename)
    echo "   • $filename"
done || echo "   (No .mdc files found)"

echo
echo -e "${GREEN}✨ All done! Your documentation is now available as Cursor rules.${NC}"
