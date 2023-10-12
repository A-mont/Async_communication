
#![no_std]
use gmeta::Metadata;
use hashbrown::HashMap;
use io::*;
use gstd::{async_main, msg, exec, prelude::*, ActorId};



#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

// Definimos una estructura de ejemplo para implementarle métodos.
#[derive(Debug, Clone, Default)]
struct Actors {  
    actors: HashMap<ActorId, u128>,
}

// Implementamos métodos en la estructura Actors
impl Actors {

    // Implementamos el metodo destructor de manera Asincrona, este método manda el mensaje mint a un contrato de token fungible
    async fn destructor( &mut self, amount_tokens: u128){

        // Definimos el estado actual usando la función que vuelve mutable el estado y esto  nos permitirá poder modificar el estado.
        let currentstate = state_mut();

        // Esta variable permite obtener la dirección del token fungible.
        let address_ft = addresft_state_mut();

        // Definimos la carga que en este caso será un mensaje de Burn hacia el contrato de token fungible.
        let payload = FTAction::Burn(amount_tokens);
     
        // Usamos la función send_for_reply_as para enviar el mensaje de Burn hacia el contrato de token fungible.
        let result =  msg::send_for_reply_as::<_, FTEvent>(address_ft.ft_program_id,payload,0,0).expect("Error in sending a message").await;
        
        // Actualizamos el vector con el ActorId y la cantidad.
        currentstate.entry(msg::source()).or_insert(amount_tokens); 


        //Usamos este patrón para controlar el error
        let _ = match result {
            Ok(event) => match event {
                FTEvent::Ok => Ok(()),
                _ => Err(()),
            },
            Err(_) => Err(()),
        };
    }

// Implementamos el metodo creator de manera Asincrona, este método lo definimos siguiendo el mismo patrón de diseño anterior.
    async fn creator(&mut self, amount_tokens: u128){

        let currentstate = state_mut();
        let address_ft = addresft_state_mut();           
        let payload = FTAction::Mint(amount_tokens);     
        let result =  msg::send_for_reply_as::<_, FTEvent>(address_ft.ft_program_id,payload,0,0).expect("Error in sending a message").await;
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        let _ = match result {
            Ok(event) => match event {
                FTEvent::Ok => Ok(()),
                _ => Err(()),
            },
            Err(_) => Err(()),
        };
    }

// Implementamos el metodo transfer de manera Asincrona, este método lo definimos siguiendo el mismo patrón de diseño anterior.
    async fn transfer(&mut self, amount_tokens: u128) {
 
        let currentstate = state_mut();
        let address_ft = addresft_state_mut();           
        let payload = FTAction::Transfer{from: exec::program_id(), to: msg::source() ,amount: amount_tokens};
        let _ = msg::send(address_ft.ft_program_id, payload, 0);
        currentstate.entry(msg::source()).or_insert(amount_tokens);  
       


    }

  
   

  
   
}


// Definimos las varibles.
static mut ACTORS:Option<Actors> = None;

static mut STATE:Option<HashMap<ActorId, u128>> = None;

static mut ADDRESSFT:Option<InitFT> = None;


// Definimos esta función para volver mutables las variables estática ACTORS
fn actors_state_mut() -> &'static mut Actors  {

    unsafe { ACTORS.get_or_insert(Default::default()) }


}



// Definimos esta función para volver mutables las variables estática STATE
fn state_mut() -> &'static mut HashMap<ActorId,u128> {

    let state = unsafe { STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }


}

// Definimos esta función para volver mutables las variables estática InitFT
fn addresft_state_mut() -> &'static mut InitFT {


    let addressft = unsafe { ADDRESSFT.as_mut()};

    unsafe { addressft.unwrap_unchecked() }


}

// Definimos esta función INIT para inicializar todas las variables
#[no_mangle]
extern "C" fn init () {

    let config: InitFT = msg::load().expect("Unable to decode InitFT");

    let _actors = Actors {
        ..Default::default()
    };

    if config.ft_program_id.is_zero() {
        panic!("FT program address can't be 0");
    }

    let initft = InitFT {
        ft_program_id: config.ft_program_id
    };

    unsafe {
        ADDRESSFT = Some(initft);
    }

   unsafe { STATE = Some(HashMap::new())}

}

// Definimos esta función manin de forma asincrona usando el macro #[async_main].
#[async_main]
async fn main(){

    // Cargamos la acción del usuario
    let action: Action = msg::load().expect("Could not load Action");

    // Definimos la varible actors para usar las implementaciones 
    let actors = unsafe { ACTORS.get_or_insert(Actors::default()) };


     //Se ejecuta el método en funcion a la accion seleccionada por el usuario.
    match action {
        Action::FTCreate(amount) =>  {
         

                actors.creator(amount).await;
               
 
            },
        Action::FTDestroy(amount) => {

                
                actors.destructor(amount).await;
                     
            }

        Action::FTTransfer(amount) => {
     
                actors.transfer(amount).await;
                
             
            }
           
            };

}

    
     //Este patrón de código de estado es solo para vectores
    #[no_mangle]
    extern "C" fn state() {
     
        let state: <ContractMetadata as Metadata>::State =
            state_mut().iter().map(|(k, v)| (*k, *v)).collect();
         
        msg::reply(state, 0).expect("failed to encode or reply from `state()`");
    }

//Estructura InitFT para inicializar el contrato inteligente.
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitFT {
   
    pub ft_program_id: ActorId,
}



