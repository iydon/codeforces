import pathlib
import requests

from bs4 import BeautifulSoup


root = pathlib.Path(__file__).parent
readme = '# [Codeforces](https://codeforces.com/)\n'
for directory in (root/'src'/'archive').iterdir():
    readme += f'## Difficulty: {directory.name}\n' \
        '| Rust Code | Problem Link | Tags |\n' \
        '| --------- | ------------ | ---- |\n'
    for path in directory.iterdir():
        with open(path, 'r') as f:
            url = f.readline().lstrip('//').strip()
            soup = BeautifulSoup(requests.get(url).content, 'lxml')
            tags = tuple(
                f'`{tag.text.strip()}`'
                for tag in soup.find_all(class_='tag-box')
            )
            readme += f'| [{path.name}]({path.relative_to(root).as_posix()}) ' \
                f'| {url} | {", ".join(tags)} |\n'
            print(path)
    readme += '\n'
(root/'README.md').write_text(readme)
