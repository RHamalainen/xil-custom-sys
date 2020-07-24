# README
This sys-crate links and exposes an FFI to the "libxil_sf.a" static library. "libxil_sf.a" is a
custom version of the board support package (BSP) provided with the course work for COMP.CE.100. The
only difference between the Xilinx generated BSP and this one is that this one is compiled with
`-mfloat-abi=soft` instead of `-mfloat-abi=hard`.

Caution: using this crate for a modified hardware specification may not work because a particular
set of drivers is always included with a particular build of the BSP. If you need to change the
hardware definition (.hdf) and the respective BSP specification (generated by ie. Vitis), rebuild
the static library (libxil.a) with `-mfloat-abi=soft` and replace the library in the root directory
of this repository (libxil_sf.a).
