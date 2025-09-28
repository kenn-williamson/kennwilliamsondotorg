#!/bin/bash

# Enhanced Copy Documentation to .cursor/rules Script
# Copies documentation files to .cursor/rules/ with .mdc formatting
# Default: Only core rules (coding standards, communication, architecture, dev workflow)
# Options: Add specific implementation areas as needed

set -e

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

# Default core files (always included)
CORE_FILES=(
    "CODING-RULES.md"
    "ARCHITECTURE.md"
    "DEVELOPMENT-WORKFLOW.md"
    "DOCUMENTATION-GUIDELINES.md"
)

# Optional implementation files (included with flags)
OPTIONAL_FILES=(
    "IMPLEMENTATION-FRONTEND.md"
    "IMPLEMENTATION-BACKEND.md"
    "IMPLEMENTATION-DATABASE.md"
    "IMPLEMENTATION-SECURITY.md"
    "IMPLEMENTATION-TESTING.md"
    "IMPLEMENTATION-SCRIPTS.md"
    "IMPLEMENTATION-NGINX.md"
    "IMPLEMENTATION-DATA-CONTRACTS.md"
    "IMPLEMENTATION-UTILS.md"
    "UX-LAYOUT.md"
)

# Files to always exclude
EXCLUDE_FILES=("README.md" "ROADMAP.md" "CLAUDE.md" "PROJECT_HISTORY.md")

# Parse command line arguments
FRONTEND=false
BACKEND=false
DATABASE=false
SECURITY=false
TESTING=false
SCRIPTS=false
NGINX=false
DATA_CONTRACTS=false
UTILS=false
UX=false
DOCS=false
ALL=false
CLEAR=false
CHECK_UNEXPECTED=false

show_help() {
    echo "Enhanced Copy Documentation to .cursor/rules Script"
    echo
    echo "Usage: $0 [OPTIONS]"
    echo
    echo "Default behavior: Copy only core rules (coding standards, architecture, dev workflow, documentation guidelines)"
    echo "Note: communication-standards.mdc is preserved and not overwritten"
    echo
    echo "Options:"
    echo "  -f, --frontend        Include frontend implementation docs"
    echo "  -b, --backend         Include backend implementation docs"
    echo "  -d, --database        Include database implementation docs"
    echo "  -s, --security        Include security implementation docs"
    echo "  -t, --testing         Include testing implementation docs"
    echo "  --scripts             Include scripts implementation docs"
    echo "  --nginx               Include nginx implementation docs"
    echo "  --data-contracts      Include data contracts implementation docs"
    echo "  --utils                Include utils implementation docs"
    echo "  --ux                  Include UX/layout docs"
    echo "  --docs                Include documentation guidelines"
    echo "  -a, --all             Include all implementation docs"
    echo "  -c, --clear           Clear all files except communication-standards.mdc"
    echo "  --check-unexpected    Prompt for unexpected files"
    echo "  -h, --help            Show this help message"
    echo
    echo "Examples:"
    echo "  $0                    # Copy only core rules"
    echo "  $0 -f -b             # Copy core + frontend + backend"
    echo "  $0 -t --testing      # Copy core + testing docs"
    echo "  $0 -a                # Copy everything"
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -f|--frontend)
            FRONTEND=true
            shift
            ;;
        -b|--backend)
            BACKEND=true
            shift
            ;;
        -d|--database)
            DATABASE=true
            shift
            ;;
        -s|--security)
            SECURITY=true
            shift
            ;;
        -t|--testing)
            TESTING=true
            shift
            ;;
        --scripts)
            SCRIPTS=true
            shift
            ;;
        --nginx)
            NGINX=true
            shift
            ;;
        --data-contracts)
            DATA_CONTRACTS=true
            shift
            ;;
        --utils)
            UTILS=true
            shift
            ;;
        --ux)
            UX=true
            shift
            ;;
        --docs)
            DOCS=true
            shift
            ;;
        -a|--all)
            ALL=true
            shift
            ;;
        -c|--clear)
            CLEAR=true
            shift
            ;;
        --check-unexpected)
            CHECK_UNEXPECTED=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            show_help
            exit 1
            ;;
    esac
done

# Build list of files to copy
FILES_TO_COPY=("${CORE_FILES[@]}")

