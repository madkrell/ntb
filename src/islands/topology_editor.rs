use leptos::prelude::*;
use crate::islands::TopologyViewport;
use crate::api::{get_node, update_node, get_connection, update_connection};
use crate::models::{UpdateNode, UpdateConnection};

/// Professional topology editor layout with panels
/// Using regular component (not island) so we can share state via context
#[component]
pub fn TopologyEditor(
    /// Topology ID to edit
    #[prop(optional)]
    topology_id: Option<i64>,
) -> impl IntoView {
    // Create signals for selected state
    let selected_node_id = RwSignal::new(None::<i64>);
    let selected_item = RwSignal::new(None::<SelectedItem>);

    // Create refetch trigger - increment this to reload viewport data
    let refetch_trigger = RwSignal::new(0u32);

    // Provide signals via context so islands can access them
    provide_context(selected_node_id);
    provide_context(selected_item);
    provide_context(refetch_trigger);

    view! {
        <div class="topology-editor w-full h-screen flex flex-col bg-gray-900 text-gray-100">
            // Top Toolbar
            <TopToolbar topology_id=topology_id />

            // Main content area with 3 panels
            <div class="flex-1 flex overflow-hidden">
                // Left: Device Palette
                <DevicePalette />

                // Center: 3D Viewport (main focus, takes most space)
                <div class="flex-1 bg-gray-800 border-l border-r border-gray-700">
                    {move || {
                        match topology_id {
                            Some(id) => view! {
                                <TopologyViewport topology_id=id />
                            }.into_any(),
                            None => view! {
                                <TopologyViewport />
                            }.into_any(),
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
fn TopToolbar(
    topology_id: Option<i64>,
) -> impl IntoView {
    view! {
        <div class="h-14 bg-gray-800 border-b border-gray-700 flex items-center px-4 gap-3">
            // Logo/Title
            <div class="flex items-center gap-2 mr-6">
                <div class="text-xl font-bold text-blue-400">"NTV"</div>
                <div class="text-sm text-gray-400">
                    {move || topology_id.map(|id| format!("Topology #{}", id)).unwrap_or_else(|| "No Topology".to_string())}
                </div>
            </div>

            // Action buttons
            <button class="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded text-sm font-medium transition">
                "Add Node"
            </button>
            <button class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded text-sm font-medium transition">
                "Connect"
            </button>
            <button class="px-4 py-2 bg-red-600 hover:bg-red-700 rounded text-sm font-medium transition">
                "Delete"
            </button>

            // Spacer
            <div class="flex-1"></div>

            // Save/Export buttons
            <button class="px-4 py-2 bg-green-600 hover:bg-green-700 rounded text-sm font-medium transition">
                "Save"
            </button>
            <button class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded text-sm font-medium transition">
                "Export"
            </button>
        </div>
    }
}

/// Left device palette/toolbar
#[component]
fn DevicePalette() -> impl IntoView {
    let device_types = vec![
        ("Router", "🔀", "router"),
        ("Switch", "🔌", "switch"),
        ("Server", "🖥️", "server"),
        ("Firewall", "🛡️", "firewall"),
        ("LoadBalancer", "⚖️", "load_balancer"),
        ("Database", "🗄️", "database"),
    ];

    view! {
        <div class="w-64 bg-gray-800 border-r border-gray-700 flex flex-col">
            <div class="h-12 border-b border-gray-700 flex items-center px-4">
                <h2 class="text-sm font-semibold text-gray-300">"Device Palette"</h2>
            </div>

            <div class="flex-1 overflow-y-auto p-3 space-y-2">
                {device_types.into_iter().map(|(name, icon, type_id)| {
                    view! {
                        <button
                            class="w-full p-3 bg-gray-700 hover:bg-gray-600 rounded border border-gray-600 hover:border-blue-500 transition flex items-center gap-3 text-left"
                            data-device-type=type_id
                        >
                            <span class="text-2xl">{icon}</span>
                            <div>
                                <div class="text-sm font-medium">{name}</div>
                                <div class="text-xs text-gray-400">"Click to add"</div>
                            </div>
                        </button>
                    }
                }).collect_view()}
            </div>

            <div class="p-3 border-t border-gray-700 text-xs text-gray-400">
                "Click a device type, then click in the 3D viewport to place"
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

    // Populate signals when data loads
    Effect::new(move || {
        if let Some(Some(node)) = node_data.get() {
            name.set(node.name);
            node_type.set(node.node_type);
            ip_address.set(node.ip_address.unwrap_or_default());
            position_x.set(node.position_x);
            position_y.set(node.position_y);
            position_z.set(node.position_z);
        }
    });

    // Save action
    let save_action = Action::new(move |_: &()| {
        let update_data = UpdateNode {
            name: Some(name.get_untracked()),
            node_type: Some(node_type.get_untracked()),
            ip_address: Some(ip_address.get_untracked()).filter(|s| !s.is_empty()),
            position_x: Some(position_x.get_untracked()),
            position_y: Some(position_y.get_untracked()),
            position_z: Some(position_z.get_untracked()),
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
                                        <label class="block text-xs font-medium text-gray-400 mb-1">"Position X"</label>
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
                                        <label class="block text-xs font-medium text-gray-400 mb-1">"Position Y"</label>
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
                                        <label class="block text-xs font-medium text-gray-400 mb-1">"Position Z"</label>
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

                                <div class="pt-4 border-t border-gray-700">
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

                                <div class="pt-4 border-t border-gray-700">
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
