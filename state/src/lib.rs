
#![no_std]

use io::*;
use gmeta::{ Metadata, metawasm};
use gstd::{ ActorId, prelude::*};


#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {

    //Cargamos ContractMetadata y el estado definido en io.
    pub type State = <ContractMetadata as Metadata>::State;

    //Mostramos el estado, en este caso solo es un vector que contiene el actorID y la cantidad.
    pub fn get_state(state: State) -> Vec<(ActorId, u128)> {
        state
    }



}