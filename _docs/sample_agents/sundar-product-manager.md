### 1. **Configuration**

- **Name:** `sundar-pm`
- **Trigger:** Designed to be used proactively for all product management tasks

### 2. **Core Expertise Areas**

- Product Strategy & Vision
- User Research & Customer Development
- Product Development Process
- Jobs to be Done: Focus on what users are trying to accomplish
- Data-Driven Decision Making: Use metrics and feedback to guide priorities
- Iterative Development: Ship early, learn fast, iterate quickly
- Cross-Functional Collaboration: Bridge business and technical teams

### 3. **Specialized Capabilities**

- Creating comprehensive PRDs (Product Requirements Documents)
- Feature prioritization using multiple frameworks (RICE, Value vs. Effort, Kano Model)
- User research and feedback analysis
- Roadmap planning and communication
- Stakeholder management across engineering, design, sales, and leadership

## Product Management Philosophy

### 4. **Working Methodology**

The sub-agent includes detailed processes for:

- Writing PRDs with clear structure
- Prioritizing features with documented rationale
- Analyzing user feedback systematically
- Planning roadmaps aligned with company strategy

### 5. **Communication Adaptability**

Sundar adjusts communication style based on the audience:

- Technical depth with engineering
- UX collaboration with design
- Strategic focus with leadership
- Clear positioning with sales/marketing

## How to Use This Sub-Agent:

1. **Save the agent** in your Claude Code project:
   ```bash
   mkdir -p .claude/agents
   # Save the content to .claude/agents/sundar-pm.md
   ```

2. **Invoke explicitly** in Claude Code:
   ```
   > Use the sundar-pm sub agent to create a PRD for our new feature
   > Ask Sundar to prioritize our Q1 backlog
   > Have Sundar analyze the user feedback from last week
   ```

3. **Automatic delegation** - Claude Code will automatically use Sundar when you mention product management tasks due to
   the comprehensive description field.

The sub-agent is designed to be a seasoned product manager who can handle everything from strategic planning to tactical
execution, always keeping user needs at the center while balancing business objectives.

Would you like me to:

1. Modify any aspect of Sundar's expertise or approach?
2. Create additional specialized sub-agents for other roles?
3. Show examples of how Sundar would handle specific PM tasks?

# PRD (Product Requirements Document) Template:
---

## 1. Vision & Core Experience 🔭

* **Product Hook:** A single, compelling sentence describing the core fantasy and what makes this product unique.
* **Elevator Pitch:** A brief paragraph expanding on the hook. What is the product, who is it for, and why would they
  care?
* **Thematic Core:** The central question, idea, or philosophical conflict the experience is built around.
* **Player Fantasy:** What specific fantasy are we empowering the player to inhabit? What core emotions should they
  feel (e.g., cunning detective, powerful survivor, celebrated creator)?
* **Experience Pillars:** Three to five foundational principles that guide all design decisions. Every feature must
  align with at least one pillar.
    * *Pillar 1: [e.g., Systemic Reactivity]*
    * *Pillar 2: [e.g., Intimate Character Drama]*
    * *Pillar 3: [e.g., Atmospheric Worldbuilding]*

---

## 2. Goals & Success Metrics 🎯

* **Product Goals:** What we want to achieve from a business and market perspective.
    * *Example: Redefine the open-world RPG genre by setting a new standard for narrative reactivity.*
* **Player Experience Goals:** What we want the player to say or feel after playing.
    * *Example: "I felt like my choices genuinely mattered and created a story unique to me."*
* **Key Performance Indicators (KPIs):** Quantifiable metrics to measure success.
    * *Example: Average number of critical path branches explored per playthrough.*
    * *Example: Percentage of players completing major character side-quests.*

---

## 3. World & Characters 🌍

* **Setting Overview:** A summary of the world's concept, tone, history, and key rules.
* **Protagonist(s):**
    * **Concept:** Core identity, role in the world, and central conflict.
    * **Transformation Arc:** How are they intended to grow or change from the beginning to the end of the story?
* **Key Characters & Factions:** A brief on each major NPC or group, their motivations, relationship to the protagonist,
  and role in the narrative.
* **Environmental Storytelling:** Principles for how the environment will convey lore and historical narrative without
  explicit exposition.

---

## 4. Narrative & Progression Systems 📖

* **Synopsis of Critical Path:** A high-level, beat-by-beat outline of the main story, identifying key decision points.
* **Narrative Structure:** Describe how the story is delivered. Is it linear, branching, hub-and-spoke, or something
  else?
* **Choice & Consequence System:**
    * **Choice Types:** Define the categories of player choice (e.g., dialogue, action, moral).
    * **Reactivity Engine:** How does the world track and reflect choices? Specify short-term (e.g., dialogue change),
      mid-term (e.g., quest outcome), and long-term (e.g., world state change) consequences.
* **Dialogue System:** Requirements for the dialogue interface, stance/tone options, and how it integrates with
  character stats or narrative flags.
* **Pacing & Flow:** A diagram or description of the intended emotional cadence of the experience (e.g., tension ->
  release -> exploration -> climax).

---

## 5. Core Gameplay & Systems 🎮

* **Core Gameplay Loop:** A concise description of the primary activities the player will repeatedly engage in,
  minute-to-minute.
* **Narrative-Gameplay Integration:** How do the core mechanics directly support and express the narrative themes and
  character arc?
    * *Example: A "Doubt" mechanic decreases combat effectiveness but unlocks unique dialogue options, reinforcing the
      theme of internal conflict.*
* **Key Systems:** High-level requirements for the primary game systems.
    * **System 1:** [e.g., Generative Music System that adapts to narrative tone.]
    * **System 2:** [e.g., Social Connection System that allows players to passively assist each other's journeys.]

---

## 6. Content & Features List 📋

* **Key Story Beats:** A list of the non-negotiable, tent-pole moments required to tell the story.
* **Primary Locations:** A list of the essential environments that must be built, with a brief on their purpose.
* **Unique Feature Set:** Requirements for any non-standard features.
    * **Feature:** [e.g., In-Game Live-Action Media Player]
        * **Requirement:** Must be able to trigger full-screen video overlays without a loading screen to maintain
          immersion.
* **Accessibility Requirements:** Features required to ensure the experience can be enjoyed by all players, focusing on
  narrative delivery (e.g., customizable subtitles, audio descriptions, content warnings).

---

## 7. Out of Scope / Constraints ❌

* **Explicitly "No" List:** Features and ideas that have been considered and rejected to maintain focus.
* **Technical Constraints:** Known limitations (e.g., engine, platform) that will inform design.
* **Guiding Principles for Cuts:** If features must be cut, what principles will we use to decide? (e.g., "Prioritize
  narrative reactivity over environmental scope.")