#include <iostream>
constexpr size_t N{1000 * 7 + 6};

void vector_add(float *out, float *a, float *b, int n) {
    for(int i = 0; i < n; i++){
        out[i] = a[i] + b[i];
    }
}

__global__ void vector_add_cuda_old(float *out, float *a, float *b, int n) {
    for(size_t i = 0; i < n; i++){
        out[i] = a[i] + b[i];
    }
}


__global__ void vector_add_cuda(float *out, float *a, float *b, int n) {
    auto n_new = n / blockDim.x;
    auto mod2 = n % blockDim.x;
    auto i = threadIdx.x * n_new;
    auto j = i + n_new;
    if(threadIdx.x == blockDim.x - 1)
    {
        j+=mod2;
    }


    printf("new_new %d, j %d, i %d mod2 %d blockDin.x %d thread.x %d\n",
         n_new, j, i, mod2, blockDim.x, threadIdx.x);

    for(; i < j; i++){
        out[i] = a[i] + b[i];
    }
}

int main(){
    float *a, *b, *out; 
    float *a_gpu, *b_gpu, *out_gpu; 

    // Allocate memory
    a   = (float*)malloc(sizeof(float) * N);
    b   = (float*)malloc(sizeof(float) * N);
    out = (float*)malloc(sizeof(float) * N);

    auto res = cudaMalloc((void**)&a_gpu, sizeof(float) * N);
    std::cout << "Cuda malloc " << res << ", a_gpu:   "  << a_gpu << std::endl;
   res = cudaMalloc((void**)&b_gpu, sizeof(float) * N);
    std::cout << "Cuda malloc " << res << ", b_gpu:   "  << b_gpu << std::endl;
   res = cudaMalloc((void**)&out_gpu, sizeof(float) * N);
    std::cout << "Cuda malloc " << res << ", out_gpu: "  << out_gpu << std::endl;

    // Initialize array
    for(int i = 0; i < N; i++)
    {
        a[i] = 1.0f; b[i] = 2.0f;
    }
    res =   cudaMemcpy(a_gpu, a, sizeof(float) * N * 1,cudaMemcpyKind::cudaMemcpyHostToDevice);
    std::cout << "Cuda cudaMemcpy " << res << ", a_gpu:   "  << a_gpu << std::endl;

    res = cudaMemcpy(b_gpu, b, sizeof(float) * N,cudaMemcpyKind::cudaMemcpyHostToDevice);
    std::cout << "Cuda cudaMemcpy " << res << ", b_gpu:   "  << b_gpu << std::endl;


    // Main function
    //vector_add(out, a, b, N);

    vector_add_cuda<<<2,25>>>(out_gpu, a_gpu, b_gpu, N);

    
   res = cudaMemcpy(out, out_gpu, sizeof(float) * N * 1, cudaMemcpyKind::cudaMemcpyDeviceToHost);
   std::cout << "Cuda cudaMemcpy " << res << ", out_gpu: "  << out_gpu << std::endl;



    std::cout << out[255] << std::endl;
    std::cout << out[256] << std::endl;
    std::cout << out[257] << std::endl;
    std::cout << out[N-257] << std::endl;
    std::cout << out[N-258] << std::endl;
    std::cout << out[N-4] << std::endl;
    std::cout << out[N-3] << std::endl;
    std::cout << out[N-2] << std::endl;
    std::cout << out[N-1] << std::endl;
}