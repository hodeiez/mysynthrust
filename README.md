# VST3 SYNTH BUILT IN RUST   
used nih-plug as dependency, it's not yet a crate so you need to fetch locally to be able to run this vst synth.  
I used the poly_mod_synth example from nih-plug as starting point.   

## To build the plugin   
run `cargo xtask bundle mysynth --release`  
and get vst3 file in bundled directory