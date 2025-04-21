#![no_std]

use soroban_sdk::{Env, Address};

/// Función para registrar la participación de un usuario y actualizar su contador.
pub fn registrar_participacion(env: Env, usuario: Address, accion: String) {
    // Guardamos la última acción del usuario
    env.storage().set(&(usuario.clone(), "ultima_accion"), &accion);

    // Obtenemos el contador actual del usuario, si no existe, es 0
    let clave_contador = (usuario.clone(), "contador");
    let contador_actual: u32 = env
        .storage()
        .get(&clave_contador)
        .unwrap_or(Ok(0))
        .unwrap();

    // Actualizamos el contador sumando 1
    env.storage().set(&clave_contador, &(contador_actual + 1));
}

/// Función para leer la participación de un usuario, devolviendo su última acción y contador.
pub fn leer_participacion(env: Env, usuario: Address) -> (String, u32) {
    // Leemos la última acción del usuario, por defecto "Sin registro"
    let accion: String = env
        .storage()
        .get(&(usuario.clone(), "ultima_accion"))
        .unwrap_or(Ok("Sin registro".into()))
        .unwrap();

    // Leemos el contador del usuario, por defecto 0
    let contador: u32 = env
        .storage()
        .get(&(usuario.clone(), "contador"))
        .unwrap_or(Ok(0))
        .unwrap();

    // Devolvemos los dos valores
    (accion, contador)
}

/// Función para verificar si un usuario tiene registro de participación (acción o contador).
pub fn existe_usuario(env: Env, usuario: Address) -> bool {
    // Verificamos si existe la acción registrada
    let tiene_accion = env
        .storage()
        .has(&(usuario.clone(), "ultima_accion"));

    // Verificamos si existe el contador
    let tiene_contador = env
        .storage()
        .has(&(usuario.clone(), "contador"));

    // Devolvemos true si cualquiera de los dos existe
    tiene_accion || tiene_contador
}

/// Función para listar los datos de un usuario en formato JSON (acción y contador).
pub fn listar_datos_usuario_json(env: Env, usuario: Address) -> String {
    // Leemos la última acción del usuario, por defecto "Sin registro"
    let accion: String = env
        .storage()
        .get(&(usuario.clone(), "ultima_accion"))
        .unwrap_or(Ok("Sin registro".into()))
        .unwrap();

    // Leemos el contador del usuario, por defecto 0
    let contador: u32 = env
        .storage()
        .get(&(usuario.clone(), "contador"))
        .unwrap_or(Ok(0))
        .unwrap();

    // Formateamos y devolvemos los datos en formato JSON
    format!("{{ \"accion\": \"{}\", \"contador\": {} }}", accion, contador)
}
