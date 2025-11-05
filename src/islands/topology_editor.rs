use leptos::prelude::*;
use crate::islands::TopologyViewport;
use crate::api::{get_node, update_node, get_connection, update_connection, create_node, delete_node, delete_connection, get_topologies, delete_topology, create_connection};
use crate::models::{UpdateNode, UpdateConnection, CreateNode, CreateConnection};

/// Connection creation mode state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectionMode {
    Disabled,
    SelectingFirstNode,
    SelectingSecondNode { first_node_id: i64 },
}

/// Grid and axes visibility settings
#[derive(Clone, Copy)]
pub struct ViewportVisibility {
    pub show_grid: RwSignal<bool>,
    pub show_x_axis: RwSignal<bool>,
    pub show_y_axis: RwSignal<bool>,
    pub show_z_axis: RwSignal<bool>,
}

/// Professional topology editor layout with panels
/// Using regular component (not island) so we can share state via context
#[component]
pub fn TopologyEditor(
    /// Current topology ID (as a signal for reactivity)
    current_topology_id: RwSignal<i64>,
) -> impl IntoView {
    // Create signals for selected state
    let selected_node_id = RwSignal::new(None::<i64>);
    let selected_item = RwSignal::new(None::<SelectedItem>);

    // Create refetch trigger - increment this to reload viewport data
    let refetch_trigger = RwSignal::new(0u32);

    // Connection creation mode state
    let connection_mode = RwSignal::new(ConnectionMode::Disabled);

    // Grid and axes visibility controls (wrapped in struct to avoid context collision)
    let viewport_visibility = ViewportVisibility {
        show_grid: RwSignal::new(true),
        show_x_axis: RwSignal::new(true),
        show_y_axis: RwSignal::new(true),
        show_z_axis: RwSignal::new(true),
    };

    // Provide signals via context so islands can access them
    provide_context(selected_node_id);
    provide_context(selected_item);
    provide_context(refetch_trigger);
    provide_context(current_topology_id);
    provide_context(connection_mode);
    provide_context(viewport_visibility);

    view! {
        <div class="topology-editor w-full h-screen flex flex-col bg-gray-900 text-gray-100">
            // Top Toolbar
            <TopToolbar />

            // Main content area with 3 panels
            <div class="flex-1 flex overflow-hidden">
                // Left: Device Palette
                <DevicePalette />

                // Center: 3D Viewport (main focus, takes most space)
                <div class="flex-1 bg-gray-800 border-l border-r border-gray-700">
                    {move || {
                        let topology_id = current_topology_id.get();
                        view! {
                            <TopologyViewport topology_id=topology_id />
                        }
                    }}
                </div>

                // Right: Properties Panel
                <PropertiesPanel selected_item=selected_item />
            </div>
        </div>
    }
}

/// Selected item type for properties panel
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SelectedItem {
    Node(i64),
    Connection(i64),
}

