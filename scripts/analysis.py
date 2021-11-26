import pathlib as p


code = set()
root = p.Path('..')
code = set(
    sum(
        [
            path.read_text().splitlines()
            for path in root.glob('**/*')
            if path.suffix == '.rs'
        ],
        start=[],
    )
)
print('\n'.join(sorted(code)))
