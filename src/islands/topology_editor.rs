use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::islands::TopologyViewport;
use crate::api::{get_node, update_node, get_connection, update_connection, create_node, delete_node, delete_connection, get_topologies, delete_topology, get_ui_settings, update_ui_settings};
use crate::models::{UpdateNode, UpdateConnection, CreateNode, UpdateUISettings};

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

/// Lighting settings for the 3D viewport
#[derive(Clone, Copy)]
pub struct LightingSettings {
    pub ambient_intensity: RwSignal<f32>,
    pub key_light_intensity: RwSignal<f32>,
    pub fill_light_intensity: RwSignal<f32>,
    pub rim_light_intensity: RwSignal<f32>,
}

/// Camera preset for quick navigation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraPreset {
    Top,
    Front,
    Side,
    Isometric,
    Reset,
}

/// Camera control commands
#[derive(Clone, Copy)]
pub struct CameraControls {
    pub preset_trigger: RwSignal<Option<CameraPreset>>,
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
    // Initialize with defaults, will be updated from database
    let viewport_visibility = ViewportVisibility {
        show_grid: RwSignal::new(true),
        show_x_axis: RwSignal::new(true),
        show_y_axis: RwSignal::new(true),
        show_z_axis: RwSignal::new(true),
    };

    // Lighting settings (wrapped in struct to avoid context collision)
    // Initialize with defaults, will be updated from database
    let lighting_settings = LightingSettings {
        ambient_intensity: RwSignal::new(0.4),
        key_light_intensity: RwSignal::new(1.5),
        fill_light_intensity: RwSignal::new(0.6),
        rim_light_intensity: RwSignal::new(0.3),
    };

    // Camera controls (wrapped in struct to avoid context collision)
    let camera_controls = CameraControls {
        preset_trigger: RwSignal::new(None),
    };

    // Provide signals via context so islands can access them
    provide_context(selected_node_id);
    provide_context(selected_item);
    provide_context(refetch_trigger);
    provide_context(current_topology_id);
    provide_context(connection_mode);
    provide_context(viewport_visibility);
    provide_context(lighting_settings);
    provide_context(camera_controls);

    // Load UI settings from database and update signals
    let ui_settings_resource = Resource::new(
        || (),
        |_| async move {
            get_ui_settings().await.ok()
        }
    );

    // Effect: Update signals when settings load from database
    Effect::new(move || {
        if let Some(Some(settings)) = ui_settings_resource.get() {
            // Update viewport visibility
            viewport_visibility.show_grid.set(settings.show_grid);
            viewport_visibility.show_x_axis.set(settings.show_x_axis);
            viewport_visibility.show_y_axis.set(settings.show_y_axis);
            viewport_visibility.show_z_axis.set(settings.show_z_axis);

            // Update lighting settings
            lighting_settings.ambient_intensity.set(settings.ambient_intensity as f32);
            lighting_settings.key_light_intensity.set(settings.key_light_intensity as f32);
            lighting_settings.fill_light_intensity.set(settings.fill_light_intensity as f32);
            lighting_settings.rim_light_intensity.set(settings.rim_light_intensity as f32);

            // Trigger viewport refresh to apply loaded settings
            refetch_trigger.update(|v| *v += 1);
        }
    });

    // Effect: Save viewport visibility settings when they change
    Effect::new(move || {
        // Track all visibility signals
        let grid = viewport_visibility.show_grid.get();
        let x = viewport_visibility.show_x_axis.get();
        let y = viewport_visibility.show_y_axis.get();
        let z = viewport_visibility.show_z_axis.get();

        // Skip save on initial mount (settings just loaded)
        if ui_settings_resource.get().is_some() {
            spawn_local(async move {
                let data = UpdateUISettings {
                    show_grid: Some(grid),
                    show_x_axis: Some(x),
                    show_y_axis: Some(y),
                    show_z_axis: Some(z),
                    ambient_intensity: None,
                    key_light_intensity: None,
                    fill_light_intensity: None,
                    rim_light_intensity: None,
                };
                let _ = update_ui_settings(data).await;
            });
        }
    });

