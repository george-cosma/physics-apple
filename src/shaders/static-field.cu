extern "C" __constant__ float G = 1.0 / 1000.0;

extern "C" __global__ void gravity(const float mass_product, const int* x2, const int* y2, const int attractors, float* out_x, float* out_y) {
    const int x1 = threadIdx.x + blockIdx.x * blockDim.x;
    const int y1 = threadIdx.y + blockIdx.y * blockDim.y;

    // int blockId  = blockIdx.x + blockIdx.y * gridDim.x;
    // int threadId = blockId * (blockDim.x * blockDim.y) 
    //                 + (threadIdx.y * blockDim.x) + threadIdx.x;

    const int index = (threadIdx.x + blockIdx.x * blockDim.x) + (threadIdx.y + blockIdx.y * blockDim.y) * gridDim.x * blockDim.x;
    // if (index == -1) {
    //     printf("(%d, %d)", x1, y1);
    // }
    // printf("%d, ", index);
    // printf("%d\n", gridDim.x * blockDim.x);
    // if (blockIdx.x != 0 || blockIdx.y != 0) {
    //     return;
    // }
    // printf("Putting (%d, %d) at index %d.\n", x1, y1, index);


    for (int i = 0; i < attractors; i++) {
        const float rx = x2[i] - x1;
        const float ry = y2[i] - y1; 
        const float radius_squared = rx * rx + ry * ry;

        if (radius_squared != 0.0) {
            const float cos_alpha = rx / radius_squared;
            const float sin_alpha = ry / radius_squared;
            
            const float prev_x = out_x[index];
            const float prev_y = out_y[index];
            
            out_x[index] = prev_x + cos_alpha * (1.0 / 1000.0) * mass_product / radius_squared;
            out_y[index] = prev_y + sin_alpha * (1.0 / 1000.0) * mass_product / radius_squared;
        }
    }

    // if (out_x[index] == 0.0) {
    //     printf("Y is 0.");
    // }
}