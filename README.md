# rust-code-analysis-python

Python bindings for [rust-code-analysis](https://github.com/mozilla/rust-code-analysis), a Rust library to analyze and collect metrics on source code.

## Installation

```bash
pip install rust-code-analysis-python
```

## Usage

```python
from rust_code_analysis_python import metrics, comment_removal

# Get code metrics
metrics_result = metrics("example.rs", code_string, unit=True)

# Remove comments from code
code_without_comments = comment_removal("example.rs", code_string)
```

## License

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/. 