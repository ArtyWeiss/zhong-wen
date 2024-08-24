use macroquad::prelude::*;
use macroquad_text::Fonts;

#[macroquad::main("ZHONG-WEN")]
async fn main() {
    let zhong = create_card_mesh("zhong.png").await;
    let wen = create_card_mesh("wen.png").await;

    let mut fonts = Fonts::default();
    fonts
        .load_font_from_bytes_with_scale("Itim", include_bytes!("../font/Itim.ttf"), 10.0)
        .unwrap();

    loop {
        clear_background(DARKGRAY);
        set_camera(&Camera3D {
            position: vec3(0.0, 0.0, -10.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            fovy: 50.0,
            projection: Projection::Perspective,
            ..Default::default()
        });

        let gl = unsafe { get_internal_gl().quad_gl };
        draw_card(gl, vec3(-0.5, 0.0, 0.0), mouse_position().0, &zhong);
        draw_card(gl, vec3(0.5, 0.0, 0.0), mouse_position().1, &wen);

        set_default_camera();
        let w = screen_width();
        let half_w = w / 2.0;
        fonts.draw_text("zhōng", half_w - 180.0, 100.0, 50, BLACK);
        fonts.draw_text("wén", half_w + 60.0, 100.0, 50, BLACK);

        next_frame().await
    }
}

async fn create_card_mesh(texture: &str) -> Mesh {
    let texture: Texture2D = load_texture(&format!("textures/{}", texture))
        .await
        .unwrap();
    Mesh {
        vertices: vec![
            Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0, WHITE),
            Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0, WHITE),
            Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0, WHITE),
            Vertex::new(-0.5, 0.5, 0.0, 0.0, 1.0, WHITE),
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
        texture: Some(texture),
    }
}

fn draw_card(gl: &mut QuadGl, pos: Vec3, rotation_y: f32, mesh: &Mesh) {
    gl.push_model_matrix(Mat4::from_scale_rotation_translation(
        vec3(1.0, 1.0, 1.0),
        Quat::from_rotation_y(rotation_y.to_radians()),
        pos,
    ));
    draw_mesh(mesh);
    gl.pop_model_matrix();
}
