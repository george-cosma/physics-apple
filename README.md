# Physics Apple

This is a small passion project centered around turning physics laws (more specifically, Newton's
law of gravity) into art. The program takes a black-and-white video, interprets each white pixel as
a point with mass, generates their gravity field, creates a blank canvas, throws some particles in
there, and lets them loose as the gravity pulls them in such a way the original image can be seen
again.

An example of this working can be seen [on this youtube video](https://youtu.be/bHrUwPVPjWQ). The
use of Bad Apple as a base for the video is the famous [Junferno](https://www.youtube.com/@Junferno)
which has remade this video in a million ways!

This was also an adventure into using CUDA inside rust, with some success. The speedup is noticable
on larger videos. For this code I left a small video of my hand. For other videos, read below and
consider also reading the Makefile.

Improvements to the GPU performance can be made. Firstly, the data could be streamed instead of
processing each frame and stopping in between to save the video. Secondly, more advanced
optimizations can be made to the CUDA code ... the problem is very similar to that of 'fast matrix
multiplication' which has been optimized especially with the rise of machine learning on GPU.

Fair warning! The rust code is not pretty. I've written this as one of my first projects in rust,
and somewhere around 2022. I've improved a lot since. If I were to write this project again, a lot
of changes would be made. Now (2025) I've made a quick pass over the main.rs file to make the CLI
arguments a bit better organized, however it's still very far from clean.

## How to run?

Prerequisites:

- Windows 10+ (I have not tested it on other systems)
- NVIDIA CUDA: <https://developer.nvidia.com/cuda-downloads>
- ffmpeg: <https://www.ffmpeg.org/>
- Visual Studio (required for NVIDIA CUDA) with C/C++ Development tools

You will also need to run a few commands to set the environment varaibles so that `rustacuda`
compiles. Make sure to modify the paths to that of your actual system.

```bash
export CUDA_LIBRARY_PATH="C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.8\lib\x64" # Make sure to modify the path
export PATH=$PATH:"/c/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.41.34120/bin/Hostx64/x64" # Make sure to modify the path
```

To run realtime simulation:

```bash
make
```

To save simulation to file (saved as ./output.mp4):

```bash
make save
```

To clean up:

```bash
make clean
```

For intermediate stepts, look at `Makefile`. Also, when opening with Visual Studio Code, make sure
to modify `.vscode/settings.json` so that rust-analyzer can also compile.

## Custom video

This is a bit trickier and I will not guarantee this will work. First, change the `Makefile` to use
your file instead. Afterwards modify in `src/gui.rs` the `WIDTH` and `HEIGHT` constants so that they
match the video. Feel free to modify the `SCALE` as well, as needed.

Other parameters can be found all over the code. Rendering parameters are found in `main.rs`.

WARNING! Big video files can take hours to days to generate their fields. The Bad Apple video took
me at least 24 hours to render from start to finish.

## Other questions?

Send me a message if you have my contact details, or open an issue otherwise. Hope you enjoy playing
around with gravity :D