    // Effect: Save lighting settings when they change
    Effect::new(move || {
        // Track all lighting signals
        let ambient = lighting_settings.ambient_intensity.get();
        let key = lighting_settings.key_light_intensity.get();
        let fill = lighting_settings.fill_light_intensity.get();
        let rim = lighting_settings.rim_light_intensity.get();

        // Skip save on initial mount (settings just loaded)
        if ui_settings_resource.get().is_some() {
            spawn_local(async move {
                let data = UpdateUISettings {
                    show_grid: None,
                    show_x_axis: None,
                    show_y_axis: None,
                    show_z_axis: None,
                    ambient_intensity: Some(ambient as f64),
                    key_light_intensity: Some(key as f64),
                    fill_light_intensity: Some(fill as f64),
                    rim_light_intensity: Some(rim as f64),
                };
                let _ = update_ui_settings(data).await;
            });
        }
    });

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
    let _viewport_visibility = use_context::<ViewportVisibility>().expect("viewport_visibility context");

    // Lighting settings - extract from struct
    let _lighting_settings = use_context::<LightingSettings>().expect("lighting_settings context");

    // Camera controls - extract from struct
    let _camera_controls = use_context::<CameraControls>().expect("camera_controls context");

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
        <div class="w-48 bg-gray-800 border-r border-gray-700 flex flex-col">
            <div class="h-12 border-b border-gray-700 flex items-center px-3">
                <h2 class="text-sm font-semibold text-gray-300">"Device Palette"</h2>
            </div>

            // Connection creation button
            <div class="p-2 border-b border-gray-700">
                <button
                    class="w-full p-2 rounded border transition flex items-center gap-2 text-left"
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
                    <span class="text-lg">"🔗"</span>
                    <div class="flex-1">
                        <div class="text-xs font-medium">"Connect Nodes"</div>
                        <div class="text-[10px] text-gray-400">
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

