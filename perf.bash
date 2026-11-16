source .venv/bin/activate
perf record -g python python/nea/render.py
perf script report flamegraph
open flamegraph.html
