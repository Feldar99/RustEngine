// build script
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn swizzle_functions_str (vector_types: Vec<&'static str>, components: Vec<char>) -> String {
    let max_dim = components.len();
    let mut result = String::new();
    for i in 0..max_dim {
        let vector_type = vector_types[i];
        let dim = i + 1;
        let mut indices = Vec::<isize>::new();
        indices.reserve(max_dim);
        result += &format!("impl<T: num::Num + Copy> {}<T> {{\n", vector_type);
        for _ in 0..max_dim {
            indices.push(-1);
        }
        loop {
            let mut function_name = String::new();

            let mut done = false;
            // set up indices for next function
            for j in 0..max_dim {
                if indices[j] < (dim as isize) - 1 {
                    indices[j] += 1;
                    break;
                }
                indices[j] = 0;
                if j == max_dim -1 {
                    done = true;
                }
            }

            if done {
                break;
            }

            // compute function name
            for j in 0..max_dim {
                if indices[j] < 0 {
                    break;
                }
                function_name.push(components[indices[j] as usize]);
            }

            let return_type = vector_types[function_name.len() - 1];

            result += &format!("pub fn {} (&self) -> {}<T> {{\n", function_name, return_type);
            result += &format!("    {}{{", return_type);
            let mut first = true;
            for j in 0..function_name.len() {
                if first {
                    first = false;
                }
                else {
                    result += ", ";
                }
                result += &format!("{}: self.{}", components[j], components[indices[j] as usize]);
            }
            result += "}\n";
            result += "}\n";
        }
        result += "}\n";
    }
    result
}

fn main () {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("swizzle_gen.rs");
    let mut file = File::create(path).expect("Cannot create swizzle file");
    file.write_all(swizzle_functions_str(vec!("Vec1", "Vec2", "Vec3", "Vec4"), vec!('x', 'y', 'z', 'w')).as_bytes()).expect("Cannot write to swizzle file");
}
