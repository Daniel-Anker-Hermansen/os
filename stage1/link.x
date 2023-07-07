SECTIONS
{
    /*     
        Placing entry section first so that we can jump to the beginner code emidiately. 
        Specifically reordering .gnu.hash and .synsym to not have conflicts
    */
    .entry : { }
    .rodata : { }
    .text : { }
}
