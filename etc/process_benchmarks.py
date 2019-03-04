from collections import OrderedDict
from six import iteritems
import os
import json
from jinja2 import Template


def main():
    b = {}
    for (meta_fn, estimates_fn) in benchmark_files():
        with open(meta_fn, 'r') as fh:
            meta = json.loads(fh.read())
            group = meta['group_id']
            function = meta.get('function_id', None) or group

        with open(estimates_fn) as fh:
            contents = json.loads(fh.read())

        if group not in b:
            b[group] = {}
            
        b[group][function] = parse_contents(contents)

    print_benchmarks(b)


def benchmark_files():
    """Iterator for benchmarks.

    Parameters
    ----------
     (none)

    Returns
    -------
     generator

    Usage
    -----
    >>> for fn in benchmark_files():
    ...     with open(fn, 'r') as fh:
    ...         contents = fh.read()

    """
    this_dir, this_filename = os.path.split(__file__)
    criterion_root = os.path.join(this_dir, '..', 'target', 'criterion')
    for dir_name, subdir_list, file_list in os.walk(criterion_root):
        if os.path.split(dir_name)[-1] != 'new':
            continue

        meta = os.path.join(dir_name, 'benchmark.json')
        estimates = os.path.join(dir_name, 'estimates.json')
        yield (meta, estimates)


def parse_contents(contents):
    """Extract information from benchmark.

    Parameters
    ----------
     contents : dict_like
        Benchmark data.

    Returns
    -------
     data : dictionary
        Relevant benchmark data.

    """
    data = {
        'Runtime': contents['Slope']['point_estimate'],
        'Throughput': 1000. / contents['Slope']['point_estimate']
    }
    return data


def print_benchmarks(benchmarks):
    """Use jinja to generate benchmark summary.

    Parameters
    ----------
     benchmarks : dict_like
        Keys are names for benchmarks, values are the
        results. Name/subname?

    Returns
    -------
     (nothing)

    """

    tmpl = """# Benchmarks

All benchmarks run on a 2017 Macbook Pro with a 3.1 GHz Intel Core i7
processor and 16 GB 2133 MHz LPDDR3 RAM.

Group | Function | Runtime (ns) | Throughput (Mops/s)
------|----------|-------------:|-------------------:
{%- for grp_name, grp in benchmarks.items() %}
{%- for fn_name, res in grp.items() %}
{{
   grp_name | replace("_", "\_")
}} | {{
   fn_name | replace("_", "\_")
}} | {{
   '%6.1f'| format(res['Runtime']|float)
}} | {{
   '%6.1f'| format(res['Throughput']|float)
}}
{%- endfor %}
{%- endfor %}"""

    template = Template(tmpl)

    # Sort benchmarks
    for k, v in iteritems(benchmarks):
        benchmarks[k] = OrderedDict(sorted(iteritems(v)))

    benchmarks = OrderedDict(sorted(iteritems(benchmarks)))

    this_dir, this_filename = os.path.split(__file__)
    bfn = os.path.join(this_dir, '..', 'benches', 'README.md')
    with open(bfn, 'w') as fh:
        fh.write(template.render(benchmarks=benchmarks))


if __name__ == '__main__':
    main()
