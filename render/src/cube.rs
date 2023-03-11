use glium::{
    backend::Facade,
    implement_vertex,
    index::{IndexBuffer, PrimitiveType},
    VertexBuffer,
};

#[derive(Copy, Clone)]
pub struct CubeVertex {
    position: [f32; 3],
    color: [f32; 4],
}

implement_vertex!(CubeVertex, position, color);

/// Generate a cube VBO and IBO.
pub fn gen_cube<F: Facade>(render: &F) -> (VertexBuffer<CubeVertex>, IndexBuffer<u8>) {
    let vertices = [
        (0.0, 1.0, 0.0),
        (0.0, 1.0, 1.0),
        (1.0, 1.0, 0.0),
        (1.0, 1.0, 1.0),
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 1.0),
        (1.0, 0.0, 0.0),
        (1.0, 0.0, 1.0),
    ]
    .map(|tuple| CubeVertex {
        position: [tuple.0, tuple.1, tuple.2],
        color: [0.0, 0.0, 1.0, 1.0],
    });

    let indices = [
        0u8, 1, 2, 1, 2, 3, 4, 5, 6, 5, 6, 7, 0, 1, 5, 0, 4, 5, 2, 3, 7, 2, 6, 7, 0, 2, 6, 0, 4, 6,
        1, 5, 7, 1, 3, 7,
    ];

    let vbo = VertexBuffer::new(render, &vertices).unwrap();
    let ibo = IndexBuffer::new(render, PrimitiveType::TriangleStrip, &indices).unwrap();

    (vbo, ibo)
}