/// Top toolbar with action buttons
#[component]
fn TopToolbar() -> impl IntoView {
    // Get current topology ID from context
    let current_topology_id = use_context::<RwSignal<i64>>().expect("current_topology_id context");
    let refetch_trigger = use_context::<RwSignal<u32>>().expect("refetch_trigger context");
    let selected_item = use_context::<RwSignal<Option<SelectedItem>>>().expect("selected_item context");

    // Load list of topologies
    let topologies = Resource::new(
        || (),
        |_| async move {
            get_topologies().await.ok().unwrap_or_default()
        }
    );

    // Delete topology action
    let delete_topology_action = Action::new(move |_: &()| {
        let topology_id = current_topology_id.get_untracked();
        async move {
            delete_topology(topology_id).await
        }
    });

    // After deleting, switch to another topology if available
    Effect::new(move || {
        if let Some(Ok(_)) = delete_topology_action.value().get() {
            // Clear selection
            selected_item.set(None);
            // Refetch topologies
            topologies.refetch();
            // Switch to topology 1 if available
            current_topology_id.set(1);
            // Trigger viewport refresh
            refetch_trigger.update(|v| *v += 1);
        }
    });

    view! {
        <div class="h-14 bg-gray-800 border-b border-gray-700 flex items-center px-4 gap-3">
            // Logo/Title
            <div class="flex items-center gap-2 mr-6">
                <div class="text-xl font-bold text-blue-400">"NTV"</div>
            </div>

            // Topology Selector
            <div class="flex items-center gap-2">
                <label class="text-sm text-gray-400">"Topology:"</label>
                <Suspense fallback=move || view! { <div class="text-sm text-gray-500">"Loading..."</div> }>
                    {move || {
                        topologies.get().map(|topos| {
                            view! {
                                <select
                                    class="px-3 py-1.5 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        if let Ok(id) = value.parse::<i64>() {
                                            current_topology_id.set(id);
                                            // Clear selection when switching topologies
                                            selected_item.set(None);
                                            // Trigger viewport refresh
                                            refetch_trigger.update(|v| *v += 1);
                                        }
                                    }
                                    prop:value=move || current_topology_id.get().to_string()
                                >
                                    {topos.into_iter().map(|topo| {
                                        view! {
                                            <option value=topo.id.to_string()>
                                                {topo.name}
                                            </option>
                                        }
                                    }).collect_view()}
                                </select>
                            }
                        })
                    }}
                </Suspense>
            </div>

            // Delete Topology button
            <button
                class="px-3 py-1.5 bg-red-600 hover:bg-red-700 rounded text-sm font-medium transition disabled:opacity-50 disabled:cursor-not-allowed"
                on:click=move |_| { delete_topology_action.dispatch(()); }
                disabled=move || delete_topology_action.pending().get()
            >
                {move || if delete_topology_action.pending().get() {
                    "Deleting Topology..."
                } else {
                    "Delete Topology"
                }}
            </button>

            // Spacer
            <div class="flex-1"></div>

            // Future action buttons (disabled for now)
            <button class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded text-sm font-medium transition opacity-50 cursor-not-allowed" disabled=true>
                "Export"
            </button>
        </div>
    }
}

