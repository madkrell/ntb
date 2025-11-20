# Simpler Connection Creation Approach

## Implementation Plan

###Step 1: Clean up old ConnectionMode system ✅ IN PROGRESS
- [x] Remove ConnectionMode enum from topology_editor.rs
- [x] Remove connection_mode signal and context
- [x] Remove "Connect Nodes" button from Device Palette
- [ ] Remove connection_mode parameters from all viewport functions
- [ ] Remove connection mode switch logic from mouseup event handler

### Step 2: Add connection dropdown to Properties Panel
When a node is selected, add new section to Node Properties:

```rust
// In PropertiesPanel component, after node properties
<div class="mb-4">
    <label class="block text-xs font-medium text-gray-400 mb-1">
        "Create Connection"
    </label>
    <select class="w-full px-2 py-1 bg-gray-700 border border-gray-600...">
        <option value="">"-- Select target node --"</option>
        // List all OTHER nodes in topology
        <option value="{node.id}">{node.name}</option>
    </select>
    <button class="mt-2 w-full px-3 py-1.5 bg-blue-600..." on:click=create_connection>
        "Create Connection"
    </button>
</div>
```

### Step 3: Implement connection creation action
```rust
let create_connection_action = Action::new(move |(source_id, target_id): &(i64, i64)| {
    async move {
        let data = CreateConnection {
            topology_id: current_topology_id.get_untracked(),
            source_node_id: *source_id,
            target_node_id: *target_id,
            connection_type: Some("ethernet".to_string()),
            bandwidth_mbps: Some(1000),
            latency_ms: Some(1.0),
            baseline_packet_loss_pct: Some(0.0),
            status: Some("active".to_string()),
            color: None,
            metadata: None,
        };
        create_connection_fn(data).await
    }
});

Effect::new(move || {
    if let Some(Ok(_)) = create_connection_action.value().get() {
        refetch_trigger.update(|v| *v += 1);
    }
});
```

## Benefits

1. **Simpler code** - No state machine, no complex event handler logic
2. **More reliable** - Works even if 3D viewport has issues
3. **Better UX** - Can see all available nodes, no "mode" confusion
4. **Easier to extend** - Can add connection type/bandwidth selection before creating
5. **Less error-prone** - Dropdown validates nodes exist

## What Stays the Same

- All database schema (same `connections` table)
- Same `create_connection()` server function
- All connection properties (bandwidth, latency, etc.)
- Traffic visualization and animations
- Source/target direction
- All connection editing in Properties Panel

## What Changes

- **Only the creation workflow:**
  - OLD: Click button → click node 1 → click node 2
  - NEW: Click node 1 → select node 2 from dropdown → click Create button
