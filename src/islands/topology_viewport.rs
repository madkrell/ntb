use leptos::prelude::*;
use leptos::html::Canvas;

// Import server function from api module (available on both client and server)
use crate::api::get_topology_full;

#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

#[cfg(feature = "hydrate")]
use std::cell::RefCell;
#[cfg(feature = "hydrate")]
use std::rc::Rc;

/// Camera state for orbit controls
#[cfg(feature = "hydrate")]
#[derive(Clone, Copy, Debug)]
struct CameraState {
    distance: f32,
    azimuth: f32,   // horizontal rotation (radians)
    elevation: f32, // vertical rotation (radians)
    pan_x: f32,     // horizontal pan offset
    pan_y: f32,     // vertical pan offset
}

#[cfg(feature = "hydrate")]
impl CameraState {
    /// Linearly interpolate between two camera states
    fn lerp(&self, other: &CameraState, t: f32) -> CameraState {
        CameraState {
            distance: self.distance + (other.distance - self.distance) * t,
            azimuth: self.azimuth + (other.azimuth - self.azimuth) * t,
            elevation: self.elevation + (other.elevation - self.elevation) * t,
            pan_x: self.pan_x + (other.pan_x - self.pan_x) * t,
            pan_y: self.pan_y + (other.pan_y - self.pan_y) * t,
        }
    }
}

#[cfg(feature = "hydrate")]
impl Default for CameraState {
    fn default() -> Self {
        Self {
            distance: 18.0,        // Zoomed out to show full topology
            pan_x: 0.0,           // No horizontal pan
            pan_y: 0.0,           // No vertical pan
            azimuth: -0.785,       // ~-45 degrees (Blender default: green Y axis lower-left to upper-right)
            elevation: 1.047,      // ~60 degrees (looking down from above, Blender-style)
        }
    }
}

/// Get camera state for a given preset
#[cfg(feature = "hydrate")]
fn get_camera_preset(preset: crate::islands::topology_editor::CameraPreset) -> CameraState {
    use std::f32::consts::PI;
    match preset {
        crate::islands::topology_editor::CameraPreset::Top => CameraState {
            distance: 18.0,
            azimuth: 0.0,
            elevation: 0.01, // Near zero elevation = looking down Z axis (top view in Z-up system)
            pan_x: 0.0,
            pan_y: 0.0,
        },
        crate::islands::topology_editor::CameraPreset::Front => CameraState {
            distance: 18.0,
            azimuth: 0.0,
            elevation: PI / 2.0 - 0.01, // Near 90° elevation = looking along Y axis (front view)
            pan_x: 0.0,
            pan_y: 0.0,
        },
        crate::islands::topology_editor::CameraPreset::Side => CameraState {
            distance: 18.0,
            azimuth: PI / 2.0, // 90 degrees horizontal rotation
            elevation: 0.01,   // Same low elevation as top view = horizon level, looking from X axis
            pan_x: 0.0,
            pan_y: 0.0,
        },
        crate::islands::topology_editor::CameraPreset::Isometric => CameraState {
            distance: 18.0,
            azimuth: PI / 4.0,  // 45 degrees
            elevation: 0.615,   // ~35.26 degrees (classic isometric angle)
            pan_x: 0.0,
            pan_y: 0.0,
        },
        crate::islands::topology_editor::CameraPreset::Reset => CameraState::default(),
        crate::islands::topology_editor::CameraPreset::ZoomToFit => {
            // This will be handled specially in the Effect that triggers zoom to fit
            // Return default for now - actual calculation happens with access to node data
            CameraState::default()
        },
    }
}

/// Calculate bounding box of all nodes and return camera state to fit them with margin
#[cfg(feature = "hydrate")]
#[allow(unused_imports)]
fn calculate_zoom_to_fit(nodes_data: &[NodeData]) -> CameraState {
    use three_d::*;

    if nodes_data.is_empty() {
        return CameraState::default();
    }

    // Find bounding box of all nodes
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    let mut min_z = f32::MAX;
    let mut max_z = f32::MIN;

    for node in nodes_data.iter() {
        min_x = min_x.min(node.position.x);
        max_x = max_x.max(node.position.x);
        min_y = min_y.min(node.position.y);
        max_y = max_y.max(node.position.y);
        min_z = min_z.min(node.position.z);
        max_z = max_z.max(node.position.z);
    }

    // Calculate bounding box dimensions
    let width = max_x - min_x;
    let height = max_y - min_y;
    let depth = max_z - min_z;

    // Calculate center of bounding box (this will be our pan target)
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;
    let _center_z = (min_z + max_z) / 2.0; // We look at Z=0 for consistency

    // Calculate the maximum dimension to determine camera distance
    // Add 10% margin around the topology
    let margin_factor = 1.1;
    let max_dimension = (width.max(height).max(depth)) * margin_factor;

    // Calculate camera distance needed to fit the bounding box
    // Using 45° FOV (field of view), we need: distance = (size / 2) / tan(FOV / 2)
    let fov_radians = 45.0_f32.to_radians();
    let distance = (max_dimension / 2.0) / (fov_radians / 2.0).tan();

    // Ensure minimum distance
    let distance = distance.max(5.0);

    CameraState {
        distance,
        azimuth: -0.785,
        elevation: 1.047,
        pan_x: center_x,
        pan_y: center_y,
    }
}

/// Animate camera from start to target position using smooth interpolation
#[cfg(feature = "hydrate")]
fn animate_camera(
    camera_state: RwSignal<CameraState>,
    start: CameraState,
    target: CameraState,
    render_fn: Rc<RefCell<Option<Rc<dyn Fn(CameraState)>>>>,
) {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    let duration = 600.0; // Animation duration in milliseconds
    let start_time = js_sys::Date::now();

    // Create animation closure
    let animation_closure = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let animation_closure_clone = animation_closure.clone();

    let animate = Closure::wrap(Box::new(move || {
        let elapsed = js_sys::Date::now() - start_time;
        let progress = (elapsed / duration).min(1.0) as f32;

        // Ease-in-out function for smooth animation
        let t = if progress < 0.5 {
            2.0 * progress * progress
        } else {
            -1.0 + (4.0 - 2.0 * progress) * progress
        };

        // Interpolate camera state
        let current = start.lerp(&target, t);
        camera_state.set(current);

        // Render with new camera state
        if let Some(render) = render_fn.borrow().as_ref() {
            render(current);
        }

        // Continue animation if not finished
        if progress < 1.0 {
            let window = web_sys::window().expect("no global window");
            window
                .request_animation_frame(
                    animation_closure_clone
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                )
                .expect("should register animation frame");
        }
    }) as Box<dyn FnMut()>);

    // Store closure for self-reference
    *animation_closure.borrow_mut() = Some(animate);

    // Start animation
    let window = web_sys::window().expect("no global window");
    window
        .request_animation_frame(
            animation_closure
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )
        .expect("should register animation frame");
}

