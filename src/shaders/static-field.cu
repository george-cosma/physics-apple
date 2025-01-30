extern "C" __constant__ float G = 1.0 / 1000.0;

extern "C" __global__ void gravity(const float mass_product,
                                   const int* x2,
                                   const int* y2,
                                   const int attractors,
                                   float* out_x,
                                   float* out_y,
                                   const int width,
                                   const int height) {
    const int x1 = threadIdx.x + blockIdx.x * blockDim.x;
    const int y1 = threadIdx.y + blockIdx.y * blockDim.y;

    if (x1 >= width || y1 >= height) {
        return;
    }

    const int index = x1 + y1 * width;

    float result_x = 0.0;
    float result_y = 0.0;
    for (int i = 0; i < attractors; i++) {
        const float rx = x2[i] - x1;
        const float ry = y2[i] - y1;
        const float radius_squared = rx * rx + ry * ry;

        const float inv_radius_squared = (radius_squared != 0.0) ? 1.0 / radius_squared : 0.0;
        const float cos_alpha = rx * inv_radius_squared;
        const float sin_alpha = ry * inv_radius_squared;

        result_x += cos_alpha * G * mass_product * inv_radius_squared;
        result_y += sin_alpha * G * mass_product * inv_radius_squared;
    }

    out_x[index] = result_x;
    out_y[index] = result_y;
}