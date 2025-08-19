---
description: "Create timestamped prompt files with terse, memorable filenames"
allowed-tools: ["Write", "Bash"]
argument-hint: "<brief description of prompt usecase>"
---

# Prompt File Generator

Creates markdown files in `_prompts/` directory with terse, memorable filenames and unix timestamps.

## Usage

`/prompt "brief description of prompt usecase"`

## Behavior

1. Generates terse filename from description (removes stop words, prioritizes technical terms)
2. Adds 10-character unix timestamp suffix
3. Creates `_prompts/{filename}_{timestamp}.md`
4. Includes template with usecase and metadata

## Examples

- `/prompt "debug bevy ecs performance"` → `_prompts/debug_bevy_perfor_1755625102.md`
- `/prompt "refactor timeline system"` → `_prompts/refact_timeli_system_1755625104.md`

## Implementation

Use the following Python script logic to generate the filename and create the file:

```python
import re
import os
import time

def generate_terse_filename(description: str) -> str:
    # Extract meaningful words, remove stop words
    words = re.findall(r'\b[a-zA-Z]+\b', description.lower())
    stop_words = {'the', 'a', 'an', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for', 'of', 'with', 'by', 'is', 'are', 'was', 'were', 'be', 'been', 'have', 'has', 'had', 'do', 'does', 'did', 'will', 'would', 'could', 'should', 'can', 'may', 'might', 'must', 'i', 'you', 'he', 'she', 'it', 'we', 'they', 'this', 'that', 'these', 'those', 'my', 'your', 'his', 'her', 'its', 'our', 'their', 'me', 'him', 'her', 'us', 'them', 'about', 'make', 'create', 'need', 'help', 'want'}
    
    meaningful_words = [word for word in words if word not in stop_words and len(word) > 2]
    
    # Prioritize technical terms
    tech_terms = {'api', 'ui', 'ux', 'css', 'html', 'js', 'py', 'rust', 'bevy', 'game', 'code', 'debug', 'test', 'build', 'deploy', 'git', 'repo', 'data', 'auth', 'user', 'admin', 'config', 'setup', 'install', 'update', 'system', 'component', 'entity', 'resource', 'plugin', 'event', 'query', 'bundle', 'asset', 'scene', 'mesh', 'material', 'shader'}
    
    priority_words = [word for word in meaningful_words if word in tech_terms or len(word) >= 5]
    
    if len(priority_words) < 3:
        priority_words.extend([word for word in meaningful_words if word not in priority_words][:3-len(priority_words)])
    
    # Take first 3 words, truncate long ones
    selected_words = priority_words[:3]
    truncated_words = [word[:6] if len(word) > 6 else word for word in selected_words]
    
    filename = '_'.join(truncated_words)
    return filename if filename and len(filename) >= 2 else 'prompt'

# Generate filename and timestamp
base_filename = generate_terse_filename("$ARGUMENTS")
timestamp = str(int(time.time()))
full_filename = f"{base_filename}_{timestamp}.md"

# Create file
os.makedirs("_prompts", exist_ok=True)
file_path = f"_prompts/{full_filename}"

with open(file_path, 'w') as f:
    f.write(f"# {base_filename}\n\n")
    f.write(f"**Usecase:** $ARGUMENTS\n\n") 
    f.write(f"**Created:** {timestamp}\n\n")
    f.write("---\n\n")
    f.write("<!-- Write your detailed prompt here -->\n\n")

print(f"Created: {file_path}")
```