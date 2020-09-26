window.BENCHMARK_DATA = {
  "lastUpdate": 1601159078829,
  "repoUrl": "https://github.com/skinny121/RustPython",
  "entries": {
    "RustPython Benchmark": [
      {
        "commit": {
          "author": {
            "email": "benlewisj@gmail.com",
            "name": "Ben Lewis",
            "username": "skinny121"
          },
          "committer": {
            "email": "benlewisj@gmail.com",
            "name": "Ben Lewis",
            "username": "skinny121"
          },
          "distinct": true,
          "id": "ebe78291bbeef287617a89a0076b16d4f120a4bc",
          "message": "Change trigger to push on test branch.",
          "timestamp": "2020-09-27T11:05:53+13:00",
          "tree_id": "b3d64858ce9f7908b5db4c7bc6856358a23e629b",
          "url": "https://github.com/skinny121/RustPython/commit/ebe78291bbeef287617a89a0076b16d4f120a4bc"
        },
        "date": 1601159078085,
        "tool": "cargo",
        "benches": [
          {
            "name": "bench_cpython_mandelbrot",
            "value": 40069760,
            "range": "± 2278561",
            "unit": "ns/iter"
          },
          {
            "name": "bench_cpython_nbody",
            "value": 7292975,
            "range": "± 816588",
            "unit": "ns/iter"
          },
          {
            "name": "bench_rustpy_mandelbrot",
            "value": 1064884661,
            "range": "± 35788396",
            "unit": "ns/iter"
          },
          {
            "name": "bench_rustpy_nbody",
            "value": 341064845,
            "range": "± 6990166",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}