/// Left device palette/toolbar
#[component]
fn DevicePalette() -> impl IntoView {
    // Get context
    let current_topology_id = use_context::<RwSignal<i64>>().expect("current_topology_id context");
    let refetch_trigger = use_context::<RwSignal<u32>>().expect("refetch_trigger context");
    let connection_mode = use_context::<RwSignal<ConnectionMode>>().expect("connection_mode context");

    // Grid and axes visibility controls - extract from struct
    let viewport_visibility = use_context::<ViewportVisibility>().expect("viewport_visibility context");
    let show_grid = viewport_visibility.show_grid;
    let show_x_axis = viewport_visibility.show_x_axis;
    let show_y_axis = viewport_visibility.show_y_axis;
    let show_z_axis = viewport_visibility.show_z_axis;

    // Counter for generating unique names and positions
    let node_counter = RwSignal::new(0u32);

    // Device type configurations: (Display Name, Icon, type_id, name_prefix)
    let device_types = vec![
        ("Router", "🔀", "router", "Router"),
        ("Switch", "🔌", "switch", "Switch"),
        ("Server", "🖥️", "server", "Server"),
        ("Firewall", "🛡️", "firewall", "Firewall"),
        ("Load Balancer", "⚖️", "load_balancer", "LoadBalancer"),
        ("Cloud", "☁️", "cloud", "Cloud"),
    ];

    // Action to create a node
    let create_node_action = Action::new(move |(node_type, name_prefix): &(String, String)| {
        let node_type = node_type.clone();
        let name_prefix = name_prefix.clone();

        async move {
            // Get current topology_id
            let tid = current_topology_id.get_untracked();

            // Increment counter for unique name and position
            let count = node_counter.get_untracked();
            node_counter.update(|c| *c += 1);

            // Generate unique name
            let name = format!("{}-{}", name_prefix, count + 1);

            // Calculate position in a grid to avoid overlap
            // Grid: 5 columns, spacing of 3.0 units
            let col = (count % 5) as f64;
            let row = (count / 5) as f64;
            let position_x = col * 3.0 - 6.0;  // Center the grid around origin
            let position_y = 0.0;               // On the floor
            let position_z = row * 3.0 - 3.0;  // Rows going back

            // Create node data
            let data = CreateNode {
                topology_id: tid,
                name,
                node_type,
                ip_address: None,
                position_x: Some(position_x),
                position_y: Some(position_y),
                position_z: Some(position_z),
                rotation_x: None, // Will use default 90°
                rotation_y: None, // Will use default 0°
                rotation_z: None, // Will use default 0°
                metadata: None,
            };

            // Call server function
            create_node(data).await
        }
    });

    // Trigger viewport refetch on successful node creation
    Effect::new(move || {
        if let Some(Ok(_)) = create_node_action.value().get() {
            refetch_trigger.update(|v| *v += 1);
        }
    });

    view! {
        <div class="w-64 bg-gray-800 border-r border-gray-700 flex flex-col">
            <div class="h-12 border-b border-gray-700 flex items-center px-4">
                <h2 class="text-sm font-semibold text-gray-300">"Device Palette"</h2>
            </div>

            // Connection creation button
            <div class="p-3 border-b border-gray-700">
                <button
                    class="w-full p-3 rounded border transition flex items-center gap-3 text-left"
                    class:bg-purple-600=move || connection_mode.get() != ConnectionMode::Disabled
                    class:hover:bg-purple-700=move || connection_mode.get() != ConnectionMode::Disabled
                    class:border-purple-500=move || connection_mode.get() != ConnectionMode::Disabled
                    class:bg-gray-700=move || connection_mode.get() == ConnectionMode::Disabled
                    class:hover:bg-gray-600=move || connection_mode.get() == ConnectionMode::Disabled
                    class:border-gray-600=move || connection_mode.get() == ConnectionMode::Disabled
                    on:click=move |_| {
                        let current_mode = connection_mode.get();
                        if current_mode == ConnectionMode::Disabled {
                            connection_mode.set(ConnectionMode::SelectingFirstNode);
                        } else {
                            connection_mode.set(ConnectionMode::Disabled);
                        }
                    }
                >
                    <span class="text-2xl">"🔗"</span>
                    <div class="flex-1">
                        <div class="text-sm font-medium">"Connect Nodes"</div>
                        <div class="text-xs text-gray-400">
                            {move || {
                                match connection_mode.get() {
                                    ConnectionMode::Disabled => "Click to activate",
                                    ConnectionMode::SelectingFirstNode => "Select first node",
                                    ConnectionMode::SelectingSecondNode { .. } => "Select second node",
                                }
                            }}
                        </div>
                    </div>
                </button>
            </div>

            <div class="flex-1 overflow-y-auto p-3 space-y-2">
                {device_types.into_iter().map(|(display_name, icon, type_id, name_prefix)| {
                    let type_id_clone = type_id.to_string();
                    let name_prefix_clone = name_prefix.to_string();

                    view! {
                        <button
                            class="w-full p-3 bg-gray-700 hover:bg-gray-600 rounded border border-gray-600 hover:border-blue-500 transition flex items-center gap-3 text-left disabled:opacity-50 disabled:cursor-not-allowed"
                            on:click=move |_| {
                                create_node_action.dispatch((type_id_clone.clone(), name_prefix_clone.clone()));
                            }
                            disabled=move || create_node_action.pending().get()
                        >
                            <span class="text-2xl">{icon}</span>
                            <div class="flex-1">
                                <div class="text-sm font-medium">{display_name}</div>
                                <div class="text-xs text-gray-400">
                                    {move || {
                                        if create_node_action.pending().get() {
                                            "Adding..."
                                        } else {
                                            "Click to add"
                                        }
                                    }}
                                </div>
                            </div>
                        </button>
                    }
                }).collect_view()}
            </div>

            // Show feedback for the last action
            <div class="px-3 py-2 border-t border-gray-700">
                {move || {
                    create_node_action.value().get().map(|result| {
                        match result {
                            Ok(node) => view! {
                                <div class="text-xs text-green-400">
                                    {format!("✓ Added: {}", node.name)}
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <div class="text-xs text-red-400">
                                    {format!("Error: {}", e)}
                                </div>
                            }.into_any(),
                        }
                    })
                }}
            </div>

            // Grid and Axes visibility controls
            <div class="p-3 border-t border-gray-700">
                <div class="text-xs font-semibold text-gray-300 mb-2">"View Controls"</div>
                <div class="space-y-2">
                    <div class="flex items-center gap-2 text-xs text-gray-300">
                        <button
                            class="px-2 py-1 rounded text-xs border transition"
                            class:bg-blue-600=move || show_grid.get()
                            class:border-blue-500=move || show_grid.get()
                            class:bg-gray-700=move || !show_grid.get()
                            class:border-gray-600=move || !show_grid.get()
                            on:click=move |_| show_grid.update(|v| *v = !*v)
                        >
                            "Grid Floor"
                        </button>
                    </div>
                    <div class="flex items-center gap-2 text-xs text-gray-300">
                        <button
                            class="px-2 py-1 rounded text-xs border transition"
                            class:bg-red-600=move || show_x_axis.get()
                            class:border-red-500=move || show_x_axis.get()
                            class:bg-gray-700=move || !show_x_axis.get()
                            class:border-gray-600=move || !show_x_axis.get()
                            on:click=move |_| show_x_axis.update(|v| *v = !*v)
                        >
                            "X Axis (Red)"
                        </button>
                    </div>
                    <div class="flex items-center gap-2 text-xs text-gray-300">
                        <button
                            class="px-2 py-1 rounded text-xs border transition"
                            class:bg-green-600=move || show_y_axis.get()
                            class:border-green-500=move || show_y_axis.get()
                            class:bg-gray-700=move || !show_y_axis.get()
                            class:border-gray-600=move || !show_y_axis.get()
                            on:click=move |_| show_y_axis.update(|v| *v = !*v)
                        >
                            "Y Axis (Green)"
                        </button>
                    </div>
                    <div class="flex items-center gap-2 text-xs text-gray-300">
                        <button
                            class="px-2 py-1 rounded text-xs border transition"
                            class:bg-blue-600=move || show_z_axis.get()
                            class:border-blue-500=move || show_z_axis.get()
                            class:bg-gray-700=move || !show_z_axis.get()
                            class:border-gray-600=move || !show_z_axis.get()
                            on:click=move |_| show_z_axis.update(|v| *v = !*v)
                        >
                            "Z Axis (Blue)"
                        </button>
                    </div>
                </div>
            </div>

            <div class="p-3 border-t border-gray-700 text-xs text-gray-400">
                "Nodes are added in a grid pattern at the origin"
            </div>
        </div>
    }
}

