use js_sys::Float32Array;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram, WebGlShader};

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
    let context = canvas.get_context("webgl2")?.unwrap().dyn_into::<GL>()?;

    let width = canvas.width();
    let height = canvas.height();

    context.viewport(0, 0, width as i32, height as i32);

    let v_shader = compile_shader(
        &context,
        GL::VERTEX_SHADER,
        r##"#version 300 es
        in vec2 position;
        uniform vec2 resolution;

        void main() {
            vec2 zeroToOne = position / resolution;
            vec2 zeroToTwo = zeroToOne * 2.0;
            vec2 clipSpace = zeroToTwo - 1.0;

            gl_Position = vec4(clipSpace, 0, 1);
        }
        "##,
    )?;

    let f_shader = compile_shader(
        &context,
        GL::FRAGMENT_SHADER,
        r##"#version 300 es
        precision highp float;
        out vec4 color;

        void main() {
            color = vec4(0, 1, 1, 1);
        }
        "##,
    )?;

    let program = link_program(&context, &v_shader, &f_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 6] = [100.0, 100.0, 300.0, 100.0, 200.0, 200.0];

    let pos_location = context.get_attrib_location(&program, "position") as u32;
    let res_location = context.get_uniform_location(&program, "resolution");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let pos_view = Float32Array::view(&vertices);
        context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &pos_view, GL::STATIC_DRAW);
    }

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(pos_location, 2, GL::FLOAT, false, 0, 0);
    context.uniform2f(res_location.as_ref(), width as f32, height as f32);
    context.enable_vertex_attrib_array(pos_location);
    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 2) as i32;
    draw(&context, vert_count);

    Ok(())
}

fn draw(context: &GL, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(GL::COLOR_BUFFER_BIT);
    context.draw_arrays(GL::TRIANGLES, 0, vert_count);
}

fn compile_shader(context: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let log = context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"));
        Err(log)
    }
}

fn link_program(
    context: &GL,
    v_shader: &WebGlShader,
    f_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create program object"))?;

    context.attach_shader(&program, v_shader);
    context.attach_shader(&program, f_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let log = context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program"));
        Err(log)
    }
}
