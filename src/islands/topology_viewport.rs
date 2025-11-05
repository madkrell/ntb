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
#[derive(Clone, Copy)]
struct CameraState {
    distance: f32,
    azimuth: f32,   // horizontal rotation (radians)
    elevation: f32, // vertical rotation (radians)
}

#[cfg(feature = "hydrate")]
impl Default for CameraState {
    fn default() -> Self {
        Self {
            distance: 18.0,        // Zoomed out to show full topology
            azimuth: -0.785,       // ~-45 degrees (Blender default: green Y axis lower-left to upper-right)
            elevation: 1.047,      // ~60 degrees (looking down from above, Blender-style)
        }
    }
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
    let selected_item = use_context::<RwSignal<Option<crate::islands::topology_editor::SelectedItem>>>().expect("selected_item context");

    // Get connection mode from context (optional - may not exist)
    let connection_mode = use_context::<RwSignal<crate::islands::topology_editor::ConnectionMode>>();

    // Get grid/axes visibility controls from context (optional - may not exist)
    let viewport_visibility = use_context::<crate::islands::topology_editor::ViewportVisibility>();
    let (show_grid, show_x_axis, show_y_axis, show_z_axis) = match viewport_visibility {
        Some(vis) => (vis.show_grid, vis.show_x_axis, vis.show_y_axis, vis.show_z_axis),
        None => (RwSignal::new(true), RwSignal::new(true), RwSignal::new(true), RwSignal::new(true)),
    };

    let canvas_ref = NodeRef::<Canvas>::new();
    let error_signal = RwSignal::new(None::<String>);
    let is_initialized = RwSignal::new(false);

    // Tooltip state: (node_name, node_type, x, y)
    let tooltip_data = RwSignal::new(None::<(String, String, f64, f64)>);

