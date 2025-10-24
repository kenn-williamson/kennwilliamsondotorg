# Feature Completion Workflow

You are helping complete a feature and prepare it for commit. Follow these steps carefully:

## Step 1: Identify the Completed Feature

Analyze the following to determine what feature was completed:
1. Current git branch name
2. Git status and changed files (`git status`)
3. Recent commit messages (`git log --oneline -10`)
4. Content of changed files if needed

If the feature is clear from context (e.g., branch name like "feat/security-notifications" or clear file changes), proceed with that feature.

If ambiguous or unclear, ask the user: "What feature did you complete?" and wait for their response.

## Step 2: Update ROADMAP.md

Read the current ROADMAP.md file and:
- **Remove** the completed feature/task entirely (do NOT mark as complete with ✅)
- If a feature is only partially complete, update the description to reflect remaining work
- Preserve the document structure and formatting
- Keep only current and upcoming work in the roadmap

## Step 3: Update PROJECT_HISTORY.md

Read PROJECT_HISTORY.md and add a new milestone entry at the bottom of the "Recent Major Features" section, following this format:

```markdown
#### [Feature Name]
**Achievement**: [Brief one-line summary of what was accomplished]

**Key Deliverables**:
- [Main feature/component 1]
- [Main feature/component 2]
- [Main feature/component 3]
[etc.]

**Technical Implementation**:
- [Technical detail 1]
- [Technical detail 2]
- [Technical detail 3]
[etc.]
```

Study the existing PROJECT_HISTORY.md entries to match the tone, level of detail, and formatting.

## Step 4: Stage Appropriate Files

Run `git add` to stage the modified files, but **exclude**:
- .env.development
- .env.production
- .env.test
- Any other .env.* files
- Any files containing secrets or credentials

Stage:
- ROADMAP.md
- PROJECT_HISTORY.md
- All code/config files that were part of the feature
- Any other relevant documentation files

## Step 5: Auto-Detect Commit Type

Based on the changed files, determine the conventional commit type:
- If most changes are in docs/ or *.md files → `docs`
- If changes refactor existing code without new features → `refactor`
- If changes fix a bug → `fix`
- If changes add new functionality → `feat`
- Default to `feat` if unclear

Optionally include a scope in parentheses if clear (e.g., `feat(email)`, `docs(readme)`)

## Step 6: Create Git Commit

Use the Bash tool to create a git commit with this format:

```bash
git commit -m "$(cat <<'EOF'
[type]([scope]): [brief description]

[Optional longer description if needed]

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

**IMPORTANT**:
- Use the Bash tool to execute the commit
- The user will see the tool use and can approve/reject before it executes
- Include a clear description like "Create conventional commit for [feature name]"

## Example Output

After completing all steps, provide output like:

```
✅ Updated ROADMAP.md - removed completed feature
✅ Updated PROJECT_HISTORY.md - added "Security Notification Emails" entry
✅ Staged files (excluding .env.development)
✅ Creating commit for review...
```

Then use the Bash tool to execute the commit command. The user will see the commit message in the tool use and can approve it before it runs.

## Notes

- Always maintain the existing formatting style in ROADMAP.md and PROJECT_HISTORY.md
- Be thorough in the PROJECT_HISTORY entry - study existing entries for the right level of detail
- Never commit .env.* files
- The user will review and execute the commit command themselves
