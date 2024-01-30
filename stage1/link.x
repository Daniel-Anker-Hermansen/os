SECTIONS
{
    /*     
        Placing entry section first so that we can jump to the beginner code emidiately. 
        Specifically reordering .gnu.hash and .synsym to not have conflicts
    */
    . = 0x7E00;
    .entry : { *(.entry) }
    .eh_frame_hdr : { *(.frame_eh_hdr) }
    .eh_frame : { *(.frame_eh) }
}