    // Create signal for topology_id (needed for connection creation)
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
                    let show_grid_val = show_grid.get_untracked();
                    let show_x_val = show_x_axis.get_untracked();
                    let show_y_val = show_y_axis.get_untracked();
                    let show_z_val = show_z_axis.get_untracked();

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
                            connection_mode,
                            refetch_trigger,
                            Some(current_topology_id),
                            already_initialized, // Skip event handlers on refetch
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
                    match initialize_threed_viewport_test(&canvas_element, camera_state, selected_node_id, selected_item, render_fn_for_effect.clone(), tooltip_data, connection_mode, refetch_trigger, Some(current_topology_id)) {
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
        </div>
    }
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
    connection_mode: Option<RwSignal<crate::islands::topology_editor::ConnectionMode>>,
    refetch_trigger: Option<RwSignal<u32>>,
    current_topology_id: Option<RwSignal<i64>>,
    skip_event_handlers: bool, // Set to true on refetches to avoid duplicate handlers
) -> Result<(), String> {
    use web_sys::WebGl2RenderingContext as GL;
    use three_d::*;
    use std::collections::HashMap;

    // Get WebGL2 context from canvas
    let webgl2_context = canvas
        .get_context("webgl2")
        .map_err(|e| format!("Failed to get WebGL2 context: {:?}", e))?
        .ok_or("WebGL2 context is None")?
        .dyn_into::<GL>()
        .map_err(|e| format!("Failed to cast to WebGL2 context: {:?}", e))?;

    // Wrap in glow::Context
    let gl = three_d::context::Context::from_webgl2_context(webgl2_context);
    let context = Context::from_gl_context(std::sync::Arc::new(gl))
        .map_err(|e| format!("Failed to create three-d Context: {:?}", e))?;

    // Load all GLB models for different node types (async loading)
    // Map node type to model filename
    let model_files: Vec<(&str, &str)> = vec![
        ("router", "blob-router.glb"),
        ("switch", "blob-switch.glb"),
        ("server", "blob-server.glb"),
        ("firewall", "blob-firewall.glb"),
        ("load_balancer", "blob-load-balancer.glb"),
        ("cloud", "blob-cloud.glb"),
    ];

    let mut node_models: HashMap<String, Option<three_d_asset::Model>> = HashMap::new();

    {
        use three_d_asset::io::load_async;

        // Build full URL from window.location
        let window = web_sys::window().expect("no global window");
        let location = window.location();
        let origin = location.origin().expect("no origin");

        // Load each model type
        for (node_type, filename) in model_files {
            let model_url = format!("{}/models/{}", origin, filename);

            match load_async(&[model_url.as_str()]).await {
                Ok(mut loaded) => {

                    // Deserialize the GLB file to CPU model
                    match loaded.deserialize::<three_d_asset::Model>(filename) {
                        Ok(cpu_model) => {
                            node_models.insert(node_type.to_string(), Some(cpu_model));
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("✗ Failed to deserialize {} GLB: {:?}", node_type, e).into());
                            node_models.insert(node_type.to_string(), None);
                        }
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("✗ Failed to load {} GLB file: {:?}", node_type, e).into());
                    node_models.insert(node_type.to_string(), None);
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
    let selected_material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Srgba::new(255, 200, 50, 255), // Yellow/orange for selected nodes
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

        // Check if we have a loaded 3D model for this node type
        let node_type_key = node.node_type.to_lowercase();
        let has_model = node_models.get(&node_type_key).and_then(|opt| opt.as_ref()).is_some();

        if has_model {
            // Render node with loaded 3D model
            let cpu_model = node_models.get(&node_type_key).unwrap().as_ref().unwrap();

            // Process each primitive (sub-mesh) in the model
            for primitive in cpu_model.geometries.iter() {
                // Geometry is an enum: Triangles(TriMesh) or Points(PointCloud)
                // We only handle triangle meshes
                match &primitive.geometry {
                    three_d_asset::geometry::Geometry::Triangles(tri_mesh) => {
                        // tri_mesh is a TriMesh, which is the same as CpuMesh!
                        // Just pass it directly to Mesh::new()

                        // Create materials with color based on node type
                        let node_color = get_node_color(&node.node_type);
                        let normal_material = PhysicalMaterial::new_opaque(
                            &context,
                            &CpuMaterial {
                                albedo: node_color,
                                ..Default::default()
                            },
                        );

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
                            * Mat4::from_scale(node_radius)
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
            let node_color = get_node_color(&node.node_type);

            // Create material for this node type
            let normal_material = PhysicalMaterial::new_opaque(
                &context,
                &CpuMaterial {
                    albedo: node_color,
                    ..Default::default()
                },
            );

            // Create normal sphere (new mesh for each node)
            let mut normal_sphere = Gm::new(
                Mesh::new(&context, &sphere_cpu_mesh),
                normal_material,
            );
            normal_sphere.set_transformation(
                Mat4::from_translation(position) * Mat4::from_scale(node_radius)
            );

            // Create selected sphere (same position, different material, new mesh)
            let mut selected_sphere = Gm::new(
                Mesh::new(&context, &sphere_cpu_mesh),
                selected_material.clone(),
            );
            selected_sphere.set_transformation(
                Mat4::from_translation(position) * Mat4::from_scale(node_radius)
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

            // Determine connection color based on type (fiber = cyan, ethernet = light gray)
            let normal_color = match conn.connection_type.as_str() {
                "fiber" => Srgba::new(100, 200, 255, 255),    // Bright cyan for fiber
                "ethernet" => Srgba::new(200, 200, 200, 255), // Light gray for ethernet
                _ => Srgba::new(180, 180, 180, 255),          // Default gray
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

    // Create lights
    let ambient = Rc::new(AmbientLight::new(&context, 0.5, Srgba::WHITE));
    let directional = Rc::new(DirectionalLight::new(
        &context,
        1.5,
        Srgba::WHITE,
        vec3(-1.0, -1.0, -1.0),
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
        let directional = directional.clone();
        let canvas = canvas.clone();
        let selected_node_id_signal = selected_node_id_signal; // Capture signal for render closure
        let selected_item_signal = selected_item_signal; // Capture signal for connection selection

        move |state: CameraState| {
            let width = canvas.client_width() as u32;
            let height = canvas.client_height() as u32;
            let viewport = Viewport::new_at_origo(width, height);

            // Calculate camera position from spherical coordinates
            let eye = vec3(
                state.distance * state.elevation.cos() * state.azimuth.sin(),
                state.distance * state.elevation.sin(),
                state.distance * state.elevation.cos() * state.azimuth.cos(),
            );

            let camera = Camera::new_perspective(
                viewport,
                eye,
                vec3(0.0, 0.0, 0.0),    // look at origin
                vec3(0.0, 0.0, 1.0),    // up = +Z (Blender convention)
                degrees(45.0),
                0.1,
                1000.0,
            );

            let clear_state = ClearState::color_and_depth(0.1, 0.1, 0.1, 1.0, 1.0);
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
                target.render(&camera, mesh_to_render, &[&*ambient, &*directional]);
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
    } else {
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
    connection_mode: Option<RwSignal<crate::islands::topology_editor::ConnectionMode>>,
    refetch_trigger: Option<RwSignal<u32>>,
    current_topology_id: Option<RwSignal<i64>>,
) -> Result<(), String> {
    use web_sys::WebGl2RenderingContext as GL;
    use three_d::*;

    // Get WebGL2 context from canvas
    let webgl2_context = canvas
        .get_context("webgl2")
        .map_err(|e| format!("Failed to get WebGL2 context: {:?}", e))?
        .ok_or("WebGL2 context is None")?
        .dyn_into::<GL>()
        .map_err(|e| format!("Failed to cast to WebGL2 context: {:?}", e))?;

    // Wrap in glow::Context (three-d uses glow internally)
    let gl = three_d::context::Context::from_webgl2_context(webgl2_context);

    // Create three-d Context from glow context
    let context = Context::from_gl_context(std::sync::Arc::new(gl))
        .map_err(|e| format!("Failed to create three-d Context: {:?}", e))?;

    // Create test cube mesh
    let cube = Rc::new(RefCell::new(Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new(100, 200, 255, 255),
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

    // Create lights
    let ambient = Rc::new(AmbientLight::new(&context, 0.4, Srgba::WHITE));
    let directional = Rc::new(DirectionalLight::new(
        &context,
        2.0,
        Srgba::WHITE,
        vec3(-1.0, -1.0, -1.0),
    ));

    // Render function that uses current camera state
    let render_scene = {
        let context = context.clone();
        let cube = cube.clone();
        let ambient = ambient.clone();
        let directional = directional.clone();
        let canvas = canvas.clone();

        move |state: CameraState| {
            let width = canvas.client_width() as u32;
            let height = canvas.client_height() as u32;
            let viewport = Viewport::new_at_origo(width, height);

            // Calculate camera position from spherical coordinates
            let eye = vec3(
                state.distance * state.elevation.cos() * state.azimuth.sin(),
                state.distance * state.elevation.sin(),
                state.distance * state.elevation.cos() * state.azimuth.cos(),
            );

            let camera = Camera::new_perspective(
                viewport,
                eye,
                vec3(0.0, 0.0, 0.0), // look at origin
                vec3(0.0, 1.0, 0.0), // up
                degrees(45.0),
                0.1,
                1000.0,
            );

            let clear_state = ClearState::color_and_depth(0.1, 0.1, 0.1, 1.0, 1.0);

            RenderTarget::screen(&context, width, height)
                .clear(clear_state)
                .render(&camera, &*cube.borrow(), &[&*ambient, &*directional]);
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
    let handler_id = HANDLER_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

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

        let mousedown = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |e: MouseEvent| {
            let pos = (e.client_x() as f32, e.client_y() as f32);
            *is_dragging.borrow_mut() = true;
            *last_mouse_pos.borrow_mut() = pos;
            *mouse_down_pos.borrow_mut() = pos; // Remember where we started
            *total_mouse_movement.borrow_mut() = 0.0; // Reset movement counter
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
                    let eye = vec3(
                        state.distance * state.elevation.cos() * state.azimuth.sin(),
                        state.distance * state.elevation.sin(),
                        state.distance * state.elevation.cos() * state.azimuth.cos(),
                    );
                    let target = vec3(0.0, 0.0, 0.0);
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
                                                metadata: None,
                                            };

                                            match create_connection(data).await {
                                                Ok(conn) => {
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
                state.azimuth += delta_x * 0.01;
                state.elevation = (state.elevation - delta_y * 0.01).clamp(-1.5, 1.5);

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
                    let eye = vec3(
                        state.distance * state.elevation.cos() * state.azimuth.sin(),
                        state.distance * state.elevation.sin(),
                        state.distance * state.elevation.cos() * state.azimuth.cos(),
                    );
                    let target = vec3(0.0, 0.0, 0.0);
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

