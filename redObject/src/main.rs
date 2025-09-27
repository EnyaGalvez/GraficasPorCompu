use std::{
    collections::{BTreeSet, HashSet},
    env,
    fs,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Default)]
struct ObjData {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    texcoords: Vec<[f32; 2]>,
    faces: Vec<String>,
    mtllibs: Vec<String>,
    materials_used: Vec<String>,
    materials_defined: BTreeSet<String>,
}

fn main() -> io::Result<()> {
    let arg = env::args().nth(1);
    let obj_path = match resolve_obj_path(arg.as_deref()) {
        Some(p) => p,
        None => {
           loop {
                eprintln!("No se proporcionó una ruta válida.");
                print!("Escribe la ruta o nombre del archivo .obj (Enter para reintentar): ");
                io::stdout().flush()?;
                let mut s = String::new();
                io::stdin().read_line(&mut s)?;
                let s = s.trim();
                if s.is_empty() {
                    continue;
                }
                if let Some(p) = resolve_obj_path(Some(s)) {
                    break p;
                } else {
                    eprintln!("No se encontró \"{}\" en las rutas probadas. Intenta de nuevo.\n", s);
                }
            } 
        }
    };

    let mut data = parse_obj(&obj_path)?;

    let base_dir = obj_path.parent().unwrap_or_else(|| Path::new("."));
    for mtl in &data.mtllibs {
        let mtl_path = base_dir.join(mtl);
        if let Ok(names) = parse_mtl(&mtl_path) {
            for n in names {
                data.materials_defined.insert(n);
            }
        }
    }

    println!("Vectores:");
    if data.vertices.is_empty() {
        println!("(ninguno)");
    } else {
        for (i, v) in data.vertices.iter().enumerate() {
            println!("- v[{i}]: ({:.6}, {:.6}, {:.6})", v[0], v[1], v[2]);
        }
    }

    println!("\nNormales:");
    if data.normals.is_empty() {
        println!("(ninguna)");
    } else {
        for (i, n) in data.normals.iter().enumerate() {
            println!("- vn[{i}]: ({:.6}, {:.6}, {:.6})", n[0], n[1], n[2]);
        }
    }

    println!("\nCoordenadas de textura:");
    if data.texcoords.is_empty() {
        println!("(ninguna)");
    } else {
        for (i, t) in data.texcoords.iter().enumerate() {
            println!("- vt[{i}]: ({:.6}, {:.6})", t[0], t[1]);
        }
    }

    println!("\nArchivos MTL referenciados:");
    if data.mtllibs.is_empty() {
        println!("(ninguno)");
    } else {
        for m in &data.mtllibs {
            println!("- {m}");
        }
    }

    println!("\nMateriales definidos en MTL:");
    if data.materials_defined.is_empty() {
        println!("(ninguno o no se pudo leer el .mtl)");
    } else {
        for name in &data.materials_defined {
            println!("- {name}");
        }
    }

    println!("\nCaras (formato crudo):");
    if data.faces.is_empty() {
        println!("(ninguna)");
    } else {
        for (i, f) in data.faces.iter().enumerate() {
            println!("- f[{i}]: {f}");
        }
    }

    println!("\n===== RESUMEN =====");
    println!("Cantidad de vértices: {}", data.vertices.len());
    println!("Cantidad de normales: {}", data.normals.len());
    println!("Cantidad de coordenadas de textura: {}", data.texcoords.len());
    println!("Cantidad de caras: {}", data.faces.len());
    println!("Cantidad de materiales definidos en MTL: {}", data.materials_defined.len());
    println!("Archivos MTL referenciados: {}", data.mtllibs.len());

    Ok(())
}

fn resolve_obj_path(input: Option<&str>) -> Option<PathBuf> {
    let s = input?;
    let candidate = PathBuf::from(s);
    if candidate.exists() && candidate.is_file() {
        return Some(candidate);
    }

    if let Ok(cwd) = env::current_dir() {
        let c1 = cwd.join(s);
        if c1.exists() && c1.is_file() {
            return Some(c1);
        }
        let c2 = cwd.join("src").join(s);
        if c2.exists() && c2.is_file() {
            return Some(c2);
        }
    }
    None
}

fn parse_obj(path: &Path) -> io::Result<ObjData> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut data = ObjData::default();
    let mut seen_mtllib = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut it = trimmed.split_whitespace();
        let tag = it.next().unwrap_or_default();

        match tag {
            "v" => {
                let vals: Vec<f32> = it.filter_map(|s| s.parse::<f32>().ok()).collect();
                if vals.len() >= 3 {
                    data.vertices.push([vals[0], vals[1], vals[2]]);
                }
            }
            "vn" => {
                let vals: Vec<f32> = it.filter_map(|s| s.parse::<f32>().ok()).collect();
                if vals.len() >= 3 {
                    data.normals.push([vals[0], vals[1], vals[2]]);
                }
            }
            "vt" => {
                let vals: Vec<f32> = it.filter_map(|s| s.parse::<f32>().ok()).collect();
                if vals.len() >= 2 {
                    data.texcoords.push([vals[0], vals[1]]);
                }
            }
            "f" => {
                data.faces.push(trimmed.to_string());
            }
            "usemtl" => {
                if let Some(name) = it.next() {
                    data.materials_used.push(name.to_string());
                }
            }
            "mtllib" => {
                for name in it {
                    if seen_mtllib.insert(name.to_string()) {
                        data.mtllibs.push(name.to_string());
                    }
                }
            }
            _ => {
                // se ignoran otras etiquetas
            }
        }
    }
    Ok(data)
}

fn parse_mtl(path: &Path) -> io::Result<BTreeSet<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut names = BTreeSet::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut it = trimmed.split_whitespace();
        if let Some(tag) = it.next() {
            if tag == "newmtl" {
                if let Some(name) = it.next() {
                    names.insert(name.to_string());
                }
            }
        }
    }
    Ok(names)
}
