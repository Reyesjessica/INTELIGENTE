#![no_std]
use soroban_sdk::{contractimpl, symbol_short, Address, Env, Map, Symbol};

// Definición de la estructura del contrato
pub struct MiContratoRW;

#[contractimpl]
impl MiContratoRW {
    /// Inicializa el usuario creando un mapa vacío asociado al usuario en el almacenamiento.
    pub fn iniciar_usuario(env: Env, usuario: Address) {
        let datos = Map::<Symbol, u32>::new(&env);
        env.storage().instance().set(&usuario, &datos);
    }

    /// Registra una lectura incrementando el contador correspondiente.
    pub fn registrar_lectura(env: Env, usuario: Address) {
        let mut datos = env
            .storage()
            .instance()
            .get::<Address, Map<Symbol, u32>>(&usuario)
            .unwrap_or_else(|| Map::new(&env));
        let count = datos.get(symbol_short!("lectura")).unwrap_or(0);
        datos.set(symbol_short!("lectura"), count + 1);
        env.storage().instance().set(&usuario, &datos);
    }

    /// Registra una escritura incrementando el contador correspondiente.
    pub fn registrar_escritura(env: Env, usuario: Address) {
        let mut datos = env
            .storage()
            .instance()
            .get::<Address, Map<Symbol, u32>>(&usuario)
            .unwrap_or_else(|| Map::new(&env));
        let count = datos.get(symbol_short!("escritura")).unwrap_or(0);
        datos.set(symbol_short!("escritura"), count + 1);
        env.storage().instance().set(&usuario, &datos);
    }

    /// Obtiene el registro completo asociado a un usuario, incluyendo lecturas y escrituras.
    pub fn obtener_registro(env: Env, usuario: Address) -> Map<Symbol, u32> {
        env.storage()
            .instance()
            .get::<Address, Map<Symbol, u32>>(&usuario)
            .unwrap_or_else(|| Map::new(&env))
    }

    /// Elimina el registro de un usuario del almacenamiento, si existe.
    pub fn eliminar_usuario(env: Env, usuario: Address) {
        env.storage().instance().remove(&usuario);
    }

    /// Resetear los contadores de lectura y escritura de un usuario.
    pub fn resetear_registro(env: Env, usuario: Address) {
    let mut datos = env
        .storage()
        .instance()
        .get::<Address, Map<Symbol, u32>>(&usuario)
        .unwrap_or_else(|| Map::new(&env));
    datos.set(symbol_short!("lectura"), 0);
    datos.set(symbol_short!("escritura"), 0);
    env.storage().instance().set(&usuario, &datos);
    }

}

