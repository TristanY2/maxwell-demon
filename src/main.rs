use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

macro_rules! Insert {
    ($hash: expr, $list: expr, $path: expr) => {{
        for ext in $list {
            $hash.insert(ext, $path);
        }
    }};
}

// no sobreescribir un archivo que ya existe
fn buscar_nombre(ruta_inicial: &Path) -> PathBuf {
    if !ruta_inicial.exists() {
        return ruta_inicial.to_path_buf();
    }

    let extension = ruta_inicial.extension().and_then(|s| s.to_str());
    let nombre_completo = ruta_inicial
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("archivo_invalido_papu");

    let nombre_completo = limpiar_nombre(nombre_completo);
    let mut contador = 1;
    loop {
        let nombre_final = match extension {
            Some(ext) => format!("{}({}).{}", nombre_completo, contador, ext),
            None => format!("{}({})", nombre_completo, contador),
        };
        let nueva_ruta = ruta_inicial.with_file_name(nombre_final);
        if !nueva_ruta.exists() {
            return nueva_ruta;
        }
        contador += 1;
    }
}

//no crear un prueba(1)(1)
fn limpiar_nombre(nombre: &str) -> &str {
    if let Some(a_parentesis) = nombre.rfind("(") {
        if let Some(c_parentesis) = nombre.rfind(")") {
            if c_parentesis == nombre.len() - 1 && a_parentesis < c_parentesis {
                let contenido = &nombre[a_parentesis + 1..c_parentesis];
                if contenido.chars().all(|c| c.is_numeric()) {
                    return nombre[..a_parentesis].trim();
                }
            }
        }
    }
    nombre
}

fn main() {
    // direcciones
    let home = env::var("HOME").expect("No se pudo encontrar $HOME");
    let desc = Path::new(&home).join("Descargas");

    let imag = Path::new(&home).join("Imágenes");
    let vide = Path::new(&home).join("Vídeos");
    let song = Path::new(&home).join("Música");
    let data = Path::new(&home).join("Documentos/Datos/");
    let text = Path::new(&home).join("Documentos/Textos/");
    let code = Path::new(&home).join("Códigos");

    let dir_text = ["pdf", "odt", "doc", "docx"];
    let dir_data = ["xlsx", "csv"];
    let dir_vide = ["mp4"];
    let dir_imag = ["png", "jpg", "jpeg", "svg", "webp"];
    let dir_song = ["mp3"];
    let dir_code = ["py", "rs", "cpp", "c", "sh", "js", "ipynb", "toml", "json"];

    // hashmap con las direcciones
    let mut direcciones = HashMap::new();
    Insert!(direcciones, dir_text, &text);
    Insert!(direcciones, dir_data, &data);
    Insert!(direcciones, dir_imag, &imag);
    Insert!(direcciones, dir_song, &song);
    Insert!(direcciones, dir_vide, &vide);
    Insert!(direcciones, dir_code, &code);

    for carpeta in direcciones.values() {
        let _ = fs::create_dir_all(carpeta);
    }

    // estetica, separar los valores
    let mut apenas: bool = true;

    if let Ok(descargas) = fs::read_dir(desc) {
        for entrada in descargas {
            match entrada {
                Ok(archivo) => {
                    let origen = archivo.path();
                    if !origen.is_file() {
                        continue;
                    }
                    if apenas {
                        apenas = false;
                    } else {
                        println!("")
                    }
                    println!("Encontré: {:?}", origen);
                    if let Some(ext_os) = origen.extension() {
                        if let Some(ext) = ext_os.to_str() {
                            if let Some(destino) = direcciones.get(ext.to_lowercase().as_str()) {
                                let nombre = archivo.file_name();
                                let mut ruta_final = Path::new(destino).join(nombre);
                                if ruta_final.exists() {
                                    ruta_final = buscar_nombre(ruta_final.as_path());
                                }
                                match fs::rename(&origen, &ruta_final) {
                                    Ok(_) => println!("Movido {:?} a {:?}", origen, ruta_final),
                                    Err(e) => {
                                        println!("===Error al mover {:?} -> {e}===", origen)
                                    }
                                }
                            } else {
                                println!("Ahi se quedó")
                            }
                        }
                    }
                }
                Err(e) => println!("Error abriendo el archivo {e}"),
            }
        }
    } else {
        println!("No se pudo abrir la carpeta de Descargas")
    }
}
