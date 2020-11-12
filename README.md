CS 
===============

# 一、使用pytorch 榨干 GPU 性能

[ubuntu使用watch命令实时监测显卡](https://blog.csdn.net/zchang81/article/details/72861212) 

[CUDA语义](https://pytorch-cn.readthedocs.io/zh/latest/notes/cuda/) 

使用以下指令检测 GPU 的数据。

```bash
watch -n 1 nvidia-smi

Every 1.0s: nvidia-smi                                                   jerome: Thu Nov 12 20:05:25 2020

Thu Nov 12 20:05:25 2020
+-----------------------------------------------------------------------------+
| NVIDIA-SMI 455.32.00    Driver Version: 455.32.00    CUDA Version: 11.1     |
|-------------------------------+----------------------+----------------------+
| GPU  Name        Persistence-M| Bus-Id        Disp.A | Volatile Uncorr. ECC |
| Fan  Temp  Perf  Pwr:Usage/Cap|         Memory-Usage | GPU-Util  Compute M. |
|                               |                      |               MIG M. |
|===============================+======================+======================|
|   0  TITAN Xp            On   | 00000000:01:00.0 Off |                  N/A |
| 23%   23C    P8    10W / 250W |    204MiB / 12196MiB |      6%      Default |
|                               |                      |                  N/A |
+-------------------------------+----------------------+----------------------+

+-----------------------------------------------------------------------------+
| Processes:                                                                  |
|  GPU   GI   CI        PID   Type   Process name                  GPU Memory |
|        ID   ID                                                   Usage      |
|=============================================================================|
|    0   N/A  N/A      1130      G   /usr/lib/xorg/Xorg                 45MiB |
|    0   N/A  N/A      2102      G   /usr/lib/xorg/Xorg                 80MiB |
|    0   N/A  N/A      2370      G   ...AAAAAAAAA= --shared-files       62MiB |
+-----------------------------------------------------------------------------+
```

## 1.1 torch.cuda API

通过 `dir()` 函数查看 `torch.cuda` 的所有 API，如下。

```python
jerome@jerome:~/2.cs/0.pytorch/0.gpu$ python3
Python 3.8.5 (default, Jul 28 2020, 12:59:40)
[GCC 9.3.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import torch
>>> dir(torch.cuda)
['Any', 'BFloat16Storage', 'BFloat16Tensor', 'BoolStorage', 'BoolTensor', 'ByteStorage', 'ByteTensor', 'Ch
arStorage', 'CharTensor', 'ComplexDoubleStorage', 'ComplexFloatStorage', 'CudaError', 'DeferredCudaCallErr
or', 'Device', 'Dict', 'DoubleStorage', 'DoubleTensor', 'Event', 'FloatStorage', 'FloatTensor', 'HalfStora
ge', 'HalfTensor', 'IntStorage', 'IntTensor', 'List', 'LongStorage', 'LongTensor', 'Optional', 'ShortStora
ge', 'ShortTensor', 'Stream', 'Tuple', 'Union', '_CudaBase', '_CudaDeviceProperties', '_StorageBase', '__a
nnotations__', '__builtins__', '__cached__', '__doc__', '__file__', '__loader__', '__name__', '__package__
', '__path__', '__spec__', '_check_capability', '_check_cubins', '_cudart', '_device', '_device_t', '_dumm
y_type', '_get_device_index', '_initialization_lock', '_initialized', '_is_in_bad_fork', '_lazy_call', '_l
azy_init', '_lazy_new', '_queued_calls', '_sleep', '_tls', '_utils', 'amp', 'caching_allocator_alloc', 'ca
ching_allocator_delete', 'check_error', 'collections', 'contextlib', 'cudaStatus', 'cudart', 'current_blas
_handle', 'current_device', 'current_stream', 'default_generators', 'default_stream', 'device', 'device_co
unt', 'device_of', 'empty_cache', 'get_arch_list', 'get_device_capability', 'get_device_name', 'get_device
_properties', 'get_gencode_flags', 'get_rng_state', 'get_rng_state_all', 'has_half', 'has_magma', 'init',
'initial_seed', 'ipc_collect', 'is_available', 'is_initialized', 'list_gpu_processes', 'manual_seed', 'man
ual_seed_all', 'max_memory_allocated', 'max_memory_cached', 'max_memory_reserved', 'memory', 'memory_alloc
ated', 'memory_cached', 'memory_reserved', 'memory_snapshot', 'memory_stats', 'memory_stats_as_nested_dict
', 'memory_summary', 'nccl', 'nvtx', 'os', 'profiler', 'random', 'reset_accumulated_memory_stats', 'reset_
max_memory_allocated', 'reset_max_memory_cached', 'reset_peak_memory_stats', 'seed', 'seed_all', 'set_devi
ce', 'set_rng_state', 'set_rng_state_all', 'sparse', 'stream', 'streams', 'synchronize', 'threading', 'tor
ch', 'traceback', 'warnings']
```

`torch.cuda` 会记录当前选择的 GPU，并且分配的所有 CUDA 张量将在上面创建。

### 1.1.1 torch.cuda.is_available()

使用 is_available() 来确定系统是否支持 CUDA。

```python
>>> torch.cuda.is_available()
True
```

### 1.1.2 torch.cuda.current_blas_handle()

返回 cublasHandle_t 指针，指向当前 cuBLAS 句柄。

```python
>>> torch.cuda.current_blas_handle()
1511594624
```

### 1.1.3 torch.cuda.current_device()

返回当前所选设备的索引。

```python
>>> torch.cuda.current_device()
0
```

### 1.1.4 torch.cuda.current_stream()

返回一个当前所选的 Stream。

```python
>>> torch.cuda.current_stream()
<torch.cuda.Stream device=cuda:0 cuda_stream=0x0>
```

### 1.1.5 torch.cuda.device()

上下文管理器，可以更改所选设备，用于上下文管理器更改所选设备。

```python
>>> torch.cuda.device(0)
<torch.cuda.device object at 0x7f6b06ec0f40>
```

### 1.1.6 torch.cuda.device_count()

返回可得到的 GPU 数量。

```python
>>> torch.cuda.device_count()
1
```

### 1.1.7 torch.cuda.get_device_name()

获取指定设备 GPU 的名称。

```python
>>> torch.cuda.get_device_name(0)
'TITAN Xp'
```

## 1.2 GPU 与 CPU 数据的相互迁移

[pytorch通过torch.cuda使用GPU加速运算且比较GPU与CPU运算效果以及应用场景](https://ptorch.com/news/53.html) 

[模型和数据(tensor\variable)在cpu和gpu之间的迁移](https://blog.csdn.net/u012370185/article/details/94397486) 

### 1.2.1 将变量从 CPU 传入 GPU

在模型中，我们最常使用的是 `Variable` 这个容器来装载使用数据。主要是由于 `Variable` 可以进行反向传播来进行自动求导。 

不论是什么类型的 Tensor（FloatTensor 或者是 LongTensor 等等），一律直接使用方法 `.cuda()` 即可。

```python
>>> import torch
>>> from torch.autograd import Variable
>>> value = Variable(torch.randn(3, 3))
>>> value
tensor([[ 0.2462, -0.4211,  0.7375],
        [-0.9359,  0.3250,  0.9147],
        [ 0.6220,  1.7273,  0.5605]])
>>> gpu_data = value.cuda(0)
>>> gpu_data
tensor([[ 0.2462, -0.4211,  0.7375],
        [-0.9359,  0.3250,  0.9147],
        [ 0.6220,  1.7273,  0.5605]], device='cuda:0')
```

### 1.2.2 将变量从 GPU 传入 CPU

若要迁移到 cpu 的内存中，则对 cuda 数据类型使用 `.cpu()` 方法即可。

```python
>>> cpu_data = gpu_data.cpu()
>>> cpu_data
tensor([[ 0.2462, -0.4211,  0.7375],
        [-0.9359,  0.3250,  0.9147],
        [ 0.6220,  1.7273,  0.5605]])
```



