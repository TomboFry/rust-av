Rust AV
=======

Converts webcam video into CD-audio compatible file. There's some theory below
that I used to calculate frame sizes and rates, but really I don't know what I'm
doing!

No guarantees that this works as it was a spontaneous idea I had back in 2019
and never finished. Uploading here for posterity (and maybe my own future
reference? 👀)

Requires Linux as it uses v4l2 to capture webcam frames.

---

Theory:
=======

CD audio is 44100 samples per second with 16-bit depth.

If we have a frame rate of 10 frames per second, that's 4410 samples per frame.

Therefore, the vsync pulse can be 10 samples long, leaving 4400 samples for
horizontal lines.

Therefore, there's room for 50 lines with horizontal resolution of 88 samples,
minus 10 samples for the hsync pulse at the end of each line. The final
total resolution of each frame is 78x50 at 10 Hz.

All values between 0 and 1024 considered vsync.
All values between 2048 and 3072 considered hsync.
All values above 4096 (up to 65536, because 16-bit) are brightness values?

For example:

--------------------------------------------------------------------------------
                                                                                
              ---                 ---                 ---                       
                 -                   -                   -                      
             -                   -                   -                         -
           --                  --                  --                        -- 
      - ---       --      - ---       --      - ---       --            - ---   
     - -                 - -                 - -                       - -      
                                                                                
                    -----               -----               -----               
                                                                                
-----                                                            -----
--------------------------------------------------------------------------------
