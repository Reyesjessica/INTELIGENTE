#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol, Map, Address};

pub struct LecturaEscritura;

#[derive(Clone)]
pub struct Registro {
    lecturas: u32,
    escrituras: u32,
}

#[contractimpl]
impl LecturaEscritura {
    // Inicializa un nuevo registro para un usuario
    pub fn iniciar_usuario(env: Env, usuario: Address) {
        let datos = Map::<Symbol, u32>::new(&env);
        env.storage().instance().set(&usuario, &datos);
    }

    // Registrar una lectura
    pub fn registrar_lectura(env: Env, usuario: Address) {
        let mut datos: Map<Symbol, u32> = env.storage().instance().get(&usuario).unwrap_or_default();
        let count = datos.get(Symbol::short("lecturas")).unwrap_or(0);
        datos.set(Symbol::short("lecturas"), count + 1);
        env.storage().instance().set(&usuario, &datos);
    }

    // Registrar una escritura
    pub fn registrar_escritura(env: Env, usuario: Address) {
        let mut datos: Map<Symbol, u32> = env.storage().instance().get(&usuario).unwrap_or_default();
        let count = datos.get(Symbol::short("escrituras")).unwrap_or(0);
        datos.set(Symbol::short("escrituras"), count + 1);
        env.storage().instance().set(&usuario, &datos);
    }

    // Obtener estado de un usuario
    pub fn obtener_registro(env: Env, usuario: Address) -> Map<Symbol, u32> {
        env.storage().instance().get(&usuario).unwrap_or_default()
    }
}
