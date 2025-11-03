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
    // Get shared state from context (provided by TopologyEditor)
    let selected_node_id = use_context::<RwSignal<Option<i64>>>().expect("selected_node_id context");
    let selected_item = use_context::<RwSignal<Option<crate::islands::topology_editor::SelectedItem>>>().expect("selected_item context");

    let canvas_ref = NodeRef::<Canvas>::new();
    let error_signal = RwSignal::new(None::<String>);
    let is_initialized = RwSignal::new(false);

    // Camera state as signals for reactivity (client-side only)
    #[cfg(feature = "hydrate")]
    let camera_state = RwSignal::new(CameraState::default());

    // Store render function so we can trigger re-renders when selection changes
    #[cfg(feature = "hydrate")]
    let render_fn: Rc<RefCell<Option<Rc<dyn Fn(CameraState)>>>> = Rc::new(RefCell::new(None));

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

    // Clone render_fn for the Effect closure below
    #[cfg(feature = "hydrate")]
    let render_fn_for_effect = render_fn.clone();

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

                    match initialize_threed_viewport(
                        &canvas_element,
                        camera_state,
                        &topo_data,
                        selected_node_id,
                        selected_item,
                        render_fn_for_effect.clone(),
                    ) {
                        Ok(_) => {
                            is_initialized.set(true);
                        }
                        Err(e) => {
                            error_signal.set(Some(e.clone()));
                            web_sys::console::error_1(&format!("Failed to initialize 3D viewport: {}", e).into());
                        }
                    }
                } else if topology_id.is_none() {
                    // Initialize with test scene if no topology_id
                    match initialize_threed_viewport_test(&canvas_element, camera_state, selected_node_id, selected_item, render_fn_for_effect.clone()) {
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

    // Component-level Effect to re-render when selection changes
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
        </div>
    }
}

// Node data for selection
#[cfg(feature = "hydrate")]
struct NodeData {
    id: i64,
    position: three_d::Vec3,
    radius: f32,
}

