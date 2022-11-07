use r2rsplugins::prelude::*;
use std::ptr;

type GetPluginObject = unsafe extern "C" fn(*const PluginObject) -> &'static GameState;
type GetGameStateChar = unsafe extern "C" fn(
    out_buf: *mut ::std::os::raw::c_char,
    out_buf_len: usize,
    var: GameStateInfoType,
) -> ::std::os::raw::c_int;
struct HelloWorld {
    get_plugin_object: Option<GetPluginObject>,
}

impl Plugin for HelloWorld {
    fn new() -> Self {
        Self {
            get_plugin_object: None,
        }
    }

    fn initialize(&mut self, get_plugin_data_external: &c_void) {
        println!("rust plugin initialized");

        // unsafe {
        //     // let thing = &getPluginData_external as &dyn Fn() -> ();
        //     let getPluginObject = get_plugin_data_external as &dyn Fn(c_uint) -> GameState;
        // }

        unsafe {
            let get_plugin_object: GetPluginObject = std::mem::transmute(get_plugin_data_external);

            self.get_plugin_object = Some(get_plugin_object);

            // println!( "{:?}", get_plugin_object(PluginObject_SERVERINFO as *const i32) );
            // let serverInfoPtr: ServerInfo = std::mem::transmute(getPluginObject(get_plugin_object as *const i32));
        }

        // ptr.
        // let getPluginData: PluginObject = getPluginData_external;
        // let gameStatePtr: GameStateInfoType = PluginObject_GAMESTATE;
        // let serverInfoPtr: ServerInfoType = PluginObject_SERVERINFO;
        // let playerInfoPtr: PlayerInfoType = PluginObject_PLAYERINFO;

        // GameState{ getGameStateChar: todo!(), getGameStateInt: todo!(), getGameStateBool: todo!() }

        // .getGameStateChar(  )
    }

    fn main(&self) {
        println!("hello world from rust");

        // wait(3000);

        unsafe {
            let server_info = self.get_plugin_object.unwrap()(PluginObject_GAMESTATE as *const i32);

            // let server_info = ptr::slice_from_raw_parts( server_info_void, 100_usize );

            println!("{:?}", server_info);

            // let gamestate: Vec<i8> = Vec::new();
            // let mut boxed_slice = gamestate.into_boxed_slice();

            // let ptr = boxed_slice.as_mut_ptr();

            let mut score = Box::new(0);
            // let len = score.to_be_bytes().len();
            let mut ptr = score.as_mut();
            

            let func = match server_info.getGameStateInt {
                Some(func) => func,
                None => panic!("coudln't get gamestate char"),
            };

            loop {
                func( ptr, GameStateInfoType_ourScore );
                
                println!("our score is : {:?}", ptr);

                wait( 10000 );
            }

            // let server_info:ServerInfo = ptr::read( server_info_void as *const );

            // let server_info:ServerInfo = std::mem::transmute(server_info_void);
            // let server_info:&ServerInfo = server_info_void as &ServerInfo;
        }
        // panic!("thx for living");
    }
}

entry!(HelloWorld);

// goodies
// https://github.com/emma-miler/NorthstarPluginLibrary/blob/main/NorthstarPluginLibrary/lib/plugin_abi.h