            <div class="flex-1 overflow-y-auto p-2 space-y-2">
                {device_types.into_iter().map(|(display_name, icon, type_id, name_prefix)| {
                    let type_id_clone = type_id.to_string();
                    let name_prefix_clone = name_prefix.to_string();

                    view! {
                        <button
                            class="w-full p-2 bg-gray-700 hover:bg-gray-600 rounded border border-gray-600 hover:border-blue-500 transition flex items-center gap-2 text-left disabled:opacity-50 disabled:cursor-not-allowed"
                            on:click=move |_| {
                                create_node_action.dispatch((type_id_clone.clone(), name_prefix_clone.clone()));
                            }
                            disabled=move || create_node_action.pending().get()
                        >
                            <span class="text-lg">{icon}</span>
                            <div class="flex-1">
                                <div class="text-xs font-medium">{display_name}</div>
                                <div class="text-[10px] text-gray-400">
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
            <div class="px-2 py-2 border-t border-gray-700">
                {move || {
                    create_node_action.value().get().map(|result| {
                        match result {
                            Ok(node) => view! {
                                <div class="text-[10px] text-green-400">
                                    {format!("✓ Added: {}", node.name)}
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <div class="text-[10px] text-red-400">
                                    {format!("Error: {}", e)}
                                </div>
                            }.into_any(),
                        }
                    })
                }}
            </div>
        </div>
    }
}

/// Right properties panel
#[component]
fn PropertiesPanel(
    selected_item: RwSignal<Option<SelectedItem>>,
) -> impl IntoView {
    // Collapsible section states
    let view_controls_open = RwSignal::new(false);
    let lighting_controls_open = RwSignal::new(false);

    // Get context for controls
    let viewport_visibility = use_context::<ViewportVisibility>().expect("viewport_visibility context");
    let show_grid = viewport_visibility.show_grid;
    let show_x_axis = viewport_visibility.show_x_axis;
    let show_y_axis = viewport_visibility.show_y_axis;
    let show_z_axis = viewport_visibility.show_z_axis;

    let lighting_settings = use_context::<LightingSettings>().expect("lighting_settings context");
    let ambient_intensity = lighting_settings.ambient_intensity;
    let key_light_intensity = lighting_settings.key_light_intensity;
    let fill_light_intensity = lighting_settings.fill_light_intensity;
    let rim_light_intensity = lighting_settings.rim_light_intensity;

    view! {
        <div class="w-60 bg-gray-800 border-l border-gray-700 flex flex-col">
            <div class="h-12 border-b border-gray-700 flex items-center px-3">
                <h2 class="text-sm font-semibold text-gray-300">"Properties"</h2>
            </div>

            <div class="flex-1 overflow-y-auto">
                // Main properties section (dominant)
                <div class="p-4 border-b border-gray-700">
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

                // Collapsible View Controls
                <div class="border-b border-gray-700">
                    <button
                        class="w-full px-3 py-2 text-left text-xs font-semibold text-gray-300 hover:bg-gray-750 flex items-center justify-between"
                        on:click=move |_| view_controls_open.update(|v| *v = !*v)
                    >
                        "View Controls"
                        <span class="text-gray-500">{move || if view_controls_open.get() { "▼" } else { "▶" }}</span>
                    </button>
                    {move || {
                        if view_controls_open.get() {
                            view! {
                                <div class="p-2 space-y-1.5">
                                    <button
                                        class="w-full px-2 py-1 rounded text-[10px] border transition text-left"
                                        class:bg-gray-600=move || show_grid.get()
                                        class:border-gray-500=move || show_grid.get()
                                        class:bg-gray-700=move || !show_grid.get()
                                        class:border-gray-600=move || !show_grid.get()
                                        on:click=move |_| show_grid.update(|v| *v = !*v)
                                    >
                                        "Grid"
                                    </button>
                                    <button
                                        class="w-full px-2 py-1 rounded text-[10px] border transition text-left text-red-300"
                                        class:bg-red-600=move || show_x_axis.get()
                                        class:border-red-500=move || show_x_axis.get()
                                        class:bg-gray-700=move || !show_x_axis.get()
                                        class:border-gray-600=move || !show_x_axis.get()
                                        on:click=move |_| show_x_axis.update(|v| *v = !*v)
                                    >
                                        "X Axis"
                                    </button>
                                    <button
                                        class="w-full px-2 py-1 rounded text-[10px] border transition text-left text-green-300"
                                        class:bg-green-600=move || show_y_axis.get()
                                        class:border-green-500=move || show_y_axis.get()
                                        class:bg-gray-700=move || !show_y_axis.get()
                                        class:border-gray-600=move || !show_y_axis.get()
                                        on:click=move |_| show_y_axis.update(|v| *v = !*v)
                                    >
                                        "Y Axis"
                                    </button>
                                    <button
                                        class="w-full px-2 py-1 rounded text-[10px] border transition text-left text-blue-300"
                                        class:bg-blue-600=move || show_z_axis.get()
                                        class:border-blue-500=move || show_z_axis.get()
                                        class:bg-gray-700=move || !show_z_axis.get()
                                        class:border-gray-600=move || !show_z_axis.get()
                                        on:click=move |_| show_z_axis.update(|v| *v = !*v)
                                    >
                                        "Z Axis"
                                    </button>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </div>

                // Collapsible Lighting Controls
                <div class="border-b border-gray-700">
                    <button
                        class="w-full px-3 py-2 text-left text-xs font-semibold text-gray-300 hover:bg-gray-750 flex items-center justify-between"
                        on:click=move |_| lighting_controls_open.update(|v| *v = !*v)
                    >
                        "Lighting Controls"
                        <span class="text-gray-500">{move || if lighting_controls_open.get() { "▼" } else { "▶" }}</span>
                    </button>
                    {move || {
                        if lighting_controls_open.get() {
                            view! {
                                <div class="p-2 space-y-2">
                                    <div>
                                        <div class="flex justify-between items-center mb-1">
                                            <label class="text-xs text-gray-400">"Ambient"</label>
                                            <span class="text-xs text-gray-500">{move || format!("{:.1}", ambient_intensity.get())}</span>
                                        </div>
                                        <input
                                            type="range"
                                            min="0.0"
                                            max="1.0"
                                            step="0.1"
                                            class="w-full h-1.5 bg-gray-700 rounded-lg appearance-none cursor-pointer slider"
                                            prop:value=move || ambient_intensity.get().to_string()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f32>() {
                                                    ambient_intensity.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                    <div>
                                        <div class="flex justify-between items-center mb-1">
                                            <label class="text-xs text-gray-400">"Key Light"</label>
                                            <span class="text-xs text-gray-500">{move || format!("{:.1}", key_light_intensity.get())}</span>
                                        </div>
                                        <input
                                            type="range"
                                            min="0.0"
                                            max="3.0"
                                            step="0.1"
                                            class="w-full h-1.5 bg-gray-700 rounded-lg appearance-none cursor-pointer slider"
                                            prop:value=move || key_light_intensity.get().to_string()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f32>() {
                                                    key_light_intensity.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                    <div>
                                        <div class="flex justify-between items-center mb-1">
                                            <label class="text-xs text-gray-400">"Fill Light"</label>
                                            <span class="text-xs text-gray-500">{move || format!("{:.1}", fill_light_intensity.get())}</span>
                                        </div>
                                        <input
                                            type="range"
                                            min="0.0"
                                            max="2.0"
                                            step="0.1"
                                            class="w-full h-1.5 bg-gray-700 rounded-lg appearance-none cursor-pointer slider"
                                            prop:value=move || fill_light_intensity.get().to_string()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f32>() {
                                                    fill_light_intensity.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                    <div>
                                        <div class="flex justify-between items-center mb-1">
                                            <label class="text-xs text-gray-400">"Rim Light"</label>
                                            <span class="text-xs text-gray-500">{move || format!("{:.1}", rim_light_intensity.get())}</span>
                                        </div>
                                        <input
                                            type="range"
                                            min="0.0"
                                            max="2.0"
                                            step="0.1"
                                            class="w-full h-1.5 bg-gray-700 rounded-lg appearance-none cursor-pointer slider"
                                            prop:value=move || rim_light_intensity.get().to_string()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f32>() {
                                                    rim_light_intensity.set(val);
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </div>
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

                                <div class="grid grid-cols-3 gap-1">
                                    <div>
                                        <label class="block text-[10px] font-medium text-red-400 mb-0.5">"Pos X"</label>
                                        <input
                                            type="number"
                                            class="w-full px-1.5 py-1 bg-gray-700 border border-gray-600 rounded text-xs focus:outline-none focus:border-blue-500"
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
                                        <label class="block text-[10px] font-medium text-green-400 mb-0.5">"Pos Y"</label>
                                        <input
                                            type="number"
                                            class="w-full px-1.5 py-1 bg-gray-700 border border-gray-600 rounded text-xs focus:outline-none focus:border-blue-500"
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
                                        <label class="block text-[10px] font-medium text-blue-400 mb-0.5">"Pos Z"</label>
                                        <input
                                            type="number"
                                            class="w-full px-1.5 py-1 bg-gray-700 border border-gray-600 rounded text-xs focus:outline-none focus:border-blue-500"
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

                                <div class="grid grid-cols-3 gap-1">
                                    <div>
                                        <label class="block text-[10px] font-medium text-red-400 mb-0.5">"Rot X"</label>
                                        <input
                                            type="number"
                                            class="w-full px-1.5 py-1 bg-gray-700 border border-gray-600 rounded text-xs focus:outline-none focus:border-blue-500"
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
                                        <label class="block text-[10px] font-medium text-green-400 mb-0.5">"Rot Y"</label>
                                        <input
                                            type="number"
                                            class="w-full px-1.5 py-1 bg-gray-700 border border-gray-600 rounded text-xs focus:outline-none focus:border-blue-500"
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
                                        <label class="block text-[10px] font-medium text-blue-400 mb-0.5">"Rot Z"</label>
                                        <input
                                            type="number"
                                            class="w-full px-1.5 py-1 bg-gray-700 border border-gray-600 rounded text-xs focus:outline-none focus:border-blue-500"
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

                                <div class="pt-4 border-t border-gray-700">
                                    // Save button group
                                    <div class="mb-4">
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

                                    // Delete button group
                                    <div>
                                        <button
                                            class="w-full px-4 py-2 bg-red-600 hover:bg-red-700 rounded text-sm font-medium transition disabled:opacity-50 disabled:cursor-not-allowed"
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

                                <div class="pt-4 border-t border-gray-700">
                                    // Save button group
                                    <div class="mb-4">
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

                                    // Delete button group
                                    <div>
                                        <button
                                            class="w-full px-4 py-2 bg-red-600 hover:bg-red-700 rounded text-sm font-medium transition disabled:opacity-50 disabled:cursor-not-allowed"
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