if [[ "$ALL" == true ]]; then
    FILES_TO_COPY=("${CORE_FILES[@]}" "${OPTIONAL_FILES[@]}")
else
    # Add optional files based on flags
    [[ "$FRONTEND" == true ]] && FILES_TO_COPY+=("IMPLEMENTATION-FRONTEND.md")
    [[ "$BACKEND" == true ]] && FILES_TO_COPY+=("IMPLEMENTATION-BACKEND.md")
    [[ "$DATABASE" == true ]] && FILES_TO_COPY+=("IMPLEMENTATION-DATABASE.md")
    [[ "$SECURITY" == true ]] && FILES_TO_COPY+=("IMPLEMENTATION-SECURITY.md")
    [[ "$TESTING" == true ]] && FILES_TO_COPY+=("IMPLEMENTATION-TESTING.md")
    [[ "$SCRIPTS" == true ]] && FILES_TO_COPY+=("IMPLEMENTATION-SCRIPTS.md")
    [[ "$NGINX" == true ]] && FILES_TO_COPY+=("IMPLEMENTATION-NGINX.md")
    [[ "$DATA_CONTRACTS" == true ]] && FILES_TO_COPY+=("IMPLEMENTATION-DATA-CONTRACTS.md")
    [[ "$UTILS" == true ]] && FILES_TO_COPY+=("IMPLEMENTATION-UTILS.md")
    [[ "$UX" == true ]] && FILES_TO_COPY+=("UX-LAYOUT.md")
    [[ "$DOCS" == true ]] && FILES_TO_COPY+=("DOCUMENTATION-GUIDELINES.md")
fi

echo -e "${BLUE}📚 Copying documentation files to .cursor/rules/ with .mdc formatting${NC}"
echo "Project root: $PROJECT_ROOT"
echo "Rules directory: $RULES_DIR"
echo "Mode: ${ALL:+All files}${ALL:-Minimal core + selected options}"
if [[ "$CHECK_UNEXPECTED" == true ]]; then
    echo "Unexpected files: Will prompt"
else
    echo "Unexpected files: Auto-skip"
fi
echo
echo -e "${YELLOW}📋 Files to copy:${NC}"
for file in "${FILES_TO_COPY[@]}"; do
    echo "   • $file"
done
echo

# Ensure rules directory exists
if [ ! -d "$RULES_DIR" ]; then
    echo -e "${YELLOW}Creating .cursor/rules directory...${NC}"
    mkdir -p "$RULES_DIR"
fi

# Handle clear option
if [[ "$CLEAR" == true ]]; then
    echo -e "${YELLOW}🧹 Clearing all files except communication-standards.mdc...${NC}"
    for mdc_file in "$RULES_DIR"/*.mdc; do
        if [ -f "$mdc_file" ]; then
            filename=$(basename "$mdc_file")
            if [[ "$filename" != "communication-standards.mdc" ]]; then
                echo -e "${BLUE}🗑️  Removing: $filename${NC}"
                rm "$mdc_file"
            else
                echo -e "${GREEN}✅ Preserving: $filename${NC}"
            fi
        fi
    done
    echo -e "${GREEN}🎉 Clear operation completed!${NC}"
    exit 0
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

# Clear existing files (except communication-standards.mdc) before copying new ones
echo -e "${YELLOW}🧹 Clearing existing files (preserving communication-standards.mdc)...${NC}"
for mdc_file in "$RULES_DIR"/*.mdc; do
    if [ -f "$mdc_file" ]; then
        filename=$(basename "$mdc_file")
        if [[ "$filename" != "communication-standards.mdc" ]]; then
            echo -e "${BLUE}🗑️  Removing: $filename${NC}"
            rm "$mdc_file"
        fi
    fi
done
echo

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
    
    # Check if file is in our list to copy
    if [[ ! " ${FILES_TO_COPY[@]} " =~ " ${filename} " ]]; then
        if [[ "$CHECK_UNEXPECTED" == true ]]; then
            echo -e "${RED}⚠️  Unexpected file found: $filename${NC}"
            echo -e "${YELLOW}   This file wasn't selected for copying.${NC}"
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
            echo -e "${YELLOW}⏭️  Auto-skipping unselected file: $filename${NC}"
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
echo -e "${BLUE}💡 Tip: Use specific flags to add only the implementation docs you need!${NC}"