/// 3D Network Topology Viewport using three-d rendering
#[component]
pub fn TopologyViewport(
    /// Optional topology ID to load and display
    #[prop(optional)]
    topology_id: Option<i64>,
) -> impl IntoView {
    #[cfg(feature = "hydrate")]

    // Get shared state from context (provided by TopologyEditor)
    let selected_node_id = use_context::<RwSignal<Option<i64>>>().expect("selected_node_id context");
    #[allow(unused_variables)]
    let selected_item = use_context::<RwSignal<Option<crate::islands::topology_editor::SelectedItem>>>().expect("selected_item context");

    // Get connection mode from context (optional - may not exist)
    #[allow(unused_variables)]
    let connection_mode = use_context::<RwSignal<crate::islands::topology_editor::ConnectionMode>>();

    // Get grid/axes visibility controls from context (optional - may not exist)
    let viewport_visibility = use_context::<crate::islands::topology_editor::ViewportVisibility>();
    #[allow(unused_variables)]
    let (show_grid, show_x_axis, show_y_axis, show_z_axis, background_color, use_environment_lighting, environment_map) = match viewport_visibility {
        Some(vis) => (vis.show_grid, vis.show_x_axis, vis.show_y_axis, vis.show_z_axis, vis.background_color, vis.use_environment_lighting, vis.environment_map),
        None => (RwSignal::new(true), RwSignal::new(true), RwSignal::new(true), RwSignal::new(true), RwSignal::new(Some((0, 0, 0))), RwSignal::new(false), RwSignal::new("studio_small_09_2k.hdr".to_string())),
    };

    // Get lighting settings from context (optional - may not exist)
    let lighting_settings = use_context::<crate::islands::topology_editor::LightingSettings>();
    #[allow(unused_variables)]
    let (ambient_intensity, key_light_intensity, fill_light_intensity, rim_light_intensity) = match lighting_settings {
        Some(settings) => (settings.ambient_intensity, settings.key_light_intensity, settings.fill_light_intensity, settings.rim_light_intensity),
        None => (RwSignal::new(0.3), RwSignal::new(1.8), RwSignal::new(0.6), RwSignal::new(0.4)),
    };

    // Get camera controls from context (optional - may not exist)
    let camera_controls = use_context::<crate::islands::topology_editor::CameraControls>();
    let preset_trigger = camera_controls.map(|c| c.preset_trigger);

    let canvas_ref = NodeRef::<Canvas>::new();
    let error_signal = RwSignal::new(None::<String>);
    let is_initialized = RwSignal::new(false);

    // Tooltip state: (node_name, node_type, x, y)
    let tooltip_data = RwSignal::new(None::<(String, String, f64, f64)>);

    // Create signal for topology_id (needed for connection creation)
    #[allow(unused_variables)]
    let current_topology_id = RwSignal::new(topology_id.unwrap_or(1));

    // Camera state as signals for reactivity (client-side only)
    #[cfg(feature = "hydrate")]
    let camera_state = RwSignal::new(CameraState::default());

    // Store render function so we can trigger re-renders when selection changes
    #[cfg(feature = "hydrate")]
    let render_fn: Rc<RefCell<Option<Rc<dyn Fn(CameraState)>>>> = Rc::new(RefCell::new(None));

    // Store nodes/connections data so event handlers can access updated data on refetch
    #[cfg(feature = "hydrate")]
    let nodes_data_storage: Rc<RefCell<Vec<NodeData>>> = Rc::new(RefCell::new(Vec::new()));
    #[cfg(feature = "hydrate")]
    let connections_data_storage: Rc<RefCell<Vec<ConnectionData>>> = Rc::new(RefCell::new(Vec::new()));

    // Get refetch trigger from context (optional - may not exist if not in editor)
    let refetch_trigger = use_context::<RwSignal<u32>>();

    // Fetch topology data if topology_id is provided
    // Also refetch when refetch_trigger changes
    #[allow(unused_variables)]
    let topology_data = Resource::new(
        move || (topology_id, refetch_trigger.map(|t| t.get())),
        |(id, _trigger)| async move {
            match id {
                Some(topology_id) => {
                    // Call the auto-generated server function
                    match get_topology_full(topology_id).await {
                        Ok(data) => Some(data),
                        Err(e) => {
                            tracing::error!("Failed to fetch topology: {:?}", e);
                            None
                        }
                    }
                }
                None => None,
            }
        },
    );

    // Clone render_fn and data storages for the Effect closure below
    #[cfg(feature = "hydrate")]
    let render_fn_for_effect = render_fn.clone();
    #[cfg(feature = "hydrate")]
    let nodes_data_for_effect = nodes_data_storage.clone();
    #[cfg(feature = "hydrate")]
    let connections_data_for_effect = connections_data_storage.clone();

    // Initialize three-d viewport when canvas mounts or data loads
    Effect::new(move || {
        #[allow(unused_variables)]
        if let Some(canvas_element) = canvas_ref.get() {
            #[cfg(feature = "hydrate")]
            {
                // Access topology_data to make Effect reactive to it
                let data_option = topology_data.get();

                // Read signal values BEFORE entering conditional branches (needed in both branches)
                // Use .get() instead of .get_untracked() to make Effect reactive to these changes
                let background_color_val = background_color.get();

                // Wait for topology data to load
                if let Some(Some(topo_data)) = data_option {
                    // Check if this is a refetch (already initialized)
                    let already_initialized = is_initialized.get_untracked();

                    // Spawn async initialization (needed for glTF loading)
                    let canvas = canvas_element.clone();
                    let topo = topo_data.clone();
                    let render_fn = render_fn_for_effect.clone();
                    let nodes_storage = nodes_data_for_effect.clone();
                    let connections_storage = connections_data_for_effect.clone();

                    // Read signal values BEFORE entering async context
                    // Use .get() instead of .get_untracked() to make Effect reactive to these changes
                    let show_grid_val = show_grid.get();
                    let show_x_val = show_x_axis.get();
                    let show_y_val = show_y_axis.get();
                    let show_z_val = show_z_axis.get();
                    let use_env_lighting_val = use_environment_lighting.get();
                    let env_map_val = environment_map.get();
                    let ambient_val = ambient_intensity.get();
                    let key_light_val = key_light_intensity.get();
                    let fill_light_val = fill_light_intensity.get();
                    let rim_light_val = rim_light_intensity.get();

                    wasm_bindgen_futures::spawn_local(async move {
                        match initialize_threed_viewport(
                            &canvas,
                            camera_state,
                            &topo,
                            selected_node_id,
                            selected_item,
                            render_fn,
                            nodes_storage,
                            connections_storage,
                            tooltip_data,
                            show_grid_val,
                            show_x_val,
                            show_y_val,
                            show_z_val,
                            background_color_val,
                            use_env_lighting_val,
                            env_map_val,
                            ambient_val,
                            key_light_val,
                            fill_light_val,
                            rim_light_val,
                            connection_mode,
                            refetch_trigger,
                            Some(current_topology_id),
                            already_initialized, // Skip event handlers on refetch
                            Some(use_environment_lighting), // Pass signal for dynamic HDR toggle
                        ).await {
                            Ok(_) => {
                                is_initialized.set(true);
                            }
                            Err(e) => {
                                web_sys::console::error_1(&format!("Initialization failed: {}", e).into());
                                error_signal.set(Some(e.clone()));
                            }
                        }
                    });
                } else if topology_id.is_none() {
                    // Initialize with test scene if no topology_id
                    match initialize_threed_viewport_test(&canvas_element, camera_state, selected_node_id, selected_item, render_fn_for_effect.clone(), tooltip_data, background_color_val, connection_mode, refetch_trigger, Some(current_topology_id)) {
                        Ok(_) => {
                            is_initialized.set(true);
                        }
                        Err(e) => {
                            error_signal.set(Some(e.clone()));
                            web_sys::console::error_1(&format!("Failed to initialize 3D viewport: {}", e).into());
                        }
                    }
                }
            }
        }
    });

    // Component-level Effect to re-render when node selection changes
    #[cfg(feature = "hydrate")]
    {
        let render_fn = render_fn.clone();
        let _effect = Effect::new(move || {
            // Track the selection signal
            let _selected = selected_node_id.get();

            // Call the stored render function if available
            if let Some(render) = render_fn.borrow().as_ref() {
                render(camera_state.get_untracked());
            }
        });
    }

    // Component-level Effect to re-render when item selection changes (connections)
    #[cfg(feature = "hydrate")]
    {
        let render_fn = render_fn.clone();
        let _effect = Effect::new(move || {
            // Track the selected item signal
            let _selected = selected_item.get();

            // Call the stored render function if available
            if let Some(render) = render_fn.borrow().as_ref() {
                render(camera_state.get_untracked());
            }
        });
    }

    // Component-level Effect to trigger re-initialization when visibility changes
    #[cfg(feature = "hydrate")]
    {
        if let Some(refetch_trigger) = refetch_trigger {
            // Track if this is the first run to avoid triggering on initial mount
            let is_first_run = RwSignal::new(true);

            let _effect = Effect::new(move || {
                // Track visibility signals
                let _grid = show_grid.get();
                let _x = show_x_axis.get();
                let _y = show_y_axis.get();
                let _z = show_z_axis.get();
                let _bg = background_color.get();

                // Skip the first run (initial mount)
                if is_first_run.get_untracked() {
                    is_first_run.set(false);
                    return;
                }

                // Trigger viewport re-initialization
                refetch_trigger.update(|v| *v += 1);
            });
        }
    }

    // Component-level Effect to trigger re-initialization when lighting changes
    #[cfg(feature = "hydrate")]
    {
        if let Some(refetch_trigger) = refetch_trigger {
            // Track if this is the first run to avoid triggering on initial mount
            let is_first_run = RwSignal::new(true);

            let _effect = Effect::new(move || {
                // Track lighting intensity signals
                let _ambient = ambient_intensity.get();
                let _key = key_light_intensity.get();
                let _fill = fill_light_intensity.get();
                let _rim = rim_light_intensity.get();

                // Skip the first run (initial mount)
                if is_first_run.get_untracked() {
                    is_first_run.set(false);
                    return;
                }

                // Trigger viewport re-initialization
                refetch_trigger.update(|v| *v += 1);
            });
        }
    }

    // Component-level Effect to handle camera preset triggers
    #[cfg(feature = "hydrate")]
    {
        if let Some(preset_signal) = preset_trigger {
            let render_fn = render_fn.clone();
            let nodes_storage = nodes_data_storage.clone();

            let _effect = Effect::new(move || {
                if let Some(preset) = preset_signal.get() {
                    // Clear trigger
                    preset_signal.set(None);

                    // Get target camera state - handle ZoomToFit specially
                    let target_state = if preset == crate::islands::topology_editor::CameraPreset::ZoomToFit {
                        // Calculate zoom to fit based on current node data
                        let nodes = nodes_storage.borrow();
                        calculate_zoom_to_fit(&nodes)
                    } else {
                        // Use standard preset
                        get_camera_preset(preset)
                    };

                    let start_state = camera_state.get_untracked();

                    // Animate camera to target position
                    animate_camera(camera_state, start_state, target_state, render_fn.clone());
                }
            });
        }
    }

    view! {
        <div class="topology-viewport-container w-full h-full flex flex-col relative">
            <canvas
                node_ref=canvas_ref
                class="w-full h-full"
                style="background-color: #1a1a1a; cursor: pointer; display: block;"
            />

            {move || {
                if let Some(err) = error_signal.get() {
                    view! {
                        <div class="absolute bottom-4 left-4 bg-red-900 text-red-200 px-3 py-2 rounded text-sm">
                            "Error: " {err}
                        </div>
                    }.into_any()
                } else if is_initialized.get() {
                    view! {
                        <div class="absolute bottom-4 left-4 bg-gray-800 bg-opacity-80 text-gray-300 px-3 py-2 rounded text-xs">
                            "🎮 Drag to rotate • Scroll to zoom"
                            {topology_id.map(|id| format!(" • Topology #{}", id)).unwrap_or_default()}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="absolute bottom-4 left-4 bg-gray-800 bg-opacity-80 text-gray-400 px-3 py-2 rounded text-xs">
                            "⏳ Initializing 3D viewport..."
                        </div>
                    }.into_any()
                }
            }}

            // Tooltip display - shows node name and type on hover
            {move || {
                tooltip_data.get().map(|(name, node_type, x, y)| {
                    view! {
                        <div
                            class="absolute bg-gray-900 text-white px-3 py-2 rounded shadow-lg text-sm pointer-events-none"
                            style:left=format!("{}px", x + 10.0)
                            style:top=format!("{}px", y + 10.0)
                            style="z-index: 1000;"
                        >
                            <div class="font-semibold">{name}</div>
                            <div class="text-gray-400 text-xs">{node_type}</div>
                        </div>
                    }
                })
            }}

            // Camera controls overlay - top right corner (compact 2x2 grid)
            {move || {
                if let Some(trigger) = preset_trigger {
                    view! {
                        <div class="absolute top-3 right-3 flex flex-col gap-1" style="z-index: 100;">
                            // 2x2 grid for preset views
                            <div class="grid grid-cols-2 gap-1">
                                <button
                                    class="w-6 h-6 bg-gray-800 bg-opacity-90 hover:bg-opacity-100 border border-gray-600 hover:border-blue-500 rounded flex items-center justify-center text-[10px] font-bold text-gray-300 hover:text-blue-400 transition shadow-lg"
                                    on:click=move |_| trigger.set(Some(crate::islands::topology_editor::CameraPreset::Top))
                                    title="Top View"
                                >
                                    "T"
                                </button>
                                <button
                                    class="w-6 h-6 bg-gray-800 bg-opacity-90 hover:bg-opacity-100 border border-gray-600 hover:border-blue-500 rounded flex items-center justify-center text-[10px] font-bold text-gray-300 hover:text-blue-400 transition shadow-lg"
                                    on:click=move |_| trigger.set(Some(crate::islands::topology_editor::CameraPreset::Front))
                                    title="Front View"
                                >
                                    "F"
                                </button>
                                <button
                                    class="w-6 h-6 bg-gray-800 bg-opacity-90 hover:bg-opacity-100 border border-gray-600 hover:border-blue-500 rounded flex items-center justify-center text-[10px] font-bold text-gray-300 hover:text-blue-400 transition shadow-lg"
                                    on:click=move |_| trigger.set(Some(crate::islands::topology_editor::CameraPreset::Side))
                                    title="Side View"
                                >
                                    "S"
                                </button>
                                <button
                                    class="w-6 h-6 bg-gray-800 bg-opacity-90 hover:bg-opacity-100 border border-gray-600 hover:border-blue-500 rounded flex items-center justify-center text-[10px] font-bold text-gray-300 hover:text-blue-400 transition shadow-lg"
                                    on:click=move |_| trigger.set(Some(crate::islands::topology_editor::CameraPreset::Isometric))
                                    title="Isometric View"
                                >
                                    "I"
                                </button>
                            </div>
                            // Reset button - matches width of 2 buttons
                            <button
                                class="w-full h-6 bg-blue-700 bg-opacity-90 hover:bg-opacity-100 border border-blue-600 hover:border-blue-500 rounded flex items-center justify-center text-[10px] font-bold text-blue-100 hover:text-white transition shadow-lg"
                                on:click=move |_| trigger.set(Some(crate::islands::topology_editor::CameraPreset::Reset))
                                title="Reset View"
                            >
                                "↺"
                            </button>
                            // Zoom to Fit button - matches width of 2 buttons
                            <button
                                class="w-full h-6 bg-green-700 bg-opacity-90 hover:bg-opacity-100 border border-green-600 hover:border-green-500 rounded flex items-center justify-center text-[10px] font-bold text-green-100 hover:text-white transition shadow-lg"
                                on:click=move |_| trigger.set(Some(crate::islands::topology_editor::CameraPreset::ZoomToFit))
                                title="Zoom to Fit All Nodes"
                            >
                                "⊡"
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}

// glTF Color Space Conversion Functions
// glTF 2.0 stores baseColorFactor in linear RGB space, but displays need sRGB
// The exact sRGB transfer function uses gamma 2.4 with piecewise definition
#[cfg(feature = "hydrate")]
fn linear_to_srgb(linear: f32) -> f32 {
    if linear <= 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}

#[cfg(feature = "hydrate")]
fn convert_linear_color_to_srgba(linear: &three_d_asset::Srgba) -> three_d_asset::Srgba {
    use three_d_asset::Srgba;

    // Extract linear values (0-255 range, but representing linear RGB)
    let linear_r = linear.r as f32 / 255.0;
    let linear_g = linear.g as f32 / 255.0;
    let linear_b = linear.b as f32 / 255.0;

    // Convert to sRGB for display
    Srgba::new(
        (linear_to_srgb(linear_r) * 255.0).clamp(0.0, 255.0) as u8,
        (linear_to_srgb(linear_g) * 255.0).clamp(0.0, 255.0) as u8,
        (linear_to_srgb(linear_b) * 255.0).clamp(0.0, 255.0) as u8,
        linear.a, // Alpha is linear, never gamma-corrected
    )
}

// Node data for selection and tooltip
#[cfg(feature = "hydrate")]
struct NodeData {
    id: i64,
    name: String,
    node_type: String,
    position: three_d::Vec3,
    radius: f32,
}

// Connection data for selection
#[cfg(feature = "hydrate")]
struct ConnectionData {
    id: i64,
    source_pos: three_d::Vec3,
    target_pos: three_d::Vec3,
    radius: f32, // Cylinder radius for hit detection
}

// Ray-cylinder intersection test
// Returns Some(distance) if ray intersects the cylinder, None otherwise
#[cfg(feature = "hydrate")]
fn ray_cylinder_intersection(
    ray_origin: three_d::Vec3,
    ray_dir: three_d::Vec3,
    cylinder_start: three_d::Vec3,
    cylinder_end: three_d::Vec3,
    cylinder_radius: f32,
) -> Option<f32> {
    use three_d::*;

    // Vector along the cylinder axis
    let axis = cylinder_end - cylinder_start;
    let axis_length = axis.magnitude();
    let axis_normalized = axis.normalize();

    // Vector from cylinder start to ray origin
    let oc = ray_origin - cylinder_start;

    // Project ray onto the plane perpendicular to cylinder axis
    // We're solving for the closest point on the ray to the cylinder axis
    let dot_axis_ray = axis_normalized.dot(ray_dir);
    let dot_axis_oc = axis_normalized.dot(oc);

    // Ray perpendicular components (components perpendicular to cylinder axis)
    let ray_perp = ray_dir - axis_normalized * dot_axis_ray;
    let oc_perp = oc - axis_normalized * dot_axis_oc;

    // Solve quadratic equation for ray-cylinder intersection
    let a = ray_perp.dot(ray_perp);
    let b = 2.0 * ray_perp.dot(oc_perp);
    let c = oc_perp.dot(oc_perp) - cylinder_radius * cylinder_radius;

    let discriminant = b * b - 4.0 * a * c;

    // No intersection if discriminant is negative
    if discriminant < 0.0 || a.abs() < 0.0001 {
        return None;
    }

    // Calculate the two intersection points along the ray
    let sqrt_disc = discriminant.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);

    // Check both intersection points
    for t in [t1, t2] {
        if t > 0.0 {
            // Calculate the point on the ray
            let point = ray_origin + ray_dir * t;

            // Project point onto cylinder axis to check if it's within the cylinder length
            let point_on_axis = (point - cylinder_start).dot(axis_normalized);

            // Check if the intersection is within the cylinder bounds
            if point_on_axis >= 0.0 && point_on_axis <= axis_length {
                return Some(t);
            }
        }
    }

    None
}

// Type alias for node mesh (sphere with material)
#[cfg(feature = "hydrate")]
type NodeMesh = three_d::Gm<three_d::Mesh, three_d::PhysicalMaterial>;

/// Initialize three-d Context with topology data
#[cfg(feature = "hydrate")]
async fn initialize_threed_viewport(
    canvas: &web_sys::HtmlCanvasElement,
    camera_state: RwSignal<CameraState>,
    topology_data: &crate::models::TopologyFull,
    selected_node_id_signal: RwSignal<Option<i64>>,
    selected_item_signal: RwSignal<Option<crate::islands::topology_editor::SelectedItem>>,
    render_fn_storage: Rc<RefCell<Option<Rc<dyn Fn(CameraState)>>>>,
    nodes_data_storage: Rc<RefCell<Vec<NodeData>>>,
    connections_data_storage: Rc<RefCell<Vec<ConnectionData>>>,
    tooltip_data: RwSignal<Option<(String, String, f64, f64)>>,
    show_grid: bool,
    show_x_axis: bool,
    show_y_axis: bool,
    show_z_axis: bool,
    background_color: Option<(u8, u8, u8)>,
    use_environment_lighting: bool,
    environment_map: String,
    ambient_intensity: f32,
    key_light_intensity: f32,
    fill_light_intensity: f32,
    rim_light_intensity: f32,
    connection_mode: Option<RwSignal<crate::islands::topology_editor::ConnectionMode>>,
    refetch_trigger: Option<RwSignal<u32>>,
    current_topology_id: Option<RwSignal<i64>>,
    skip_event_handlers: bool, // Set to true on refetches to avoid duplicate handlers
    use_environment_lighting_signal: Option<RwSignal<bool>>, // ADDED: Signal for dynamic HDR toggle
) -> Result<(), String> {
    use web_sys::WebGl2RenderingContext as GL;
    use three_d::*;
    use std::collections::HashMap;
    // Create WebGL2 context attributes with preserveDrawingBuffer enabled
    // This is required for canvas.toDataURL() to work properly
    use wasm_bindgen::JsValue;
    use js_sys::Object;
    use js_sys::Reflect;

    let context_options = Object::new();
    Reflect::set(
        &context_options,
        &JsValue::from_str("preserveDrawingBuffer"),
        &JsValue::from_bool(true),
    ).map_err(|e| format!("Failed to set preserveDrawingBuffer: {:?}", e))?;

    // Get WebGL2 context from canvas with options
    let webgl2_context = canvas
        .get_context_with_context_options("webgl2", &context_options)
        .map_err(|e| format!("Failed to get WebGL2 context: {:?}", e))?
        .ok_or("WebGL2 context is None")?
        .dyn_into::<GL>()
        .map_err(|e| format!("Failed to cast to WebGL2 context: {:?}", e))?;

    // Wrap in glow::Context
    let gl = three_d::context::Context::from_webgl2_context(webgl2_context);
    let context = Context::from_gl_context(std::sync::Arc::new(gl))
        .map_err(|e| format!("Failed to create three-d Context: {:?}", e))?;

    // Load HDR environment map (if environment lighting is enabled)
    let skybox_option: Option<Skybox> = if use_environment_lighting {
        use three_d_asset::io::load_async;

        // Build full URL from window.location
        let window = web_sys::window().expect("no global window");
        let location = window.location();
        let origin = location.origin().expect("no origin");

        let hdr_url = format!("{}/environments/{}", origin, environment_map);
        web_sys::console::log_1(&format!("Loading HDR environment: {}", hdr_url).into());

        match load_async(&[hdr_url.as_str()]).await {
            Ok(mut loaded) => {
                match loaded.deserialize::<three_d_asset::Texture2D>(&environment_map) {
                    Ok(hdr_texture) => {
                        // Create skybox from equirectangular HDR
                        let skybox = Skybox::new_from_equirectangular(&context, &hdr_texture);
                        web_sys::console::log_1(&format!("✓ HDR environment loaded: {}", environment_map).into());
                        Some(skybox)
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("✗ Failed to deserialize HDR {}: {:?}", environment_map, e).into());
                        None
                    }
                }
            }
            Err(e) => {
                web_sys::console::error_1(&format!("✗ Failed to load HDR from {}: {:?}", hdr_url, e).into());
                None
            }
        }
    } else {
        None
    };

    // Load GLB models for each unique vendor/model combination (async loading)
    // Build cache key from vendor + model_name + node_type
    let mut node_models: HashMap<String, Option<three_d_asset::Model>> = HashMap::new();

    {
        use three_d_asset::io::load_async;

        // Build full URL from window.location
        let window = web_sys::window().expect("no global window");
        let location = window.location();
        let origin = location.origin().expect("no origin");

        // Collect unique model paths from all nodes
        let mut unique_models: std::collections::HashSet<(String, String, String, String)> = std::collections::HashSet::new();
        for node in &topology_data.nodes {
            unique_models.insert((
                node.node_type.clone(),
                node.vendor.clone(),
                node.model_name.clone(),
                format!("{}/{}/{}", node.node_type, node.vendor, node.model_name)
            ));
        }

        // Load each unique model
        for (node_type, vendor, model_name, path) in unique_models {
            let model_url = format!("{}/models/{}.glb", origin, path);
            let cache_key = format!("{}:{}:{}", node_type, vendor, model_name);

            match load_async(&[model_url.as_str()]).await {
                Ok(mut loaded) => {
                    // Deserialize the GLB file to CPU model
                    match loaded.deserialize::<three_d_asset::Model>(&model_name) {
                        Ok(cpu_model) => {
                            node_models.insert(cache_key.clone(), Some(cpu_model));
                            web_sys::console::log_1(&format!("✓ Loaded model: {}", cache_key).into());
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("✗ Failed to deserialize {}: {:?}", cache_key, e).into());
                            node_models.insert(cache_key, None);
                        }
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("✗ Failed to load GLB from {}: {:?}", model_url, e).into());
                    node_models.insert(cache_key, None);
                }
            }
        }
    }

    // Create node meshes (3D models or spheres at x/y/z positions) and store node data
    let mut node_meshes: Vec<(i64, NodeMesh, NodeMesh)> = Vec::new();
    let mut node_positions = HashMap::new();
    let mut nodes_data = Vec::new();
    let node_radius = 0.3;

    // Create CPU mesh once for reuse
    let sphere_cpu_mesh = CpuMesh::sphere(16);

    // Selected material (yellow/orange) - same for all nodes
    // Use slightly metallic, medium roughness for visual interest
    let selected_material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Srgba::new(255, 200, 50, 255), // Yellow/orange for selected nodes
            metallic: 0.3,
            roughness: 0.4,
            ..Default::default()
        },
    );

    for node in &topology_data.nodes {
        // Map database coordinates to Blender convention (XY floor, Z up)
        // DB: position_y was "up" → now render as Z (up in Blender)
        // DB: position_z was "depth" → now render as Y (front-back)
        let position = vec3(
            node.position_x as f32,  // X stays X (left-right)
            node.position_z as f32,  // Z becomes Y (front-back)
            node.position_y as f32,  // Y becomes Z (up-down)
        );
        node_positions.insert(node.id, position);

        // Store node data for selection and tooltip (use larger radius for easier clicking)
        nodes_data.push(NodeData {
            id: node.id,
            name: node.name.clone(),
            node_type: node.node_type.clone(),
            position,
            radius: node_radius * 2.0,  // 2x visual radius for easier clicking
        });

        // Check if we have a loaded 3D model for this vendor/model combination
        let model_cache_key = format!("{}:{}:{}", node.node_type, node.vendor, node.model_name);
        let has_model = node_models.get(&model_cache_key).and_then(|opt| opt.as_ref()).is_some();

        if has_model {
            // Render node with loaded 3D model
            let cpu_model = node_models.get(&model_cache_key).unwrap().as_ref().unwrap();

            // Process each primitive (sub-mesh) in the model
            for primitive in cpu_model.geometries.iter() {
                // Geometry is an enum: Triangles(TriMesh) or Points(PointCloud)
                // We only handle triangle meshes
                match &primitive.geometry {
                    three_d_asset::geometry::Geometry::Triangles(tri_mesh) => {
                        // tri_mesh is a TriMesh, which is the same as CpuMesh!
                        // Just pass it directly to Mesh::new()

                        // Parse node color from database (format: "R,G,B")
                        let node_color = {
                            let parts: Vec<&str> = node.color.split(',').collect();
                            if parts.len() == 3 {
                                if let (Ok(r), Ok(g), Ok(b)) = (
                                    parts[0].parse::<u8>(),
                                    parts[1].parse::<u8>(),
                                    parts[2].parse::<u8>(),
                                ) {
                                    Srgba::new(r, g, b, 255)
                                } else {
                                    get_node_color(&node.node_type) // Fallback to type-based color
                                }
                            } else {
                                get_node_color(&node.node_type) // Fallback to type-based color
                            }
                        };
                        let (metallic, roughness) = get_node_material_properties(&node.node_type);

                        // Material system: Use glTF materials with full texture support
                        let normal_material = if let Some(mat_idx) = primitive.material_index {
                            // Model has glTF material
                            if let Some(gltf_mat) = cpu_model.materials.get(mat_idx) {
                                // Check if material has textures
                                let has_textures = gltf_mat.albedo_texture.is_some()
                                    || gltf_mat.metallic_roughness_texture.is_some()
                                    || gltf_mat.normal_texture.is_some()
                                    || gltf_mat.occlusion_texture.is_some()
                                    || gltf_mat.emissive_texture.is_some();

                                if has_textures {
                                    // Material has textures - use PhysicalMaterial::new() for full glTF support
                                    // This handles: albedo textures, metallic/roughness textures, normal maps,
                                    // occlusion maps, emissive textures, and alpha transparency
                                    web_sys::console::log_1(&format!(
                                        "✓ Using FULL glTF material with textures for {} (albedo_tex: {}, metallic_roughness_tex: {}, normal_tex: {}, occlusion_tex: {}, emissive_tex: {})",
                                        node.model_name,
                                        gltf_mat.albedo_texture.is_some(),
                                        gltf_mat.metallic_roughness_texture.is_some(),
                                        gltf_mat.normal_texture.is_some(),
                                        gltf_mat.occlusion_texture.is_some(),
                                        gltf_mat.emissive_texture.is_some()
                                    ).into());
                                    PhysicalMaterial::new(&context, gltf_mat)
                                } else {
                                    // No textures - only base color factor (needs color space conversion)
                                    // Apply proper linear→sRGB conversion to fix three-d color space bug
                                    let corrected_albedo = convert_linear_color_to_srgba(&gltf_mat.albedo);

                                    web_sys::console::log_1(&format!(
                                        "✓ Using glTF material (color-only) with sRGB conversion for {} - Linear: {:?}, sRGB: {:?}",
                                        node.model_name,
                                        (gltf_mat.albedo.r, gltf_mat.albedo.g, gltf_mat.albedo.b),
                                        (corrected_albedo.r, corrected_albedo.g, corrected_albedo.b)
                                    ).into());

                                    PhysicalMaterial::new_opaque(
                                        &context,
                                        &CpuMaterial {
                                            albedo: corrected_albedo,
                                            metallic: gltf_mat.metallic,
                                            roughness: gltf_mat.roughness,
                                            ..Default::default()
                                        },
                                    )
                                }
                            } else {
                                // Fallback: material_index invalid
                                web_sys::console::log_1(&format!("⚠ Invalid material_index for {}, using database material", node.model_name).into());
                                PhysicalMaterial::new_opaque(
                                    &context,
                                    &CpuMaterial {
                                        albedo: node_color,
                                        metallic,
                                        roughness,
                                        ..Default::default()
                                    },
                                )
                            }
                        } else {
                            // No glTF material - use database + type properties
                            web_sys::console::log_1(&format!("✓ Using database material for {}", node.model_name).into());
                            PhysicalMaterial::new_opaque(
                                &context,
                                &CpuMaterial {
                                    albedo: node_color,
                                    metallic,
                                    roughness,
                                    ..Default::default()
                                },
                            )
                        };

                        // Create GPU meshes (tri_mesh is already &TriMesh/&CpuMesh)
                        let mut normal_mesh = Gm::new(
                            Mesh::new(&context, tri_mesh),
                            normal_material,
                        );

                        let mut selected_mesh = Gm::new(
                            Mesh::new(&context, tri_mesh),
                            selected_material.clone(),
                        );

                        // Apply node rotations from database (in degrees)
                        // Use degrees() to create Deg type, which auto-converts to radians
                        let x_rotation = Mat4::from_angle_x(degrees(node.rotation_x as f32));
                        let y_rotation = Mat4::from_angle_y(degrees(node.rotation_y as f32));
                        let z_rotation = Mat4::from_angle_z(degrees(node.rotation_z as f32));

                        let transform = Mat4::from_translation(position)
                            * Mat4::from_scale(node_radius * node.scale as f32)
                            * z_rotation
                            * y_rotation
                            * x_rotation
                            * primitive.transformation;

                        normal_mesh.set_transformation(transform);
                        selected_mesh.set_transformation(transform);

                        node_meshes.push((node.id, normal_mesh, selected_mesh));
                    }
                    three_d_asset::geometry::Geometry::Points(_) => {
                    }
                }
            }
        } else {
            // Render nodes without 3D models as colored spheres (fallback)
            // Parse node color from database (format: "R,G,B")
            let node_color = {
                let parts: Vec<&str> = node.color.split(',').collect();
                if parts.len() == 3 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        parts[0].parse::<u8>(),
                        parts[1].parse::<u8>(),
                        parts[2].parse::<u8>(),
                    ) {
                        Srgba::new(r, g, b, 255)
                    } else {
                        get_node_color(&node.node_type) // Fallback to type-based color
                    }
                } else {
                    get_node_color(&node.node_type) // Fallback to type-based color
                }
            };
            let (metallic, roughness) = get_node_material_properties(&node.node_type);

            // Create material for this node type with PBR properties
            let normal_material = PhysicalMaterial::new_opaque(
                &context,
                &CpuMaterial {
                    albedo: node_color,
                    metallic,
                    roughness,
                    ..Default::default()
                },
            );

            // Create normal sphere (new mesh for each node)
            let mut normal_sphere = Gm::new(
                Mesh::new(&context, &sphere_cpu_mesh),
                normal_material,
            );
            normal_sphere.set_transformation(
                Mat4::from_translation(position) * Mat4::from_scale(node_radius * node.scale as f32)
            );

            // Create selected sphere (same position, different material, new mesh)
            let mut selected_sphere = Gm::new(
                Mesh::new(&context, &sphere_cpu_mesh),
                selected_material.clone(),
            );
            selected_sphere.set_transformation(
                Mat4::from_translation(position) * Mat4::from_scale(node_radius * node.scale as f32)
            );

            // Store (node_id, normal_mesh, selected_mesh) tuple
            node_meshes.push((node.id, normal_sphere, selected_sphere));
        }
    }

    // Create grid and axes for spatial reference (based on visibility settings)
    let grid_axes_meshes = create_grid_and_axes(
        &context,
        show_grid,
        show_x_axis,
        show_y_axis,
        show_z_axis,
    );

    // Shared cylinder mesh for all connections (for efficiency)
    let cylinder_cpu_mesh = CpuMesh::cylinder(16);

    // Create connection meshes (lines between nodes) - both normal and selected versions
    let mut connection_meshes = Vec::new();
    let mut connections_data = Vec::new();

    for conn in &topology_data.connections {
        if let (Some(&start_pos), Some(&end_pos)) = (
            node_positions.get(&conn.source_node_id),
            node_positions.get(&conn.target_node_id),
        ) {
            // Store connection data for selection (use larger radius for easier clicking)
            connections_data.push(ConnectionData {
                id: conn.id,
                source_pos: start_pos,
                target_pos: end_pos,
                radius: 0.15, // Larger than visual radius for easier clicking
            });

            // Parse connection color from database (format: "R,G,B")
            let normal_color = {
                let parts: Vec<&str> = conn.color.split(',').collect();
                if parts.len() == 3 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        parts[0].parse::<u8>(),
                        parts[1].parse::<u8>(),
                        parts[2].parse::<u8>(),
                    ) {
                        Srgba::new(r, g, b, 255)
                    } else {
                        Srgba::new(128, 128, 128, 255) // Fallback gray
                    }
                } else {
                    Srgba::new(128, 128, 128, 255) // Fallback gray
                }
            };

            // Selected color - bright yellow/orange for visibility
            let selected_color = Srgba::new(255, 200, 0, 255);

            // Create normal and selected meshes
            let normal_mesh = create_line_cylinder(
                &context,
                start_pos,
                end_pos,
                0.012, // Very thin thickness for clean, delicate lines
                normal_color,
                &cylinder_cpu_mesh,
            );

            let selected_mesh = create_line_cylinder(
                &context,
                start_pos,
                end_pos,
                0.020, // Slightly thicker when selected for better visibility
                selected_color,
                &cylinder_cpu_mesh,
            );

            if let (Some(normal), Some(selected)) = (normal_mesh, selected_mesh) {
                connection_meshes.push((conn.id, normal, selected));
            }
        }
    }

    // Create professional lighting setup with user-controlled intensities
    // Ambient light - either environment-based or simple flat lighting
    let ambient = if use_environment_lighting && skybox_option.is_some() {
        // Use HDR environment lighting for realistic ambient illumination
        let skybox = skybox_option.as_ref().unwrap();
        Rc::new(AmbientLight::new_with_environment(
            &context,
            ambient_intensity,
            Srgba::WHITE,
            skybox.texture(),
        ))
    } else {
        // Fallback to simple ambient light
        Rc::new(AmbientLight::new(&context, ambient_intensity, Srgba::WHITE))
    };

    // Key light - Main directional light from above-front with warm tone
    // Direction: from upper-front-right toward origin
    let key_light = Rc::new(DirectionalLight::new(
        &context,
        key_light_intensity,
        Srgba::new(255, 248, 240, 255), // Warm white (slight yellow tint)
        vec3(-0.5, -0.3, -1.0),
    ));

    // Fill light - Softer light from the side with cool tone to reduce harsh shadows
    // Direction: from left side
    let fill_light = Rc::new(DirectionalLight::new(
        &context,
        fill_light_intensity,
        Srgba::new(200, 220, 255, 255), // Cool white (slight blue tint)
        vec3(1.0, 0.5, -0.3),
    ));

    // Rim/back light - Subtle light from behind to highlight edges and add depth
    // Direction: from behind and below
    let rim_light = Rc::new(DirectionalLight::new(
        &context,
        rim_light_intensity,
        Srgba::new(220, 230, 255, 255), // Subtle cool highlight
        vec3(0.3, 0.8, 0.5),
    ));

    // Update storage containers with new data (for event handlers to reference)
    *nodes_data_storage.borrow_mut() = nodes_data;
    *connections_data_storage.borrow_mut() = connections_data;

    // Wrap meshes in Rc<RefCell> for render closure
    let node_meshes = Rc::new(RefCell::new(node_meshes));
    let connection_meshes = Rc::new(RefCell::new(connection_meshes));
    let grid_axes_meshes = Rc::new(RefCell::new(grid_axes_meshes)); // RefCell so we can update it

    // Get canvas dimensions
    let canvas_width = canvas.client_width() as u32;
    let canvas_height = canvas.client_height() as u32;

    // Update canvas resolution to match display size
    canvas.set_width(canvas_width);
    canvas.set_height(canvas_height);

    // Render function
    let render_scene = {
        let context = context.clone();
        let node_meshes = node_meshes.clone();
        let connection_meshes = connection_meshes.clone();
        let grid_axes_meshes = grid_axes_meshes.clone();
        let ambient = ambient.clone();
        let key_light = key_light.clone();
        let fill_light = fill_light.clone();
        let rim_light = rim_light.clone();
        let canvas = canvas.clone();
        let selected_node_id_signal = selected_node_id_signal; // Capture signal for render closure
        let selected_item_signal = selected_item_signal; // Capture signal for connection selection
        let use_env_lighting_signal = use_environment_lighting_signal; // Capture HDR signal (not value!)

        move |state: CameraState| {
            // Read HDR environment signal dynamically each frame (if available)
            let use_env_lighting = use_env_lighting_signal
                .map(|sig| sig.get_untracked())
                .unwrap_or(use_environment_lighting); // Fallback to initial value
            // Always get current canvas dimensions (handles window resize and fullscreen toggle)
            let width = canvas.client_width() as u32;
            let height = canvas.client_height() as u32;

            // Update canvas resolution to match display size (prevents distortion)
            canvas.set_width(width);
            canvas.set_height(height);

            let viewport = Viewport::new_at_origo(width, height);

            // Calculate camera position from spherical coordinates
            let target = vec3(state.pan_x, state.pan_y, 0.0);    // look at pan offset
            let eye = target + vec3(
                state.distance * state.elevation.cos() * state.azimuth.sin(),
                state.distance * state.elevation.sin(),
                state.distance * state.elevation.cos() * state.azimuth.cos(),
            );

            let camera = Camera::new_perspective(
                viewport,
                eye,
                target,    // look at pan-adjusted target
                vec3(0.0, 0.0, 1.0),    // up = +Z (Blender convention)
                degrees(45.0),
                0.1,
                1000.0,
            );

            let clear_state = match background_color {
                Some((r, g, b)) => ClearState::color_and_depth(
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                    1.0,  // Opaque
                    1.0   // Depth
                ),
                None => ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0), // Transparent background
            };
            let target = RenderTarget::screen(&context, width, height);
            target.clear(clear_state);

            // Render grid and axes first (background reference)
            for mesh in grid_axes_meshes.borrow().iter() {
                target.render(&camera, mesh, &[]);
            }

            // Get currently selected item (untracked - we handle reactivity via Effect)
            let selected_item = selected_item_signal.get_untracked();

            // Render connections (use selected mesh if connection is selected)
            let connections_to_render = connection_meshes.borrow();
            for (conn_id, normal_mesh, selected_mesh) in connections_to_render.iter() {
                let is_selected = matches!(selected_item, Some(crate::islands::topology_editor::SelectedItem::Connection(id)) if id == *conn_id);
                let mesh_to_render = if is_selected {
                    selected_mesh
                } else {
                    normal_mesh
                };
                target.render(&camera, mesh_to_render, &[]);
            }

            // Get currently selected node ID (untracked - we handle reactivity via Effect)
            let selected_id = selected_node_id_signal.get_untracked();

            // Render nodes (use selected material if node is selected)
            for (node_id, normal_mesh, selected_mesh) in node_meshes.borrow().iter() {
                let is_selected = Some(*node_id) == selected_id;
                let mesh_to_render = if is_selected {
                    selected_mesh
                } else {
                    normal_mesh
                };
                // Conditional lighting based on environment mode
                if use_env_lighting {
                    // HDR environment lighting mode: Use ONLY ambient (which contains HDR environment)
                    // The HDR environment map provides comprehensive lighting - no additional lights needed
                    target.render(&camera, mesh_to_render, &[&*ambient]);
                } else {
                    // Manual lighting mode: Use full three-point lighting setup
                    target.render(&camera, mesh_to_render, &[&*ambient, &*key_light, &*fill_light, &*rim_light]);
                }
            }

            // Debug logging (only on first frame)
            use std::sync::atomic::{AtomicBool, Ordering};
            static LOGGED: AtomicBool = AtomicBool::new(false);
            if !LOGGED.swap(true, Ordering::Relaxed) {
                web_sys::console::log_1(&format!(
                    "🔦 Lighting mode: {}, lights used: {}",
                    if use_env_lighting { "HDR Environment" } else { "Manual 3-Point" },
                    if use_env_lighting { "1 (ambient only)" } else { "4 (ambient + key + fill + rim)" }
                ).into());
            }
        }
    };

    // Wrap render_scene in Rc for sharing
    let render_scene = Rc::new(render_scene);

    // Store render function so component-level Effect can call it
    *render_fn_storage.borrow_mut() = Some(render_scene.clone());

    // Initial render
    render_scene(camera_state.get_untracked());

    // Set up orbit controls with integrated click handler and tooltip
    // ONLY on first initialization - skip on refetches to avoid duplicate handlers
    if !skip_event_handlers {
        setup_orbit_controls(
            canvas,
            camera_state,
            render_fn_storage.clone(), // Pass storage so handlers always use latest render function
            nodes_data_storage.clone(), // Pass storage so handlers always reference latest data
            connections_data_storage.clone(), // Pass storage so handlers always reference latest data
            selected_node_id_signal,
            selected_item_signal,
            tooltip_data,
            connection_mode,
            refetch_trigger,
            current_topology_id,
        )?;
    }

    Ok(())
}

