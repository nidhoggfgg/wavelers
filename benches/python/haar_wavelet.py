import pywt
import numpy as np
import timeit

# 1D Haar Wavelet Transform
def haar_wavelet_transform(signal):
    coeffs = pywt.dwt(signal, 'haar')
    return coeffs

# Inverse 1D Haar Wavelet Transform
def inverse_haar_wavelet_transform(approx, detail):
    reconstructed_signal = pywt.idwt(approx, detail, 'haar')
    return reconstructed_signal

# 2D Haar Wavelet Transform
def haar_wavelet_transform_2d(signal):
    coeffs = pywt.dwt2(signal, 'haar')
    return coeffs

# Inverse 2D Haar Wavelet Transform
def inverse_haar_wavelet_transform_2d(ll, lh, hl, hh):
    reconstructed_signal = pywt.idwt2((ll, (lh, hl, hh)), 'haar')
    return reconstructed_signal

# Benchmarking
def benchmark():
    # Setup for 1D Haar Wavelet Transform
    setup_1d = """
import numpy as np
import pywt
signal = np.arange(1024, dtype=np.float64)
"""
    # Benchmark 1D Haar Wavelet Transform
    time_1d = timeit.timeit('haar_wavelet_transform(signal)', setup=setup_1d, globals=globals(), number=1000)
    print(f"1D Haar Wavelet Transform: {time_1d:.6f} seconds")

    # Setup for Inverse 1D Haar Wavelet Transform
    setup_inv_1d = """
import numpy as np
import pywt
approx = np.arange(512, dtype=np.float64)
detail = np.arange(512, dtype=np.float64)
"""
    # Benchmark Inverse 1D Haar Wavelet Transform
    time_inv_1d = timeit.timeit('inverse_haar_wavelet_transform(approx, detail)', setup=setup_inv_1d, globals=globals(), number=1000)
    print(f"Inverse 1D Haar Wavelet Transform: {time_inv_1d:.6f} seconds")

    # Setup for 2D Haar Wavelet Transform
    setup_2d = """
import numpy as np
import pywt
signal = np.arange(1024 * 1024, dtype=np.float64).reshape(1024, 1024)
"""
    # Benchmark 2D Haar Wavelet Transform
    time_2d = timeit.timeit('haar_wavelet_transform_2d(signal)', setup=setup_2d, globals=globals(), number=1000)
    print(f"2D Haar Wavelet Transform: {time_2d:.6f} seconds")

    # Setup for Inverse 2D Haar Wavelet Transform
    setup_inv_2d = """
import numpy as np
import pywt
ll = np.arange(512 * 512, dtype=np.float64).reshape(512, 512)
lh = np.arange(512 * 512, dtype=np.float64).reshape(512, 512)
hl = np.arange(512 * 512, dtype=np.float64).reshape(512, 512)
hh = np.arange(512 * 512, dtype=np.float64).reshape(512, 512)
"""
    # Benchmark Inverse 2D Haar Wavelet Transform
    time_inv_2d = timeit.timeit('inverse_haar_wavelet_transform_2d(ll, lh, hl, hh)', setup=setup_inv_2d, globals=globals(), number=1000)
    print(f"Inverse 2D Haar Wavelet Transform: {time_inv_2d:.6f} seconds")

if __name__ == "__main__":
    benchmark()