/// Right properties panel
#[component]
fn PropertiesPanel(
    selected_item: RwSignal<Option<SelectedItem>>,
) -> impl IntoView {
    view! {
        <div class="w-80 bg-gray-800 border-l border-gray-700 flex flex-col">
            <div class="h-12 border-b border-gray-700 flex items-center px-4">
                <h2 class="text-sm font-semibold text-gray-300">"Properties"</h2>
            </div>

            <div class="flex-1 overflow-y-auto p-4">
                {move || {
                    match selected_item.get() {
                        Some(SelectedItem::Node(id)) => view! {
                            <NodeProperties node_id=id />
                        }.into_any(),
                        Some(SelectedItem::Connection(id)) => view! {
                            <ConnectionProperties connection_id=id />
                        }.into_any(),
                        None => view! {
                            <div class="text-center text-gray-500 mt-8">
                                <div class="text-4xl mb-2">"📋"</div>
                                <p class="text-sm">"Select a node or connection to view properties"</p>
                            </div>
                        }.into_any(),
                    }
                }}
            </div>
        </div>
    }
}

/// Node properties editor with live data loading and saving
#[component]
fn NodeProperties(node_id: i64) -> impl IntoView {
    // Get refetch trigger from context
    let refetch_trigger = use_context::<RwSignal<u32>>().expect("refetch_trigger context");

    // Get selected_item from context to clear selection after deletion
    let selected_item = use_context::<RwSignal<Option<SelectedItem>>>().expect("selected_item context");

    // Load node data from server
    let node_data = Resource::new(
        move || node_id,
        |id| async move {
            get_node(id).await.ok()
        }
    );

    // Create signals for editable fields
    let name = RwSignal::new(String::new());
    let node_type = RwSignal::new(String::new());
    let ip_address = RwSignal::new(String::new());
    let position_x = RwSignal::new(0.0);
    let position_y = RwSignal::new(0.0);
    let position_z = RwSignal::new(0.0);
    let rotation_x = RwSignal::new(0.0);
    let rotation_y = RwSignal::new(0.0);
    let rotation_z = RwSignal::new(0.0);

    // Populate signals when data loads
    // NOTE: Swap Y and Z to match Blender convention in UI
    // Database stores: position_y (vertical in DB), position_z (depth in DB)
    // UI shows: Position Y (horizontal green), Position Z (vertical blue)
    Effect::new(move || {
        if let Some(Some(node)) = node_data.get() {
            name.set(node.name);
            node_type.set(node.node_type);
            ip_address.set(node.ip_address.unwrap_or_default());
            position_x.set(node.position_x);
            position_y.set(node.position_z);  // UI Y ← DB Z (horizontal)
            position_z.set(node.position_y);  // UI Z ← DB Y (vertical)
            rotation_x.set(node.rotation_x);
            rotation_y.set(node.rotation_y);
            rotation_z.set(node.rotation_z);
        }
    });

    // Save action
    // NOTE: Swap Y and Z back when saving to database
    // UI Position Y (green, horizontal) → DB position_z
    // UI Position Z (blue, vertical) → DB position_y
    let save_action = Action::new(move |_: &()| {
        let update_data = UpdateNode {
            name: Some(name.get_untracked()),
            node_type: Some(node_type.get_untracked()),
            ip_address: Some(ip_address.get_untracked()).filter(|s| !s.is_empty()),
            position_x: Some(position_x.get_untracked()),
            position_y: Some(position_z.get_untracked()),  // DB Y ← UI Z (vertical)
            position_z: Some(position_y.get_untracked()),  // DB Z ← UI Y (horizontal)
            rotation_x: Some(rotation_x.get_untracked()),
            rotation_y: Some(rotation_y.get_untracked()),
            rotation_z: Some(rotation_z.get_untracked()),
            metadata: None,
        };

        async move {
            update_node(node_id, update_data).await
        }
    });

    // Trigger viewport refetch on successful save
    Effect::new(move || {
        if let Some(Ok(_)) = save_action.value().get() {
            // Increment trigger to cause viewport to refetch
            refetch_trigger.update(|v| *v += 1);
        }
    });

    // Delete action
    let delete_action = Action::new(move |_: &()| {
        async move {
            delete_node(node_id).await
        }
    });

    // Clear selection and trigger refetch on successful deletion
    Effect::new(move || {
        if let Some(Ok(_)) = delete_action.value().get() {
            // Clear selection
            selected_item.set(None);
            // Trigger viewport refetch
            refetch_trigger.update(|v| *v += 1);
        }
    });

    view! {
        <div class="space-y-4">
            <Suspense fallback=move || view! {
                <div class="text-center text-gray-500 mt-8">
                    <div class="text-2xl mb-2">"⏳"</div>
                    <p class="text-sm">"Loading node data..."</p>
                </div>
            }>
                {move || {
                    node_data.get().map(|data| {
                        match data {
                            Some(_) => view! {
                            <div class="space-y-4">
                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"Node ID"</label>
                                    <div class="text-sm text-gray-300">{node_id}</div>
                                </div>

                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"Name"</label>
                                    <input
                                        type="text"
                                        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                        placeholder="Node name"
                                        prop:value=move || name.get()
                                        on:input=move |ev| name.set(event_target_value(&ev))
                                    />
                                </div>

                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"Type"</label>
                                    <select
                                        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                        prop:value=move || node_type.get()
                                        on:change=move |ev| node_type.set(event_target_value(&ev))
                                    >
                                        <option value="router">"Router"</option>
                                        <option value="switch">"Switch"</option>
                                        <option value="server">"Server"</option>
                                        <option value="firewall">"Firewall"</option>
                                        <option value="load_balancer">"Load Balancer"</option>
                                        <option value="database">"Database"</option>
                                    </select>
                                </div>

                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"IP Address"</label>
                                    <input
                                        type="text"
                                        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                        placeholder="192.168.1.1"
                                        prop:value=move || ip_address.get()
                                        on:input=move |ev| ip_address.set(event_target_value(&ev))
                                    />
                                </div>

                                <div class="grid grid-cols-3 gap-2">
                                    <div>
                                        <label class="block text-xs font-medium text-red-400 mb-1">"Position X"</label>
                                        <input
                                            type="number"
                                            class="w-full px-2 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                            step="0.1"
                                            prop:value=move || position_x.get()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                    position_x.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-xs font-medium text-green-400 mb-1">"Position Y"</label>
                                        <input
                                            type="number"
                                            class="w-full px-2 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                            step="0.1"
                                            prop:value=move || position_y.get()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                    position_y.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-xs font-medium text-blue-400 mb-1">"Position Z"</label>
                                        <input
                                            type="number"
                                            class="w-full px-2 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                            step="0.1"
                                            prop:value=move || position_z.get()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                    position_z.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                </div>

                                <div class="grid grid-cols-3 gap-2">
                                    <div>
                                        <label class="block text-xs font-medium text-red-400 mb-1">"Rotation X"</label>
                                        <input
                                            type="number"
                                            class="w-full px-2 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                            step="1"
                                            min="-180"
                                            max="180"
                                            prop:value=move || rotation_x.get()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                    rotation_x.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-xs font-medium text-green-400 mb-1">"Rotation Y"</label>
                                        <input
                                            type="number"
                                            class="w-full px-2 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                            step="1"
                                            min="-180"
                                            max="180"
                                            prop:value=move || rotation_y.get()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                    rotation_y.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-xs font-medium text-blue-400 mb-1">"Rotation Z"</label>
                                        <input
                                            type="number"
                                            class="w-full px-2 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                            step="1"
                                            min="-180"
                                            max="180"
                                            prop:value=move || rotation_z.get()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                    rotation_z.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                </div>

                                <div class="pt-4 border-t border-gray-700 space-y-3">
                                    <button
                                        class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded text-sm font-medium transition disabled:opacity-50 disabled:cursor-not-allowed"
                                        on:click=move |_| { save_action.dispatch(()); }
                                        disabled=move || save_action.pending().get()
                                    >
                                        {move || if save_action.pending().get() {
                                            "Saving..."
                                        } else {
                                            "Save Changes"
                                        }}
                                    </button>

                                    // Show save result
                                    {move || {
                                        save_action.value().get().map(|result| {
                                            match result {
                                                Ok(_) => view! {
                                                    <div class="mt-2 text-xs text-green-400 text-center">
                                                        "✓ Saved successfully"
                                                    </div>
                                                }.into_any(),
                                                Err(e) => view! {
                                                    <div class="mt-2 text-xs text-red-400 text-center">
                                                        {format!("Error: {}", e)}
                                                    </div>
                                                }.into_any(),
                                            }
                                        })
                                    }}

                                    // Delete button
                                    <button
                                        class="w-full px-4 py-2 bg-red-600 hover:bg-red-700 rounded text-sm font-medium transition disabled:opacity-50 disabled:cursor-not-allowed mt-4"
                                        on:click=move |_| { delete_action.dispatch(()); }
                                        disabled=move || delete_action.pending().get()
                                    >
                                        {move || if delete_action.pending().get() {
                                            "Deleting..."
                                        } else {
                                            "Delete Node"
                                        }}
                                    </button>

                                    // Show delete result
                                    {move || {
                                        delete_action.value().get().map(|result| {
                                            match result {
                                                Ok(_) => view! {
                                                    <div class="text-xs text-green-400 text-center">
                                                        "✓ Node deleted"
                                                    </div>
                                                }.into_any(),
                                                Err(e) => view! {
                                                    <div class="text-xs text-red-400 text-center">
                                                        {format!("Error: {}", e)}
                                                    </div>
                                                }.into_any(),
                                            }
                                        })
                                    }}
                                </div>
                            </div>
                        }.into_any(),
                        None => view! {
                            <div class="text-center text-gray-500 mt-8">
                                <div class="text-2xl mb-2">"❌"</div>
                                <p class="text-sm">"Failed to load node data"</p>
                            </div>
                        }.into_any(),
                    }
                })
            }}
            </Suspense>
        </div>
    }
}

