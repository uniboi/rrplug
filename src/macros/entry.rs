
/// marco used to generate the entry for your plugin
/// 
/// takes in a struct that implements the [`crate::plugin::Plugin`] trait.
#[macro_export]
macro_rules! entry {
    ( $func:ty ) => {
        use rrplug::bindings::plugin_abi::{PluginInitFuncs,PluginNorthstarData};

        #[no_mangle]
        #[export_name = "PLUGIN_INIT"]
        extern "C" fn plguin_init(plugin_init_funcs: *const PluginInitFuncs, plugin_northstar_data: *const PluginNorthstarData) {
            let mut plugin: $func = $crate::plugin::Plugin::new();
            
            plugin.initialize( plugin_init_funcs, plugin_northstar_data );

            std::thread::spawn(move || plugin.main());
        }
    };
}
