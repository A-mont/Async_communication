
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{In, InOut, Metadata};



pub type TransactionId = u64;



// Definir los acciones a usar
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Action {
    FTCreate(u128),
    FTDestroy(u128),
    FTTransfer(u128)
}



// Declarar las funciones del token fungible
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTAction {
    Mint(u128),
    Burn(u128),
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    BalanceOf(ActorId),
}

// Declarar los eventos 
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    SuccessfulCreate,
    SuccessfulDestroy,
    SuccessfulTransfer
}



// Declarar una estructura para el inicio del programa
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitFT {
   
    pub ft_program_id: ActorId,
}


// Declaracion de errores si se necesitan
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Error {
    ZeroAmount,
    ZeroReward,
    ZeroTime,
    TransferTokens,
    PreviousTxMustBeCompleted,
    InsufficentBalance,
    NotOwner,
    StakerNotFound,
    ContractError(String),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Transaction<T> {
    pub id: TransactionId,
    pub action: T,
}


// Declaracion de eventos del token fungible
#[derive(Encode, Decode, TypeInfo)]
pub enum FTEvent {
    Ok,
    Err,
    Balance(u128),
    PermitId(u128),
}


// Declaracion de estructura para el estado
pub struct ContractMetadata;


impl Metadata for ContractMetadata{
    // iniciamos con la estructura InitFT
    type Init = In<InitFT>;
    // Definimos las acciones y eventos para la funcion Handle
     type Handle = InOut<Action,Event>;
     type Others = ();
     type Reply=();
     type Signal = ();
     // Definimos el estado como un vector.
     type State = Vec<(ActorId, u128)>;

}