/// Connection properties editor with live data loading and saving
#[component]
fn ConnectionProperties(connection_id: i64) -> impl IntoView {
    // Get refetch trigger from context
    let refetch_trigger = use_context::<RwSignal<u32>>().expect("refetch_trigger context");

    // Get selected_item from context to clear selection after deletion
    let selected_item = use_context::<RwSignal<Option<SelectedItem>>>().expect("selected_item context");

    // Load connection data from server
    let connection_data = Resource::new(
        move || connection_id,
        |id| async move {
            get_connection(id).await.ok()
        }
    );

    // Create signals for editable fields
    let connection_type = RwSignal::new(String::new());
    let bandwidth_mbps = RwSignal::new(0i64);
    let latency_ms = RwSignal::new(0.0f64);
    let status = RwSignal::new(String::new());

    // Populate signals when data loads
    Effect::new(move || {
        if let Some(Some(connection)) = connection_data.get() {
            connection_type.set(connection.connection_type);
            bandwidth_mbps.set(connection.bandwidth_mbps.unwrap_or(0));
            latency_ms.set(connection.latency_ms.unwrap_or(0.0));
            status.set(connection.status);
        }
    });

    // Save action
    let save_action = Action::new(move |_: &()| {
        let update_data = UpdateConnection {
            connection_type: Some(connection_type.get_untracked()),
            bandwidth_mbps: Some(bandwidth_mbps.get_untracked()).filter(|&v| v > 0),
            latency_ms: Some(latency_ms.get_untracked()).filter(|&v| v > 0.0),
            status: Some(status.get_untracked()),
            metadata: None,
        };

        async move {
            update_connection(connection_id, update_data).await
        }
    });

    // Trigger viewport refetch on successful save
    Effect::new(move || {
        if let Some(Ok(_)) = save_action.value().get() {
            refetch_trigger.update(|v| *v += 1);
        }
    });

    // Delete action
    let delete_action = Action::new(move |_: &()| {
        async move {
            delete_connection(connection_id).await
        }
    });

    // Clear selection and trigger refetch on successful deletion
    Effect::new(move || {
        if let Some(Ok(_)) = delete_action.value().get() {
            // Clear selection
            selected_item.set(None);
            // Trigger viewport refetch
            refetch_trigger.update(|v| *v += 1);
        }
    });

    view! {
        <div class="space-y-4">
            <Suspense fallback=move || view! {
                <div class="text-center text-gray-500 mt-8">
                    <div class="text-2xl mb-2">"⏳"</div>
                    <p class="text-sm">"Loading connection data..."</p>
                </div>
            }>
                {move || {
                    connection_data.get().map(|data| {
                        match data {
                            Some(connection) => view! {
                            <div class="space-y-4">
                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"Connection ID"</label>
                                    <div class="text-sm text-gray-300">{connection_id}</div>
                                </div>

                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"Source Node"</label>
                                    <div class="text-sm text-gray-300">"Node #{"{connection.source_node_id}"}"</div>
                                </div>

                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"Target Node"</label>
                                    <div class="text-sm text-gray-300">"Node #{"{connection.target_node_id}"}"</div>
                                </div>

                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"Type"</label>
                                    <select
                                        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                        prop:value=move || connection_type.get()
                                        on:change=move |ev| connection_type.set(event_target_value(&ev))
                                    >
                                        <option value="ethernet">"Ethernet"</option>
                                        <option value="fiber">"Fiber"</option>
                                        <option value="wireless">"Wireless"</option>
                                    </select>
                                </div>

                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"Bandwidth (Mbps)"</label>
                                    <input
                                        type="number"
                                        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                        placeholder="1000"
                                        prop:value=move || bandwidth_mbps.get()
                                        on:input=move |ev| {
                                            if let Ok(val) = event_target_value(&ev).parse::<i64>() {
                                                bandwidth_mbps.set(val);
                                            }
                                        }
                                    />
                                </div>

                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"Latency (ms)"</label>
                                    <input
                                        type="number"
                                        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                        placeholder="10"
                                        step="0.1"
                                        prop:value=move || latency_ms.get()
                                        on:input=move |ev| {
                                            if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                latency_ms.set(val);
                                            }
                                        }
                                    />
                                </div>

                                <div>
                                    <label class="block text-xs font-medium text-gray-400 mb-1">"Status"</label>
                                    <select
                                        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-sm focus:outline-none focus:border-blue-500"
                                        prop:value=move || status.get()
                                        on:change=move |ev| status.set(event_target_value(&ev))
                                    >
                                        <option value="active">"Active"</option>
                                        <option value="inactive">"Inactive"</option>
                                        <option value="error">"Error"</option>
                                    </select>
                                </div>

                                <div class="pt-4 border-t border-gray-700 space-y-3">
                                    <button
                                        class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded text-sm font-medium transition disabled:opacity-50 disabled:cursor-not-allowed"
                                        on:click=move |_| { save_action.dispatch(()); }
                                        disabled=move || save_action.pending().get()
                                    >
                                        {move || if save_action.pending().get() {
                                            "Saving..."
                                        } else {
                                            "Save Changes"
                                        }}
                                    </button>

                                    // Show save result
                                    {move || {
                                        save_action.value().get().map(|result| {
                                            match result {
                                                Ok(_) => view! {
                                                    <div class="mt-2 text-xs text-green-400 text-center">
                                                        "✓ Saved successfully"
                                                    </div>
                                                }.into_any(),
                                                Err(e) => view! {
                                                    <div class="mt-2 text-xs text-red-400 text-center">
                                                        {format!("Error: {}", e)}
                                                    </div>
                                                }.into_any(),
                                            }
                                        })
                                    }}

                                    // Delete button
                                    <button
                                        class="w-full px-4 py-2 bg-red-600 hover:bg-red-700 rounded text-sm font-medium transition disabled:opacity-50 disabled:cursor-not-allowed mt-4"
                                        on:click=move |_| { delete_action.dispatch(()); }
                                        disabled=move || delete_action.pending().get()
                                    >
                                        {move || if delete_action.pending().get() {
                                            "Deleting..."
                                        } else {
                                            "Delete Connection"
                                        }}
                                    </button>

                                    // Show delete result
                                    {move || {
                                        delete_action.value().get().map(|result| {
                                            match result {
                                                Ok(_) => view! {
                                                    <div class="text-xs text-green-400 text-center">
                                                        "✓ Connection deleted"
                                                    </div>
                                                }.into_any(),
                                                Err(e) => view! {
                                                    <div class="text-xs text-red-400 text-center">
                                                        {format!("Error: {}", e)}
                                                    </div>
                                                }.into_any(),
                                            }
                                        })
                                    }}
                                </div>
                            </div>
                        }.into_any(),
                        None => view! {
                            <div class="text-center text-gray-500 mt-8">
                                <div class="text-2xl mb-2">"❌"</div>
                                <p class="text-sm">"Failed to load connection data"</p>
                            </div>
                        }.into_any(),
                    }
                })
            }}
            </Suspense>
        </div>
    }
}
