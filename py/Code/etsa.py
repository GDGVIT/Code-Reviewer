from pylint import epylint as lint
from io import StringIO
import numpy
(pylint_stdout, pylint_stderr) = lint.py_run('test.py', return_std=True)
print(StringIO.read(pylint_stdout))