/// Initialize three-d Context with test cube (fallback when no topology data)
#[cfg(feature = "hydrate")]
fn initialize_threed_viewport_test(
    canvas: &web_sys::HtmlCanvasElement,
    camera_state: RwSignal<CameraState>,
    selected_node_id_signal: RwSignal<Option<i64>>,
    selected_item_signal: RwSignal<Option<crate::islands::topology_editor::SelectedItem>>,
    render_fn_storage: Rc<RefCell<Option<Rc<dyn Fn(CameraState)>>>>,
    tooltip_data: RwSignal<Option<(String, String, f64, f64)>>,
    background_color: Option<(u8, u8, u8)>,
    connection_mode: Option<RwSignal<crate::islands::topology_editor::ConnectionMode>>,
    refetch_trigger: Option<RwSignal<u32>>,
    current_topology_id: Option<RwSignal<i64>>,
) -> Result<(), String> {
    use web_sys::WebGl2RenderingContext as GL;
    use three_d::*;

    // Create WebGL2 context attributes with preserveDrawingBuffer enabled
    // This is required for canvas.toDataURL() to work properly
    use wasm_bindgen::JsValue;
    use js_sys::Object;
    use js_sys::Reflect;

    let context_options = Object::new();
    Reflect::set(
        &context_options,
        &JsValue::from_str("preserveDrawingBuffer"),
        &JsValue::from_bool(true),
    ).map_err(|e| format!("Failed to set preserveDrawingBuffer: {:?}", e))?;

    // Get WebGL2 context from canvas with options
    let webgl2_context = canvas
        .get_context_with_context_options("webgl2", &context_options)
        .map_err(|e| format!("Failed to get WebGL2 context: {:?}", e))?
        .ok_or("WebGL2 context is None")?
        .dyn_into::<GL>()
        .map_err(|e| format!("Failed to cast to WebGL2 context: {:?}", e))?;

    // Wrap in glow::Context (three-d uses glow internally)
    let gl = three_d::context::Context::from_webgl2_context(webgl2_context);

    // Create three-d Context from glow context
    let context = Context::from_gl_context(std::sync::Arc::new(gl))
        .map_err(|e| format!("Failed to create three-d Context: {:?}", e))?;

    // Create test cube mesh with PBR material
    let cube = Rc::new(RefCell::new(Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new(100, 200, 255, 255),
                metallic: 0.5,
                roughness: 0.4,
                ..Default::default()
            },
        ),
    )));
    cube.borrow_mut().set_transformation(Mat4::from_scale(1.5));

    // Get canvas dimensions
    let canvas_width = canvas.client_width() as u32;
    let canvas_height = canvas.client_height() as u32;

    // Update canvas resolution to match display size
    canvas.set_width(canvas_width);
    canvas.set_height(canvas_height);

    // Create professional three-point lighting setup (same as main viewport)
    let ambient = Rc::new(AmbientLight::new(&context, 0.3, Srgba::WHITE));
    let key_light = Rc::new(DirectionalLight::new(
        &context,
        1.8,
        Srgba::new(255, 248, 240, 255),
        vec3(-0.5, -0.3, -1.0),
    ));
    let fill_light = Rc::new(DirectionalLight::new(
        &context,
        0.6,
        Srgba::new(200, 220, 255, 255),
        vec3(1.0, 0.5, -0.3),
    ));
    let rim_light = Rc::new(DirectionalLight::new(
        &context,
        0.4,
        Srgba::new(220, 230, 255, 255),
        vec3(0.3, 0.8, 0.5),
    ));

    // Render function that uses current camera state
    let render_scene = {
        let context = context.clone();
        let cube = cube.clone();
        let ambient = ambient.clone();
        let key_light = key_light.clone();
        let fill_light = fill_light.clone();
        let rim_light = rim_light.clone();
        let canvas = canvas.clone();

        move |state: CameraState| {
            // Always get current canvas dimensions (handles window resize and fullscreen toggle)
            let width = canvas.client_width() as u32;
            let height = canvas.client_height() as u32;

            // Update canvas resolution to match display size (prevents distortion)
            canvas.set_width(width);
            canvas.set_height(height);

            let viewport = Viewport::new_at_origo(width, height);

            // Calculate camera position from spherical coordinates
            let target = vec3(state.pan_x, state.pan_y, 0.0);
            let eye = target + vec3(
                state.distance * state.elevation.cos() * state.azimuth.sin(),
                state.distance * state.elevation.sin(),
                state.distance * state.elevation.cos() * state.azimuth.cos(),
            );

            let camera = Camera::new_perspective(
                viewport,
                eye,
                target, // look at pan-adjusted target
                vec3(0.0, 0.0, 1.0), // up = +Z
                degrees(45.0),
                0.1,
                1000.0,
            );

            let clear_state = match background_color {
                Some((r, g, b)) => ClearState::color_and_depth(
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                    1.0,  // Opaque
                    1.0   // Depth
                ),
                None => ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0), // Transparent background
            };

            RenderTarget::screen(&context, width, height)
                .clear(clear_state)
                .render(&camera, &*cube.borrow(), &[&*ambient, &*key_light, &*fill_light, &*rim_light]);
        }
    };

    // Wrap render_scene in Rc for sharing
    let render_scene = Rc::new(render_scene);

    // Store render function so component-level Effect can call it
    *render_fn_storage.borrow_mut() = Some(render_scene.clone());

    // Initial render
    render_scene(camera_state.get_untracked());

    // Set up mouse drag for orbit (no node selection for test scene - use empty storage)
    let empty_nodes = Rc::new(RefCell::new(Vec::new()));
    let empty_connections = Rc::new(RefCell::new(Vec::new()));
    setup_orbit_controls(canvas, camera_state, render_fn_storage.clone(), empty_nodes, empty_connections, selected_node_id_signal, selected_item_signal, tooltip_data, connection_mode, refetch_trigger, current_topology_id)?;

    Ok(())
}

