---
description: "Read the latest prompt file from _prompts directory"
allowed-tools: ["Read", "Bash", "Glob"]
argument-hint: ""
---

# Read Latest Prompt (/rp)

Reads and displays the latest prompt file from the `_prompts` directory based on unix timestamp suffix.

## Task

Find and read the most recent prompt file by:

1. Use Glob tool to find all `.md` files in `_prompts/` directory
2. Parse the unix timestamps from filenames (format: `filename_timestamp.md`) 
3. Identify the file with the highest timestamp (most recent)
4. Use Read tool to display the complete file contents

If no files exist in `_prompts/`, inform the user that no prompt files were found.

Display the filename being read so the user knows which prompt file they're viewing.