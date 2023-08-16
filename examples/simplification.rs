use std::path::Path;

use baby_shark::{
    decimation::{edge_decimation::ConstantMaxError, prelude::EdgeDecimator},
    io::stl::{StlReader, StlWriter},
    mesh::corner_table::prelude::CornerTableF,
};

fn main() {
    let mut reader = StlReader::new();
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("Enter an input file");
    let output = args.next().expect("Enter an output file");
    let error = args.next().expect("Enter a float error");
    let error: f32 = error.parse().expect("Error converting cmdline error value");

    let mut mesh: CornerTableF = reader
        .read_stl_from_file(Path::new(&path))
        .expect("Read mesh from STL");

    let max_error = ConstantMaxError::new(error);

    let mut decimator = EdgeDecimator::new().max_error(max_error);
    decimator.decimate(&mut mesh);

    let writer = StlWriter::new();
    writer
        .write_stl_to_file(&mesh, Path::new(&output))
        .expect("Save mesh to STL");
}