/// Initialize three-d Context with topology data
#[cfg(feature = "hydrate")]
fn initialize_threed_viewport(
    canvas: &web_sys::HtmlCanvasElement,
    camera_state: RwSignal<CameraState>,
    topology_data: &crate::models::TopologyFull,
    selected_node_id_signal: RwSignal<Option<i64>>,
    selected_item_signal: RwSignal<Option<crate::islands::topology_editor::SelectedItem>>,
    render_fn_storage: Rc<RefCell<Option<Rc<dyn Fn(CameraState)>>>>,
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

    // Create node meshes (spheres at x/y/z positions) and store node data
    let mut node_meshes = Vec::new();
    let mut node_positions = HashMap::new();
    let mut nodes_data = Vec::new();
    let node_radius = 0.3;

    // Create materials for normal and selected states
    let normal_material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Srgba::new(50, 150, 255, 255), // Blue for normal nodes
            ..Default::default()
        },
    );

    let selected_material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Srgba::new(255, 200, 50, 255), // Yellow/orange for selected nodes
            ..Default::default()
        },
    );

    // Create CPU mesh once for reuse
    let sphere_cpu_mesh = CpuMesh::sphere(16);

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

        // Store node data for selection (use larger radius for easier clicking)
        nodes_data.push(NodeData {
            id: node.id,
            position,
            radius: node_radius * 2.0,  // 2x visual radius for easier clicking
        });

        // Create normal sphere (new mesh for each node)
        let mut normal_sphere = Gm::new(
            Mesh::new(&context, &sphere_cpu_mesh),
            normal_material.clone(),
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

    // Create grid and axes for spatial reference
    let grid_axes_meshes = create_grid_and_axes(&context);

    // Create connection meshes (lines between nodes)
    let mut connection_meshes = Vec::new();

    for conn in &topology_data.connections {
        if let (Some(&start_pos), Some(&end_pos)) = (
            node_positions.get(&conn.source_node_id),
            node_positions.get(&conn.target_node_id),
        ) {
            // Create cylinder between two points
            let direction = end_pos - start_pos;
            let length = direction.magnitude();
            let midpoint = start_pos + direction * 0.5;

            // Calculate rotation to align cylinder with connection direction
            // Default cylinder in three-d is along Y axis, so we need to rotate it
            let normalized_dir = direction.normalize();
            let up = vec3(0.0, 1.0, 0.0);

            // Calculate rotation axis (cross product) and angle
            let rotation = if (normalized_dir - up).magnitude() < 0.001 {
                // Already aligned with Y axis
                Mat4::identity()
            } else if (normalized_dir + up).magnitude() < 0.001 {
                // Pointing opposite to Y axis (180 degree rotation)
                Mat4::from_angle_x(radians(std::f32::consts::PI))
            } else {
                // General case: rotate from up vector to direction vector
                let axis = up.cross(normalized_dir).normalize();
                let angle = up.dot(normalized_dir).acos();
                Mat4::from_axis_angle(axis, radians(angle))
            };

            // Create thin cylinder
            let mut cylinder = Gm::new(
                Mesh::new(&context, &CpuMesh::cylinder(8)),
                PhysicalMaterial::new_opaque(
                    &context,
                    &CpuMaterial {
                        albedo: Srgba::new(150, 150, 150, 255), // Gray for connections
                        ..Default::default()
                    },
                ),
            );

            // Transform: translate to midpoint, rotate to align, then scale length
            let scale = Mat4::from_nonuniform_scale(0.05, length * 0.5, 0.05); // length * 0.5 because cylinder is unit height 2
            cylinder.set_transformation(Mat4::from_translation(midpoint) * rotation * scale);

            connection_meshes.push(cylinder);
        }
    }

    // Create lights
    let ambient = Rc::new(AmbientLight::new(&context, 0.5, Srgba::WHITE));
    let directional = Rc::new(DirectionalLight::new(
        &context,
        1.5,
        Srgba::WHITE,
        &vec3(-1.0, -1.0, -1.0),
    ));

    // Wrap meshes and data in Rc<RefCell> for render closure
    let node_meshes = Rc::new(RefCell::new(node_meshes));
    let connection_meshes = Rc::new(RefCell::new(connection_meshes));
    let grid_axes_meshes = Rc::new(grid_axes_meshes);
    let nodes_data = Rc::new(nodes_data);

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
            for mesh in grid_axes_meshes.iter() {
                target.render(&camera, mesh, &[]);
            }

            // Render connections (so they appear behind nodes)
            for conn in connection_meshes.borrow().iter() {
                target.render(&camera, conn, &[&*ambient, &*directional]);
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

    // Set up orbit controls with integrated click handler
    setup_orbit_controls(
        canvas,
        camera_state,
        render_scene.clone(),
        Some(nodes_data),
        selected_node_id_signal,
        selected_item_signal,
    )?;

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
        &vec3(-1.0, -1.0, -1.0),
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

    // Set up mouse drag for orbit (no node selection for test scene)
    setup_orbit_controls(canvas, camera_state, render_scene.clone(), None, selected_node_id_signal, selected_item_signal)?;

    Ok(())
}

/// Set up mouse and scroll event handlers for orbit controls
#[cfg(feature = "hydrate")]
fn setup_orbit_controls(
    canvas: &web_sys::HtmlCanvasElement,
    camera_state: RwSignal<CameraState>,
    render_scene: Rc<dyn Fn(CameraState)>,
    nodes_data: Option<Rc<Vec<NodeData>>>,
    selected_node_id_signal: RwSignal<Option<i64>>,
    selected_item_signal: RwSignal<Option<crate::islands::topology_editor::SelectedItem>>,
) -> Result<(), String> {
    use web_sys::{MouseEvent, WheelEvent};
    use three_d::*;

    let is_dragging = Rc::new(RefCell::new(false));
    let last_mouse_pos = Rc::new(RefCell::new((0.0, 0.0)));
    let mouse_down_pos = Rc::new(RefCell::new((0.0, 0.0))); // Track where mouse was pressed
    let total_mouse_movement = Rc::new(RefCell::new(0.0)); // Track total movement distance

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
        mousedown.forget();
    }

    // Mouse up - stop dragging OR handle click
    {
        let is_dragging = is_dragging.clone();
        let total_mouse_movement = total_mouse_movement.clone();
        let canvas_clone = canvas.clone();
        let nodes_data = nodes_data.clone(); // Clone for closure
        let selected_node_id_signal = selected_node_id_signal; // Copy Option<RwSignal>
        let selected_item_signal = selected_item_signal; // Copy Option<RwSignal>
        let render_scene = render_scene.clone(); // Clone for re-rendering on selection

        let mouseup = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |e: MouseEvent| {
            let was_dragging = *is_dragging.borrow();
            *is_dragging.borrow_mut() = false;
            canvas_clone.set_attribute("style", "cursor: pointer; border: 1px solid #ccc; display: block; background-color: #1a1a1a;").ok();

            // If total movement is very small, treat as a click for node selection
            let movement = *total_mouse_movement.borrow();
            if was_dragging && movement < 5.0 {
                // Perform node selection if we have the data
                if let Some(nodes) = nodes_data.as_ref()
                {
                    let rect = canvas_clone.get_bounding_client_rect();
                    let x = e.client_x() as f64 - rect.left();
                    let y = e.client_y() as f64 - rect.top();

                    // Convert to normalized device coordinates (-1 to 1)
                    let width = canvas_clone.client_width() as f64;
                    let height = canvas_clone.client_height() as f64;
                    let ndc_x = (x / width) * 2.0 - 1.0;
                    let ndc_y = 1.0 - (y / height) * 2.0;

                    // Get camera position and view direction
                    let state = camera_state.get_untracked();
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

                    // Update the selection signals
                    let selected_id = closest_node.map(|(id, _)| id);

                    selected_node_id_signal.set(selected_id);
                    selected_item_signal.set(selected_id.map(crate::islands::topology_editor::SelectedItem::Node));

                    // Trigger re-render immediately to show selection
                    render_scene(camera_state.get_untracked());
                }
            }
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref())
            .map_err(|e| format!("Failed to add mouseup listener: {:?}", e))?;
        mouseup.forget();
    }

    // Mouse move - rotate camera
    {
        let is_dragging = is_dragging.clone();
        let last_mouse_pos = last_mouse_pos.clone();
        let total_mouse_movement = total_mouse_movement.clone();
        let render_scene = render_scene.clone();

        let mousemove = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |e: MouseEvent| {
            if *is_dragging.borrow() {
                let current_pos = (e.client_x() as f32, e.client_y() as f32);
                let last_pos = *last_mouse_pos.borrow();

                let delta_x = current_pos.0 - last_pos.0;
                let delta_y = current_pos.1 - last_pos.1;

                // Track total movement distance
                let movement_dist = (delta_x * delta_x + delta_y * delta_y).sqrt();
                *total_mouse_movement.borrow_mut() += movement_dist;

                let mut state = camera_state.get_untracked();
                state.azimuth += delta_x * 0.01;
                state.elevation = (state.elevation - delta_y * 0.01).clamp(-1.5, 1.5);

                camera_state.set(state);
                render_scene(state);

                *last_mouse_pos.borrow_mut() = current_pos;
            }
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref())
            .map_err(|e| format!("Failed to add mousemove listener: {:?}", e))?;
        mousemove.forget();
    }

    // Mouse wheel - zoom
    {
        let render_scene = render_scene.clone();

        let wheel = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |e: WheelEvent| {
            e.prevent_default();

            let mut state = camera_state.get_untracked();
            state.distance = (state.distance + e.delta_y() as f32 * 0.01).clamp(2.0, 50.0);

            camera_state.set(state);
            render_scene(state);
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("wheel", wheel.as_ref().unchecked_ref())
            .map_err(|e| format!("Failed to add wheel listener: {:?}", e))?;
        wheel.forget();
    }

    Ok(())
}

/// Create grid floor and XYZ axes for spatial reference (Blender-style)
#[cfg(feature = "hydrate")]
fn create_grid_and_axes(context: &three_d::Context) -> Vec<three_d::Gm<three_d::Mesh, three_d::ColorMaterial>> {
    use three_d::*;

    let mut meshes = Vec::new();

    // Grid parameters (Blender convention: XY plane floor, Z is up)
    let grid_size = 10; // 10 units in each direction from origin
    let grid_spacing = 1.0; // 1 unit between lines
    let grid_z = 0.0; // Floor at Z=0 (Blender convention)
    let grid_line_thickness = 0.006; // Very thin lines
    let axis_line_thickness = 0.012; // Slightly thicker for axes

    // Create grid lines as thin cylinders on XY plane (Z=0)
    let grid_color = Srgba::new(50, 50, 50, 180); // Faint dark gray with transparency
    let cylinder_cpu_mesh = CpuMesh::cylinder(8); // 8-sided cylinder for lines

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

    // Create XYZ axis lines (span full grid extent in both directions)
    let axis_length = 15.0;

    // X axis (Red) - left to right on floor
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

    // Y axis (Green) - front to back on floor
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

    // Z axis (Blue) - vertical up/down
    if let Some(z_axis) = create_line_cylinder(
        context,
        vec3(0.0, 0.0, -axis_length),
        vec3(0.0, 0.0, axis_length),
        axis_line_thickness,
        Srgba::new(80, 160, 240, 200), // Faint light blue with transparency
        &cylinder_cpu_mesh,
    ) {
        meshes.push(z_axis);
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