/// Set up mouse and scroll event handlers for orbit controls
#[cfg(feature = "hydrate")]
fn setup_orbit_controls(
    canvas: &web_sys::HtmlCanvasElement,
    camera_state: RwSignal<CameraState>,
    render_fn_storage: Rc<RefCell<Option<Rc<dyn Fn(CameraState)>>>>,
    nodes_data: Rc<RefCell<Vec<NodeData>>>,
    connections_data: Rc<RefCell<Vec<ConnectionData>>>,
    selected_node_id_signal: RwSignal<Option<i64>>,
    selected_item_signal: RwSignal<Option<crate::islands::topology_editor::SelectedItem>>,
    tooltip_data: RwSignal<Option<(String, String, f64, f64)>>,
    connection_mode: Option<RwSignal<crate::islands::topology_editor::ConnectionMode>>,
    refetch_trigger: Option<RwSignal<u32>>,
    current_topology_id: Option<RwSignal<i64>>,
) -> Result<(), String> {
    use web_sys::{MouseEvent, WheelEvent};
    use three_d::*;

    use std::sync::{Arc, Mutex};

    // Generate unique handler ID for debugging
    static HANDLER_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let _handler_id = HANDLER_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let is_dragging = Rc::new(RefCell::new(false));
    let last_mouse_pos = Rc::new(RefCell::new((0.0, 0.0)));
    let mouse_down_pos = Rc::new(RefCell::new((0.0, 0.0))); // Track where mouse was pressed
    let total_mouse_movement = Rc::new(RefCell::new(0.0)); // Track total movement distance

    // Track if component is disposed - prevents accessing disposed signals
    // Must use Arc + Mutex for thread-safe access (required by on_cleanup)
    let is_disposed = Arc::new(Mutex::new(false));

    // Store camera state in a non-reactive way for safe access from event handlers
    // This avoids the disposed signal issue entirely
    let camera_state_snapshot = Arc::new(Mutex::new(camera_state.get_untracked()));

    // Mouse down - start dragging
    {
        let is_dragging = is_dragging.clone();
        let last_mouse_pos = last_mouse_pos.clone();
        let mouse_down_pos = mouse_down_pos.clone();
        let total_mouse_movement = total_mouse_movement.clone();
        let canvas_clone = canvas.clone();
        let camera_state_snapshot = camera_state_snapshot.clone();

        let mousedown = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |e: MouseEvent| {
            let pos = (e.client_x() as f32, e.client_y() as f32);
            *is_dragging.borrow_mut() = true;
            *last_mouse_pos.borrow_mut() = pos;
            *mouse_down_pos.borrow_mut() = pos; // Remember where we started
            *total_mouse_movement.borrow_mut() = 0.0; // Reset movement counter

            // Sync camera snapshot from current signal value - allows dragging from preset positions
            *camera_state_snapshot.lock().unwrap() = camera_state.get_untracked();

            canvas_clone.set_attribute("style", "cursor: grabbing; border: 1px solid #ccc; display: block; background-color: #1a1a1a;").ok();
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref())
            .map_err(|e| format!("Failed to add mousedown listener: {:?}", e))?;

        mousedown.forget(); // Leak closure - will be cleaned up when page unloads
    }

    // Mouse up - stop dragging OR handle click
    {
        let is_dragging = is_dragging.clone();
        let total_mouse_movement = total_mouse_movement.clone();
        let canvas_clone = canvas.clone();
        let nodes_data = nodes_data.clone(); // Clone for closure
        let connections_data = connections_data.clone(); // Clone for closure
        let is_disposed = is_disposed.clone(); // Clone for disposal check
        let camera_state_snapshot = camera_state_snapshot.clone();
        let render_fn_storage = render_fn_storage.clone(); // Clone storage to always use latest render function

        let mouseup = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |e: MouseEvent| {
            let was_dragging = *is_dragging.borrow();
            *is_dragging.borrow_mut() = false;
            canvas_clone.set_attribute("style", "cursor: pointer; border: 1px solid #ccc; display: block; background-color: #1a1a1a;").ok();


            // If total movement is very small, treat as a click for node selection
            let movement = *total_mouse_movement.borrow();
            if was_dragging && movement < 5.0 {
                // Check if component is disposed before accessing signals
                if *is_disposed.lock().unwrap() {
                    return;
                }

                // Perform node selection using stored data
                {
                    let nodes = nodes_data.borrow(); // Borrow from storage
                    let rect = canvas_clone.get_bounding_client_rect();
                    let x = e.client_x() as f64 - rect.left();
                    let y = e.client_y() as f64 - rect.top();

                    // Convert to normalized device coordinates (-1 to 1)
                    let width = canvas_clone.client_width() as f64;
                    let height = canvas_clone.client_height() as f64;
                    let ndc_x = (x / width) * 2.0 - 1.0;
                    let ndc_y = 1.0 - (y / height) * 2.0;

                    // Use snapshot for raycasting (safe - no reactive signals)
                    let state = *camera_state_snapshot.lock().unwrap();
                    let target = vec3(state.pan_x, state.pan_y, 0.0);
                    let eye = target + vec3(
                        state.distance * state.elevation.cos() * state.azimuth.sin(),
                        state.distance * state.elevation.sin(),
                        state.distance * state.elevation.cos() * state.azimuth.cos(),
                    );
                    let up = vec3(0.0, 0.0, 1.0);  // Z-up (Blender convention)

                    // Calculate camera basis vectors
                    let forward = (target - eye).normalize();
                    let right = forward.cross(up).normalize();
                    let camera_up = right.cross(forward);

                    // Calculate ray direction from camera through click point
                    let fov = 45.0_f32.to_radians();
                    let aspect = width as f32 / height as f32;
                    let tan_fov = (fov / 2.0).tan();

                    let ray_dir = (forward
                        + right * (ndc_x as f32 * tan_fov * aspect)
                        + camera_up * (ndc_y as f32 * tan_fov)).normalize();

                    // Test ray intersection with each node sphere
                    let mut closest_node: Option<(i64, f32)> = None;

                    for node in nodes.iter() {
                        // Ray-sphere intersection test
                        let oc = eye - node.position;
                        let a = ray_dir.dot(ray_dir);
                        let b = 2.0 * oc.dot(ray_dir);
                        let c = oc.dot(oc) - node.radius * node.radius;
                        let discriminant = b * b - 4.0 * a * c;

                        if discriminant >= 0.0 {
                            let t = (-b - discriminant.sqrt()) / (2.0 * a);
                            if t > 0.0 {
                                match closest_node {
                                    None => closest_node = Some((node.id, t)),
                                    Some((_, prev_t)) if t < prev_t => closest_node = Some((node.id, t)),
                                    _ => {}
                                }
                            }
                        }
                    }

                    // Handle node click based on connection mode
                    let selected_id = closest_node.map(|(id, _)| id);

                    // Check connection mode (if available)
                    let current_mode = if let Some(mode_signal) = connection_mode {
                        if !*is_disposed.lock().unwrap() {
                            Some(mode_signal.get_untracked())
                        } else {
                            None
                        }
                    } else {
                        None
                    };


                    if !*is_disposed.lock().unwrap() {
                        match current_mode {
                            Some(crate::islands::topology_editor::ConnectionMode::Disabled) | None => {
                                // Normal selection mode - check nodes first, then connections
                                if selected_id.is_some() {
                                    // Node was clicked
                                    selected_node_id_signal.set(selected_id);
                                    selected_item_signal.set(selected_id.map(crate::islands::topology_editor::SelectedItem::Node));
                                } else {
                                    // No node clicked - check for connection clicks using stored data
                                    let connections = connections_data.borrow();
                                    let mut closest_connection: Option<(i64, f32)> = None;

                                    for conn in connections.iter() {
                                        if let Some(t) = ray_cylinder_intersection(
                                            eye,
                                            ray_dir,
                                            conn.source_pos,
                                            conn.target_pos,
                                            conn.radius,
                                        ) {
                                            match closest_connection {
                                                None => closest_connection = Some((conn.id, t)),
                                                Some((_, prev_t)) if t < prev_t => closest_connection = Some((conn.id, t)),
                                                _ => {}
                                            }
                                        }
                                    }

                                    if let Some((conn_id, _)) = closest_connection {
                                        // Connection was clicked
                                        selected_node_id_signal.set(None);
                                        selected_item_signal.set(Some(crate::islands::topology_editor::SelectedItem::Connection(conn_id)));
                                    } else {
                                        // Empty space clicked - deselect
                                        selected_node_id_signal.set(None);
                                        selected_item_signal.set(None);
                                    }
                                }
                            }
                            Some(crate::islands::topology_editor::ConnectionMode::SelectingFirstNode) => {
                                // First node selected - transition to selecting second node
                                if let Some(node_id) = selected_id {
                                    if let Some(mode_signal) = connection_mode {
                                        mode_signal.set(crate::islands::topology_editor::ConnectionMode::SelectingSecondNode {
                                            first_node_id: node_id,
                                        });
                                    } else {
                                        web_sys::console::error_1(&"  ERROR: connection_mode is None!".into());
                                    }
                                    // Also update selection to show which node was picked
                                    selected_node_id_signal.set(selected_id);
                                    selected_item_signal.set(selected_id.map(crate::islands::topology_editor::SelectedItem::Node));
                                } else {
                                    // Clicked empty space - deselect to ensure render happens
                                    selected_node_id_signal.set(None);
                                    selected_item_signal.set(None);
                                }
                            }
                            Some(crate::islands::topology_editor::ConnectionMode::SelectingSecondNode { first_node_id }) => {
                                // Second node selected - create connection
                                if let Some(second_node_id) = selected_id {

                                    // Create connection via server function
                                    if let (Some(mode_signal), Some(trigger), Some(topo_id_signal)) = (connection_mode, refetch_trigger, current_topology_id) {
                                        let topology_id = topo_id_signal.get_untracked();

                                        // Spawn async task to create connection
                                        wasm_bindgen_futures::spawn_local(async move {
                                            use crate::api::create_connection;
                                            use crate::models::CreateConnection;

                                            let data = CreateConnection {
                                                topology_id,
                                                source_node_id: first_node_id,
                                                target_node_id: second_node_id,
                                                connection_type: Some("ethernet".to_string()),
                                                bandwidth_mbps: Some(1000),
                                                latency_ms: Some(1.0),
                                                status: Some("active".to_string()),
                                                color: None, // Use default color
                                                metadata: None,
                                            };

                                            match create_connection(data).await {
                                                Ok(_conn) => {
                                                    // Stay in connection mode - reset to SelectingFirstNode so user can create more connections
                                                    mode_signal.set(crate::islands::topology_editor::ConnectionMode::SelectingFirstNode);
                                                    // Trigger viewport refetch to show new connection
                                                    trigger.update(|v| *v += 1);
                                                }
                                                Err(e) => {
                                                    web_sys::console::error_1(&format!("✗ Failed to create connection: {}", e).into());
                                                    // On error, also go back to SelectingFirstNode to allow retry
                                                    mode_signal.set(crate::islands::topology_editor::ConnectionMode::SelectingFirstNode);
                                                }
                                            }
                                        });
                                    }
                                } else {
                                    // Clicked empty space in SelectingSecondNode - cancel and go back to first node selection
                                    if let Some(mode_signal) = connection_mode {
                                        mode_signal.set(crate::islands::topology_editor::ConnectionMode::SelectingFirstNode);
                                    }
                                    // Deselect to trigger re-render
                                    selected_node_id_signal.set(None);
                                    selected_item_signal.set(None);
                                }
                            }
                        }
                    }

                    // Trigger re-render immediately to show selection (use snapshot, borrow latest render function)
                    if let Some(render_fn) = render_fn_storage.borrow().as_ref() {
                        render_fn(*camera_state_snapshot.lock().unwrap());
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref())
            .map_err(|e| format!("Failed to add mouseup listener: {:?}", e))?;

        mouseup.forget(); // Leak closure - will be cleaned up when page unloads
    }

    // Mouse move - rotate camera and update tooltip
    {
        let is_dragging = is_dragging.clone();
        let last_mouse_pos = last_mouse_pos.clone();
        let total_mouse_movement = total_mouse_movement.clone();
        let is_disposed = is_disposed.clone(); // Clone for disposal check
        let camera_state_snapshot = camera_state_snapshot.clone();
        let render_fn_storage = render_fn_storage.clone(); // Clone storage to always use latest render function
        let nodes_data = nodes_data.clone();
        let canvas_clone = canvas.clone();

        let mousemove = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |e: MouseEvent| {
            // Check if component is disposed before accessing signals
            if *is_disposed.lock().unwrap() {
                return;
            }

            if *is_dragging.borrow() {
                let current_pos = (e.client_x() as f32, e.client_y() as f32);
                let last_pos = *last_mouse_pos.borrow();

                let delta_x = current_pos.0 - last_pos.0;
                let delta_y = current_pos.1 - last_pos.1;

                // Track total movement distance
                let movement_dist = (delta_x * delta_x + delta_y * delta_y).sqrt();
                *total_mouse_movement.borrow_mut() += movement_dist;

                // Update snapshot (safe - no reactive signals)
                let mut state = camera_state_snapshot.lock().unwrap();

                // Pan mode: Middle mouse button OR Shift+drag
                let is_pan_mode = e.button() == 1 || e.shift_key();

                if is_pan_mode {
                    // Pan camera (move target point)
                    let pan_speed = state.distance * 0.001; // Scale with camera distance
                    state.pan_x -= delta_x * pan_speed;
                    state.pan_y += delta_y * pan_speed; // Invert Y (screen space -> world space)
                } else {
                    // Rotate camera (orbit mode)
                    state.azimuth += delta_x * 0.01;
                    state.elevation = (state.elevation - delta_y * 0.01).clamp(-1.5, 1.5);
                }

                // Only update reactive signal if not disposed
                if !*is_disposed.lock().unwrap() {
                    camera_state.set(*state);
                }

                // Render using latest render function from storage
                if let Some(render_fn) = render_fn_storage.borrow().as_ref() {
                    render_fn(*state);
                }
                drop(state); // Release lock

                *last_mouse_pos.borrow_mut() = current_pos;

                // Clear tooltip while dragging
                if !*is_disposed.lock().unwrap() {
                    tooltip_data.set(None);
                }
            } else {
                // Not dragging - check for hover and update tooltip
                // Borrow from storage to get latest node data
                {
                    let nodes = nodes_data.borrow();
                    let rect = canvas_clone.get_bounding_client_rect();
                    let x = e.client_x() as f64 - rect.left();
                    let y = e.client_y() as f64 - rect.top();
                    let width = canvas_clone.client_width() as f64;
                    let height = canvas_clone.client_height() as f64;
                    let ndc_x = (x / width) * 2.0 - 1.0;
                    let ndc_y = 1.0 - (y / height) * 2.0;

                    // Use snapshot for hover detection (safe - no reactive signals)
                    let state = *camera_state_snapshot.lock().unwrap();
                    let target = vec3(state.pan_x, state.pan_y, 0.0);
                    let eye = target + vec3(
                        state.distance * state.elevation.cos() * state.azimuth.sin(),
                        state.distance * state.elevation.sin(),
                        state.distance * state.elevation.cos() * state.azimuth.cos(),
                    );
                    let up = vec3(0.0, 0.0, 1.0);

                    let forward = (target - eye).normalize();
                    let right = forward.cross(up).normalize();
                    let camera_up = right.cross(forward);

                    let fov = 45.0_f32.to_radians();
                    let aspect = width as f32 / height as f32;
                    let tan_fov = (fov / 2.0).tan();

                    let ray_dir = (forward
                        + right * (ndc_x as f32 * tan_fov * aspect)
                        + camera_up * (ndc_y as f32 * tan_fov)).normalize();

                    // Test ray intersection with each node
                    let mut hovered_node: Option<&NodeData> = None;
                    let mut closest_t = f32::MAX;

                    for node in nodes.iter() {
                        let oc = eye - node.position;
                        let a = ray_dir.dot(ray_dir);
                        let b = 2.0 * oc.dot(ray_dir);
                        let c = oc.dot(oc) - node.radius * node.radius;
                        let discriminant = b * b - 4.0 * a * c;

                        if discriminant >= 0.0 {
                            let t = (-b - discriminant.sqrt()) / (2.0 * a);
                            if t > 0.0 && t < closest_t {
                                closest_t = t;
                                hovered_node = Some(node);
                            }
                        }
                    }

                    // Update tooltip data with canvas-relative coordinates (guard against disposal)
                    if !*is_disposed.lock().unwrap() {
                        if let Some(node) = hovered_node {
                            tooltip_data.set(Some((node.name.clone(), node.node_type.clone(), x, y)));
                        } else {
                            tooltip_data.set(None);
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref())
            .map_err(|e| format!("Failed to add mousemove listener: {:?}", e))?;

        mousemove.forget(); // Leak closure - will be cleaned up when page unloads
    }

    // Mouse wheel - zoom
    {
        let is_disposed = is_disposed.clone(); // Clone for disposal check
        let camera_state_snapshot = camera_state_snapshot.clone();
        let render_fn_storage = render_fn_storage.clone(); // Clone storage to always use latest render function

        let wheel = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |e: WheelEvent| {
            e.prevent_default();

            // Check if component is disposed before accessing signals
            if *is_disposed.lock().unwrap() {
                return;
            }

            // Update snapshot (safe - no reactive signals)
            let mut state = camera_state_snapshot.lock().unwrap();
            state.distance = (state.distance + e.delta_y() as f32 * 0.01).clamp(2.0, 50.0);

            // Only update reactive signal if not disposed
            if !*is_disposed.lock().unwrap() {
                camera_state.set(*state);
            }

            // Render using latest render function from storage
            if let Some(render_fn) = render_fn_storage.borrow().as_ref() {
                render_fn(*state);
            }
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("wheel", wheel.as_ref().unchecked_ref())
            .map_err(|e| format!("Failed to add wheel listener: {:?}", e))?;

        wheel.forget(); // Leak closure - will be cleaned up when page unloads
    }

    // Register cleanup callback to mark component as disposed
    // This prevents event handlers from accessing disposed signals
    leptos::prelude::on_cleanup(move || {
        *is_disposed.lock().unwrap() = true;
    });

    Ok(())
}

/// Create grid floor and XYZ axes for spatial reference (Blender-style)
#[cfg(feature = "hydrate")]
fn create_grid_and_axes(
    context: &three_d::Context,
    show_grid: bool,
    show_x_axis: bool,
    show_y_axis: bool,
    show_z_axis: bool,
) -> Vec<three_d::Gm<three_d::Mesh, three_d::ColorMaterial>> {
    use three_d::*;

    let mut meshes = Vec::new();

    // Grid parameters (Blender convention: XY plane floor, Z is up)
    let grid_size = 10; // 10 units in each direction from origin
    let grid_spacing = 1.0; // 1 unit between lines
    let grid_z = 0.0; // Floor at Z=0 (Blender convention)
    let grid_line_thickness = 0.006; // Very thin lines
    let axis_line_thickness = 0.006; // Same thickness as grid lines (half of previous 0.012)

    let cylinder_cpu_mesh = CpuMesh::cylinder(8); // 8-sided cylinder for lines

    // Create grid lines only if enabled
    if show_grid {
        let grid_color = Srgba::new(50, 50, 50, 180); // Faint dark gray with transparency

        // Lines parallel to X axis (varying Y) - these go left-right
        for i in -grid_size..=grid_size {
            let y = i as f32 * grid_spacing;
            let start = vec3(-grid_size as f32 * grid_spacing, y, grid_z);
            let end = vec3(grid_size as f32 * grid_spacing, y, grid_z);

            if let Some(line_mesh) = create_line_cylinder(context, start, end, grid_line_thickness, grid_color, &cylinder_cpu_mesh) {
                meshes.push(line_mesh);
            }
        }

        // Lines parallel to Y axis (varying X) - these go front-back
        for i in -grid_size..=grid_size {
            let x = i as f32 * grid_spacing;
            let start = vec3(x, -grid_size as f32 * grid_spacing, grid_z);
            let end = vec3(x, grid_size as f32 * grid_spacing, grid_z);

            if let Some(line_mesh) = create_line_cylinder(context, start, end, grid_line_thickness, grid_color, &cylinder_cpu_mesh) {
                meshes.push(line_mesh);
            }
        }
    }

    // Create XYZ axis lines (span full grid extent in both directions)
    let axis_length = 15.0;

    // X axis (Red) - left to right on floor
    if show_x_axis {
        if let Some(x_axis) = create_line_cylinder(
            context,
            vec3(-axis_length, 0.0, 0.0),
            vec3(axis_length, 0.0, 0.0),
            axis_line_thickness,
            Srgba::new(200, 80, 80, 200), // Faint red with transparency
            &cylinder_cpu_mesh,
        ) {
            meshes.push(x_axis);
        }
    }

    // Y axis (Green) - front to back on floor, along three-d Y coordinate
    if show_y_axis {
        if let Some(y_axis) = create_line_cylinder(
            context,
            vec3(0.0, -axis_length, 0.0),
            vec3(0.0, axis_length, 0.0),
            axis_line_thickness,
            Srgba::new(80, 200, 80, 200), // Faint green with transparency
            &cylinder_cpu_mesh,
        ) {
            meshes.push(y_axis);
        }
    }

    // Z axis (Blue) - vertical up/down, along three-d Z coordinate
    if show_z_axis {
        if let Some(z_axis) = create_line_cylinder(
            context,
            vec3(0.0, 0.0, -axis_length),
            vec3(0.0, 0.0, axis_length),
            axis_line_thickness,
            Srgba::new(80, 160, 240, 25), // Extremely transparent blue (barely visible)
            &cylinder_cpu_mesh,
        ) {
            meshes.push(z_axis);
        }
    }

    meshes
}

/// Helper function to create a thin cylinder between two points (for lines)
#[cfg(feature = "hydrate")]
fn create_line_cylinder(
    context: &three_d::Context,
    start: three_d::Vec3,
    end: three_d::Vec3,
    thickness: f32,
    color: three_d::Srgba,
    cylinder_cpu_mesh: &three_d::CpuMesh,
) -> Option<three_d::Gm<three_d::Mesh, three_d::ColorMaterial>> {
    use three_d::*;

    let direction = end - start;
    let length = direction.magnitude();

    if length < 0.001 {
        return None; // Skip zero-length lines
    }

    let midpoint = start + direction * 0.5;
    let normalized_dir = direction.normalize();
    let up = vec3(0.0, 1.0, 0.0);

    // Calculate rotation to align cylinder with line direction
    let rotation = if (normalized_dir - up).magnitude() < 0.001 {
        Mat4::identity()
    } else if (normalized_dir + up).magnitude() < 0.001 {
        Mat4::from_angle_x(radians(std::f32::consts::PI))
    } else {
        let axis = up.cross(normalized_dir).normalize();
        let angle = up.dot(normalized_dir).acos();
        Mat4::from_axis_angle(axis, radians(angle))
    };

    let mut cylinder = Gm::new(
        Mesh::new(context, cylinder_cpu_mesh),
        ColorMaterial {
            color,
            ..Default::default()
        },
    );

    // Transform: translate to midpoint, rotate to align, then scale
    let scale = Mat4::from_nonuniform_scale(thickness, length * 0.5, thickness);
    cylinder.set_transformation(Mat4::from_translation(midpoint) * rotation * scale);

    Some(cylinder)
}

/// Map node type to color
#[cfg(feature = "hydrate")]
fn get_node_color(node_type: &str) -> three_d::Srgba {
    use three_d::Srgba;

    match node_type.to_lowercase().as_str() {
        "router" => Srgba::new(255, 140, 60, 255),   // Orange - routing/core device
        "switch" => Srgba::new(80, 200, 120, 255),   // Green - switching/connecting
        "server" => Srgba::new(70, 140, 255, 255),   // Blue - computing/services
        "firewall" => Srgba::new(220, 60, 60, 255),  // Red - security/protection
        "load_balancer" => Srgba::new(180, 100, 200, 255), // Purple - load distribution
        "host" | "client" => Srgba::new(150, 150, 150, 255), // Gray - generic host
        _ => Srgba::new(120, 120, 120, 255),         // Dark gray - unknown type
    }
}

/// Get material properties (metallic, roughness) based on node type
/// Returns (metallic, roughness) tuple
#[cfg(feature = "hydrate")]
fn get_node_material_properties(node_type: &str) -> (f32, f32) {
    match node_type.to_lowercase().as_str() {
        "router" => (0.6, 0.3),       // Metallic and smooth - metal enclosure
        "switch" => (0.5, 0.4),       // Slightly metallic, medium roughness
        "server" => (0.2, 0.6),       // Less metallic, more matte - server chassis
        "firewall" => (0.7, 0.2),     // Very metallic and smooth - hardened hardware
        "load_balancer" => (0.4, 0.5), // Medium metallic, medium roughness
        "host" | "client" => (0.3, 0.7), // Low metallic, rough - desktop/laptop
        _ => (0.4, 0.5),              // Default: medium values
    }
}

