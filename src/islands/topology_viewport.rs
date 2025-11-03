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
            distance: 18.0,    // Increased from 8.0 to show full topology
            azimuth: 0.785,    // ~45 degrees
            elevation: 0.785,  // ~45 degrees
        }
    }
}

/// 3D Network Topology Viewport using three-d rendering
///
/// NOTE: #[lazy] code splitting doesn't work with complex reactive islands that use Effects
/// Islands still provide on-demand loading, just not separate WASM bundles
#[island]
pub fn TopologyViewport(
    /// Optional topology ID to load and display
    #[prop(optional)]
    topology_id: Option<i64>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let error_signal = RwSignal::new(None::<String>);
    let is_initialized = RwSignal::new(false);

    // Camera state as signals for reactivity (client-side only)
    #[cfg(feature = "hydrate")]
    let camera_state = RwSignal::new(CameraState::default());

    // Fetch topology data if topology_id is provided
    let topology_data = Resource::new(
        move || topology_id,
        |id| async move {
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

    // Initialize three-d viewport when canvas mounts or data loads
    Effect::new(move || {
        #[allow(unused_variables)]
        if let Some(canvas_element) = canvas_ref.get() {
            #[cfg(feature = "hydrate")]
            {
                // Access topology_data to make Effect reactive to it
                let data_option = topology_data.get();

                // Log to browser console
                web_sys::console::log_1(&format!("Effect running: topology_id={:?}, data_loaded={}",
                    topology_id, data_option.is_some()).into());

                // Wait for topology data to load
                if let Some(Some(topo_data)) = data_option {
                    web_sys::console::log_1(&format!("Rendering topology with {} nodes, {} connections",
                        topo_data.nodes.len(), topo_data.connections.len()).into());

                    match initialize_threed_viewport(&canvas_element, camera_state, &topo_data) {
                        Ok(_) => {
                            is_initialized.set(true);
                            web_sys::console::log_1(&"✅ 3D Viewport initialized with topology data".into());
                        }
                        Err(e) => {
                            error_signal.set(Some(e.clone()));
                            web_sys::console::error_1(&format!("❌ Failed to initialize: {}", e).into());
                        }
                    }
                } else if topology_id.is_none() {
                    // Initialize with test scene if no topology_id
                    web_sys::console::log_1(&"Rendering test cube (no topology_id)".into());
                    match initialize_threed_viewport_test(&canvas_element, camera_state) {
                        Ok(_) => {
                            is_initialized.set(true);
                            web_sys::console::log_1(&"✅ 3D Viewport initialized with test scene".into());
                        }
                        Err(e) => {
                            error_signal.set(Some(e.clone()));
                            web_sys::console::error_1(&format!("❌ Failed to initialize: {}", e).into());
                        }
                    }
                } else {
                    web_sys::console::log_1(&"⏳ Waiting for topology data to load...".into());
                }
            }
        }
    });

    view! {
        <div class="topology-viewport-container">
            <canvas
                node_ref=canvas_ref
                width="800"
                height="600"
                style="border: 1px solid #ccc; display: block; background-color: #1a1a1a; cursor: grab;"
            />

            {move || {
                if let Some(err) = error_signal.get() {
                    view! {
                        <div class="error" style="color: red; margin-top: 10px;">
                            "Error: " {err}
                        </div>
                    }.into_any()
                } else if is_initialized.get() {
                    view! {
                        <div class="info" style="color: green; margin-top: 10px;">
                            "✅ 3D Viewport - Drag to rotate, scroll to zoom"
                            {topology_id.map(|id| format!(" - Topology ID: {}", id)).unwrap_or_default()}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="info" style="color: gray; margin-top: 10px;">
                            "Initializing 3D viewport..."
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Initialize three-d Context with topology data
#[cfg(feature = "hydrate")]
fn initialize_threed_viewport(
    canvas: &web_sys::HtmlCanvasElement,
    camera_state: RwSignal<CameraState>,
    topology_data: &crate::models::TopologyFull,
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

    // Create node meshes (spheres at x/y/z positions)
    let mut node_meshes = Vec::new();
    let mut node_positions = HashMap::new();

    for node in &topology_data.nodes {
        let position = vec3(
            node.position_x as f32,
            node.position_y as f32,
            node.position_z as f32,
        );
        node_positions.insert(node.id, position);

        // Create sphere for each node
        let mut sphere = Gm::new(
            Mesh::new(&context, &CpuMesh::sphere(16)),
            PhysicalMaterial::new_opaque(
                &context,
                &CpuMaterial {
                    albedo: Srgba::new(50, 150, 255, 255), // Blue for nodes
                    ..Default::default()
                },
            ),
        );

        // Position and scale the sphere (smaller size: 0.3)
        sphere.set_transformation(
            Mat4::from_translation(position) * Mat4::from_scale(0.3)
        );

        node_meshes.push(sphere);
    }

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

    // Wrap meshes in Rc<RefCell> for render closure
    let node_meshes = Rc::new(RefCell::new(node_meshes));
    let connection_meshes = Rc::new(RefCell::new(connection_meshes));

    // Render function
    let render_scene = {
        let context = context.clone();
        let node_meshes = node_meshes.clone();
        let connection_meshes = connection_meshes.clone();
        let ambient = ambient.clone();
        let directional = directional.clone();

        move |state: CameraState| {
            let viewport = Viewport::new_at_origo(800, 600);

            // Calculate camera position from spherical coordinates
            let eye = vec3(
                state.distance * state.elevation.cos() * state.azimuth.sin(),
                state.distance * state.elevation.sin(),
                state.distance * state.elevation.cos() * state.azimuth.cos(),
            );

            let camera = Camera::new_perspective(
                viewport,
                eye,
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                degrees(45.0),
                0.1,
                1000.0,
            );

            let clear_state = ClearState::color_and_depth(0.1, 0.1, 0.1, 1.0, 1.0);
            let target = RenderTarget::screen(&context, 800, 600);
            target.clear(clear_state);

            // Render connections first (so they appear behind nodes)
            for conn in connection_meshes.borrow().iter() {
                target.render(&camera, conn, &[&*ambient, &*directional]);
            }

            // Render nodes
            for node in node_meshes.borrow().iter() {
                target.render(&camera, node, &[&*ambient, &*directional]);
            }
        }
    };

    // Initial render
    render_scene(camera_state.get_untracked());

    // Set up orbit controls
    setup_orbit_controls(canvas, camera_state, render_scene)?;

    web_sys::console::log_1(&format!("✅ Topology viewport ready with {} nodes and {} connections",
        topology_data.nodes.len(), topology_data.connections.len()).into());
    Ok(())
}

/// Initialize three-d Context with test cube (fallback when no topology data)
#[cfg(feature = "hydrate")]
fn initialize_threed_viewport_test(
    canvas: &web_sys::HtmlCanvasElement,
    camera_state: RwSignal<CameraState>,
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

        move |state: CameraState| {
            let viewport = Viewport::new_at_origo(800, 600);

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

            RenderTarget::screen(&context, 800, 600)
                .clear(clear_state)
                .render(&camera, &*cube.borrow(), &[&*ambient, &*directional]);
        }
    };

    // Initial render
    render_scene(camera_state.get_untracked());

    // Set up mouse drag for orbit
    setup_orbit_controls(canvas, camera_state, render_scene)?;

    web_sys::console::log_1(&"✅ Interactive 3D viewport ready".into());
    Ok(())
}

/// Set up mouse and scroll event handlers for orbit controls
#[cfg(feature = "hydrate")]
fn setup_orbit_controls(
    canvas: &web_sys::HtmlCanvasElement,
    camera_state: RwSignal<CameraState>,
    render_scene: impl Fn(CameraState) + 'static,
) -> Result<(), String> {
    use web_sys::{MouseEvent, WheelEvent};

    let is_dragging = Rc::new(RefCell::new(false));
    let last_mouse_pos = Rc::new(RefCell::new((0.0, 0.0)));
    let render_scene = Rc::new(render_scene);

    // Mouse down - start dragging
    {
        let is_dragging = is_dragging.clone();
        let last_mouse_pos = last_mouse_pos.clone();
        let canvas_clone = canvas.clone();

        let mousedown = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |e: MouseEvent| {
            *is_dragging.borrow_mut() = true;
            *last_mouse_pos.borrow_mut() = (e.client_x() as f32, e.client_y() as f32);
            canvas_clone.set_attribute("style", "cursor: grabbing; border: 1px solid #ccc; display: block; background-color: #1a1a1a;").ok();
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref())
            .map_err(|e| format!("Failed to add mousedown listener: {:?}", e))?;
        mousedown.forget();
    }

    // Mouse up - stop dragging
    {
        let is_dragging = is_dragging.clone();
        let canvas_clone = canvas.clone();

        let mouseup = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: MouseEvent| {
            *is_dragging.borrow_mut() = false;
            canvas_clone.set_attribute("style", "cursor: grab; border: 1px solid #ccc; display: block; background-color: #1a1a1a;").ok();
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
        let render_scene = render_scene.clone();

        let mousemove = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |e: MouseEvent| {
            if *is_dragging.borrow() {
                let current_pos = (e.client_x() as f32, e.client_y() as f32);
                let last_pos = *last_mouse_pos.borrow();

                let delta_x = current_pos.0 - last_pos.0;
                let delta_y = current_pos.1 - last_pos.1;

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
