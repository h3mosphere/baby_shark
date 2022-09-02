use nalgebra::Point3;
use crate::mesh::traits::Mesh;
use super::{prelude::CornerTableF, connectivity::{corner::DefaultCorner, vertex::VertexF}};

pub fn create_unit_square_mesh() -> CornerTableF {
    let vertices = vec![
        Point3::<f32>::new(0.0, 1.0, 0.0),
        Point3::<f32>::new(0.0, 0.0, 0.0),
        Point3::<f32>::new(1.0, 0.0, 0.0),
        Point3::<f32>::new(1.0, 1.0, 0.0)
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    return CornerTableF::from_vertices_and_indices(&vertices, &indices);
}

pub fn create_unit_cross_square_mesh() -> CornerTableF {
    let vertices = vec![
        Point3::<f32>::new(0.0, 1.0, 0.0),
        Point3::<f32>::new(0.0, 0.0, 0.0),
        Point3::<f32>::new(1.0, 0.0, 0.0),
        Point3::<f32>::new(1.0, 1.0, 0.0),
        Point3::<f32>::new(0.5, 0.5, 0.0)
    ];

    let indices = vec![
        0, 1, 4, 
        1, 2, 4, 
        2, 3, 4, 
        3, 0, 4
    ];

    return CornerTableF::from_vertices_and_indices(&vertices, &indices);
}

pub fn create_single_face_mesh() -> CornerTableF {
    let vertices = vec![
        Point3::<f32>::new(0.0, 1.0, 0.0),
        Point3::<f32>::new(0.0, 0.0, 0.0),
        Point3::<f32>::new(1.0, 0.0, 0.0)
    ];

    let indices = vec![0, 1, 2];

    return CornerTableF::from_vertices_and_indices(&vertices, &indices);
}

pub fn create_collapse_edge_sample_mesh() -> CornerTableF {
    let vertices = vec![
        Point3::<f32>::new(0.0, 1.0, 0.0),
        Point3::<f32>::new(0.0, 0.5, 0.0),
        Point3::<f32>::new(0.0, 0.0, 0.0),
        Point3::<f32>::new(0.5, 0.0, 0.0),
        Point3::<f32>::new(1.0, 0.0, 0.0),
        Point3::<f32>::new(1.0, 0.5, 0.0),
        Point3::<f32>::new(1.0, 1.0, 0.0),
        Point3::<f32>::new(0.5, 1.0, 0.0),
        Point3::<f32>::new(0.25, 0.5, 0.0),
        Point3::<f32>::new(0.75, 0.5, 0.0)
    ];

    let indices = vec![
        0, 1, 8,
        1, 2, 8,
        2, 3, 8,
        3, 9, 8,
        3, 4, 9,
        4, 5, 9,
        5, 6, 9,
        6, 7, 9,
        7, 8, 9,
        7, 0, 8
    ];

    return CornerTableF::from_vertices_and_indices(&vertices, &indices);
}

pub fn assert_mesh_equals(mesh: &CornerTableF, expected_corners: &Vec<DefaultCorner>, expected_vertices: &Vec<VertexF>) {
    // Assert equality for each element separately for readability

    assert_eq!(expected_vertices.len(), mesh.vertices.len());
    assert_eq!(expected_corners.len(), mesh.corners.len());

    for i in 0..expected_vertices.len() {
        assert_eq!(expected_vertices[i], mesh.vertices[i]);
    }

    for i in 0..expected_corners.len() {
        assert_eq!(expected_corners[i], mesh.corners[i]);
    }
}
