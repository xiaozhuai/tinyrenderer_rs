use crate::{Vec2, Vec3};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub struct Model {
    pub verts: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub norms: Vec<Vec3>,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ModelError {
    IoError(std::io::Error),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
    SyntaxError,
}

impl From<std::io::Error> for ModelError {
    fn from(error: std::io::Error) -> Self {
        ModelError::IoError(error)
    }
}

impl From<std::num::ParseFloatError> for ModelError {
    fn from(error: std::num::ParseFloatError) -> Self {
        ModelError::ParseFloatError(error)
    }
}

impl From<std::num::ParseIntError> for ModelError {
    fn from(error: std::num::ParseIntError) -> Self {
        ModelError::ParseIntError(error)
    }
}

impl Model {
    pub fn load(filepath: impl AsRef<Path>) -> Result<Self, ModelError> {
        let file = File::open(filepath)?;
        let lines = std::io::BufReader::new(file).lines();
        let mut verts: Vec<Vec3> = Vec::new();
        let mut norms: Vec<Vec3> = Vec::new();
        let mut uvs: Vec<Vec2> = Vec::new();
        let mut v_faces: Vec<Vec<usize>> = Vec::new();
        let mut vn_faces: Vec<Vec<usize>> = Vec::new();
        let mut vt_faces: Vec<Vec<usize>> = Vec::new();
        for line in lines.flatten() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            } else if line.starts_with("v ") {
                let v = line
                    .split_whitespace()
                    .skip(1)
                    .map(|x| x.parse::<f32>())
                    .collect::<Result<Vec<f32>, std::num::ParseFloatError>>()?;
                if v.len() != 3 {
                    return Err(ModelError::SyntaxError);
                }
                verts.push(Vec3::new(v[0], v[1], v[2]));
            } else if line.starts_with("vt ") {
                let vt = line
                    .split_whitespace()
                    .skip(1)
                    .map(|x| x.parse::<f32>())
                    .collect::<Result<Vec<f32>, std::num::ParseFloatError>>()?;
                if vt.len() != 2 && vt.len() != 3 {
                    return Err(ModelError::SyntaxError);
                }
                uvs.push(Vec2::new(vt[0], vt[1]));
            } else if line.starts_with("vn ") {
                let vn = line
                    .split_whitespace()
                    .skip(1)
                    .map(|x| x.parse::<f32>())
                    .collect::<Result<Vec<f32>, std::num::ParseFloatError>>()?;
                if vn.len() != 3 {
                    return Err(ModelError::SyntaxError);
                }
                norms.push(Vec3::new(vn[0], vn[1], vn[2]));
            } else if line.starts_with("f ") {
                let points = line
                    .split_whitespace()
                    .skip(1)
                    .map(|point| {
                        let indices = point
                            .split('/')
                            .map(|x| x.parse::<usize>())
                            .collect::<Result<Vec<usize>, std::num::ParseIntError>>();
                        match indices {
                            Ok(indices) => {
                                if indices.len() == 3 {
                                    Ok(indices)
                                } else {
                                    Err(ModelError::SyntaxError)
                                }
                            }
                            Err(err) => Err(ModelError::ParseIntError(err)),
                        }
                    })
                    .collect::<Result<Vec<Vec<usize>>, ModelError>>()?;
                let mut v_face: Vec<usize> = Vec::new();
                let mut vt_face: Vec<usize> = Vec::new();
                let mut vn_face: Vec<usize> = Vec::new();

                if points.len() == 3 {
                    v_face.push(points[0][0] - 1);
                    vt_face.push(points[0][1] - 1);
                    vn_face.push(points[0][2] - 1);
                    v_face.push(points[1][0] - 1);
                    vt_face.push(points[1][1] - 1);
                    vn_face.push(points[1][2] - 1);
                    v_face.push(points[2][0] - 1);
                    vt_face.push(points[2][1] - 1);
                    vn_face.push(points[2][2] - 1);
                } else if points.len() == 4 {
                    v_face.push(points[0][0] - 1);
                    vt_face.push(points[0][1] - 1);
                    vn_face.push(points[0][2] - 1);
                    v_face.push(points[1][0] - 1);
                    vt_face.push(points[1][1] - 1);
                    vn_face.push(points[1][2] - 1);
                    v_face.push(points[2][0] - 1);
                    vt_face.push(points[2][1] - 1);
                    vn_face.push(points[2][2] - 1);

                    v_face.push(points[2][0] - 1);
                    vt_face.push(points[2][1] - 1);
                    vn_face.push(points[2][2] - 1);
                    v_face.push(points[1][0] - 1);
                    vt_face.push(points[1][1] - 1);
                    vn_face.push(points[1][2] - 1);
                    v_face.push(points[3][0] - 1);
                    vt_face.push(points[3][1] - 1);
                    vn_face.push(points[3][2] - 1);
                } else {
                    return Err(ModelError::SyntaxError);
                }
                v_faces.push(v_face);
                vt_faces.push(vt_face);
                vn_faces.push(vn_face);
            } else {
                // TODO(xiaozhuai)
                continue;
            }
        }
        Ok(Model {
            verts: Self::all_verts(&verts, &v_faces),
            uvs: Self::all_uvs(&uvs, &vt_faces),
            norms: Self::all_norms(&norms, &vn_faces),
        })
    }

    fn vert<'a>(
        verts: &'a [Vec3],
        v_faces: &'a [Vec<usize>],
        face_index: usize,
        index: usize,
    ) -> &'a Vec3 {
        &verts[v_faces[face_index][index]]
    }

    fn uv<'a>(
        uvs: &'a [Vec2],
        vt_faces: &'a [Vec<usize>],
        face_index: usize,
        index: usize,
    ) -> &'a Vec2 {
        &uvs[vt_faces[face_index][index]]
    }

    fn norm<'a>(
        norms: &'a [Vec3],
        vn_faces: &'a [Vec<usize>],
        face_index: usize,
        index: usize,
    ) -> &'a Vec3 {
        &norms[vn_faces[face_index][index]]
    }

    fn all_verts(verts: &[Vec3], v_faces: &[Vec<usize>]) -> Vec<Vec3> {
        let mut data: Vec<Vec3> = Vec::new();
        for face_index in 0..(v_faces.len()) {
            data.push(*Self::vert(verts, v_faces, face_index, 0));
            data.push(*Self::vert(verts, v_faces, face_index, 1));
            data.push(*Self::vert(verts, v_faces, face_index, 2));
        }
        data
    }

    fn all_uvs(uvs: &[Vec2], vt_faces: &[Vec<usize>]) -> Vec<Vec2> {
        let mut data: Vec<Vec2> = Vec::new();
        for face_index in 0..(vt_faces.len()) {
            data.push(*Self::uv(uvs, vt_faces, face_index, 0));
            data.push(*Self::uv(uvs, vt_faces, face_index, 1));
            data.push(*Self::uv(uvs, vt_faces, face_index, 2));
        }
        data
    }

    fn all_norms(norms: &[Vec3], vn_faces: &[Vec<usize>]) -> Vec<Vec3> {
        let mut data: Vec<Vec3> = Vec::new();
        for face_index in 0..(vn_faces.len()) {
            data.push(*Self::norm(norms, vn_faces, face_index, 0));
            data.push(*Self::norm(norms, vn_faces, face_index, 1));
            data.push(*Self::norm(norms, vn_faces, face_index, 2));
        }
        data
    }
}
