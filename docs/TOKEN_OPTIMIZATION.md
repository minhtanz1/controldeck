# AI Token Optimization Guide (MCP + Graphify + Justless + RTK)

Developing modern software with LLMs (Claude, Gemini, GPT) can quickly consume millions of tokens if agents read entire files, dump full terminal outputs, or recursively search directory trees.

ControlDeck is engineered with a **4-tier Token Optimization Architecture** that reduces context window consumption by up to **90%**.

---

## 1. Token Savings Comparison

| Workflow Step | Standard Agent Flow | ControlDeck Optimized Flow | Token Reduction |
| :--- | :--- | :--- | :--- |
| **Locating a Route / Function** | Agent reads 5-10 files (`~15,000 tokens`) | `search_graph` MCP query (`~250 tokens`) | **~98%** |
| **Tracing Cross-Module Calls** | Grepping across folders & reading modules (`~20,000 tokens`) | `trace_path` or `graphify path` (`~300 tokens`) | **~98%** |
| **Inspecting Implementation** | Loading entire 1,000-line file (`~4,000 tokens`) | `get_code_snippet` or `justless code` (`~300 tokens`) | **~92%** |
| **Running Terminal Commands** | Unfiltered build/test logs (`~8,000 tokens`) | RTK / Justless compact proxy (`~500 tokens`) | **~93%** |

---

## 2. Tool Integration Matrix

### 1. `codebase-memory-mcp` (AST Knowledge Graph)
- **Role**: High-precision indexed code graph.
- **Project Name**: `D-waze-controldeck-prod`
- **Key MCP Tools**:
  - `search_graph(project="D-waze-controldeck-prod", name_pattern="...")`: Instant symbol and route lookup.
  - `trace_path(project="D-waze-controldeck-prod", function_name="...", direction="inbound"|"outbound")`: Call-chain tracking.
  - `get_code_snippet(project="D-waze-controldeck-prod", qualified_name="...")`: Reads solely the requested function body.

### 2. `graphify` (Visual & High-Level Call Flow)
- **Role**: Structural clustering and topological understanding.
- **Location**: Pre-built in `graphify-out/`.
- **Key Commands**:
  - `graphify explain "<Symbol>"`: Plain-language explanation of a node and its 20 closest connections.
  - `graphify path "<A>" "<B>"`: Shortest dependency path between two components.
  - Interactive Visualizers: Open `graphify-out/callflow.html` or `graphify-out/graph.html` for human and agent architectural review.

### 3. `justless` (Surgical Retrieval & Output Compactor)
- **Role**: Command execution archiver and targeted code retriever.
- **Location**: Initialized in `.justless/`.
- **Key Commands**:
  - `justless lookup "pattern"`: Retrieves ranked matching code/doc snippets.
  - `justless pack "<task description>"`: Gathers exact relevant files for a task.
  - `justless run -- <command>`: Captures verbose execution logs to an archived run reference rather than blowing the model context.

### 4. `RTK` (Rust Token Killer)
- **Role**: Universal CLI transparent filtering proxy.
- **Rules**: Applied globally via `AGENTS.md` and user rules.
- **Outcome**: Trims verbose progress bars, repetitive warnings, and redundant compilation logs from `cargo`, `npm`, and `git`.

---

## 3. Recommended Prompting Protocol for Developers

When prompting an AI assistant on this repository, you can provide this concise instruction:

```
Please follow the guidelines in AGENTS.md.
Use codebase-memory-mcp or justless for all code queries to conserve tokens.
Do not read full files unless necessary.
```
