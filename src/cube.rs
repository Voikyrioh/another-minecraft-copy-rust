pub const BASIC_VERTEXES: [[f32; 3]; 6] = [
    [-0.5,  -0.5, 0.5],
    [-0.5,   0.5, 0.5],
    [ 0.5,   0.5, 0.5],
    [ 0.5,   0.5, 0.5],
    [ 0.5,  -0.5, 0.5],
    [-0.5,  -0.5, 0.5],
];

pub const BASIC_FRACTIONS: [f32; 5] = [
    1f32/2f32,
    1f32/3f32,
    2f32/3f32,
    1f32/4f32,
    3f32/4f32,
];

pub struct Face {
    uv: [Vec<f32>; 6],
    normal: Vec<i8>,
    vertexes: [Vec<f32>; 6],
    obfuscated: bool,
}

pub struct Cube {
    coord: Vec<i32>,
    faces: [Face; 6],
}

impl Cube {
    pub fn new(coord: Vec<i32>) -> Cube {
        let faces = [
            Face {
                uv: [
                    vec![BASIC_FRACTIONS[3], BASIC_FRACTIONS[2]],
                    vec![BASIC_FRACTIONS[3], BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[0], BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[0], BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[0], BASIC_FRACTIONS[2]],
                    vec![BASIC_FRACTIONS[3], BASIC_FRACTIONS[2]]
                ],
                normal: vec![0, 0, 1],
                vertexes: BASIC_VERTEXES.map(|v| { vec!(-v[0], v[1], v[2]) }),
                obfuscated: false
            },
            Face {
                uv: [
                    vec![BASIC_FRACTIONS[3], 1f32],
                    vec![BASIC_FRACTIONS[3], BASIC_FRACTIONS[2]],
                    vec![BASIC_FRACTIONS[0], BASIC_FRACTIONS[2]],
                    vec![BASIC_FRACTIONS[0], BASIC_FRACTIONS[2]],
                    vec![BASIC_FRACTIONS[0], 1f32],
                    vec![BASIC_FRACTIONS[3], 1f32]
                ],
                normal: vec![0, 0, -1],
                vertexes: BASIC_VERTEXES.map(|v| { vec!(v[0], -v[2], -v[1]) }),
                obfuscated: false
            },
            Face {
                uv: [
                    vec![BASIC_FRACTIONS[0], BASIC_FRACTIONS[2]],
                    vec![BASIC_FRACTIONS[0], BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[4], BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[4], BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[4], BASIC_FRACTIONS[2]],
                    vec![BASIC_FRACTIONS[0], BASIC_FRACTIONS[2]]
                ],
                normal: vec![1, 0, 0],
                vertexes: BASIC_VERTEXES.map(|v| { vec!(v[2], v[1], v[0]) }),
                obfuscated: false
            },
            Face {
                uv: [
                    vec![0f32, BASIC_FRACTIONS[2]],
                    vec![0f32, BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[3], BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[3], BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[3], BASIC_FRACTIONS[2]],
                    vec![0f32, BASIC_FRACTIONS[2]]
                ],
                normal: vec![-1, 0, 0],
                vertexes: BASIC_VERTEXES.map(|v| { vec!(-v[2], v[1], -v[0]) }),
                obfuscated: false
            },
            Face {
                uv: [
                    vec![BASIC_FRACTIONS[3], BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[3], 0f32],
                    vec![BASIC_FRACTIONS[0], 0f32],
                    vec![BASIC_FRACTIONS[0], 0f32],
                    vec![BASIC_FRACTIONS[0], BASIC_FRACTIONS[1]],
                    vec![BASIC_FRACTIONS[3], BASIC_FRACTIONS[1]]
                ],
                normal: vec![0, 1, 0],
                vertexes: BASIC_VERTEXES.map(|v| { vec!(v[0], v[2], v[1]) }),
                obfuscated: false
            },
            Face {
                uv: [
                    vec![BASIC_FRACTIONS[4], BASIC_FRACTIONS[2]],
                    vec![BASIC_FRACTIONS[4], BASIC_FRACTIONS[1]],
                    vec![1f32, BASIC_FRACTIONS[1]],
                    vec![1f32, BASIC_FRACTIONS[1]],
                    vec![1f32, BASIC_FRACTIONS[2]],
                    vec![BASIC_FRACTIONS[4], BASIC_FRACTIONS[2]]
                ],
                normal: vec![0, -1, 0],
                vertexes: BASIC_VERTEXES.map(|v| { vec!(v[0], v[1], -v[2]) }),
                obfuscated: false
            },
        ];

        Self {
            coord,
            faces
        }
    }

    fn to_vertex(&self) -> [[f32; 8*6]; 6] {
        let mut result: [[f32; 8 * 6]; 6] = [[0.0; 8 * 6]; 6];

        for (face_index, f) in self.faces.iter().enumerate() {
            for i in 0..6 {
                let combined: Vec<f32> = f.vertexes[i]
                    .iter()
                    .copied() // Copie les valeurs des vertexes
                    .chain(f.uv[i].iter().copied()) // Combine les coordonnées UV
                    .collect();

                // Convertir le vecteur combiné en tableau [f32; 8]
                result[face_index][i * 8..(i + 1) * 8]
                    .copy_from_slice(combined.as_slice());
            }
        }

        result
    }
}
