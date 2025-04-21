#![no_std]
use soroban_sdk::{contractimpl, symbol_short, Address, Env, Map, Symbol};

pub struct MiContratoRW;

#[contractimpl]
impl MiContratoRW {
    pub fn iniciar_usuario(env: Env, usuario: Address) {
        let datos = Map::<Symbol, u32>::new(&env);
        env.storage().instance().set(&usuario, &datos);
    }

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

    pub fn obtener_registro(env: Env, usuario: Address) -> Map<Symbol, u32> {
        env.storage()
            .instance()
            .get::<Address, Map<Symbol, u32>>(&usuario)
            .unwrap_or_else(|| Map::new(&env))
    }
}

