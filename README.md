
<https://developer.nvidia.com/cuda-downloads>
export CUDA_LIBRARY_PATH="C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.8\lib\x64" # Make sure to modify the path
ffmpeg -i ../rat.mp4 image-%04d.png
nvcc ./static-field.cu -ptx -ccbin 'C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.41.34120\bin\Hostx64\x64'

nvcc ./static-field.cu -O3 -use_fast_math -fmad=true -ptx -ccbin 'C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.41.34120\bin\Hostx64\x64\cl.exe'

export PATH=$PATH:"/c/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.41.34120/bin/Hostx64/x64"
