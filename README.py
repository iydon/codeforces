import pathlib
import requests
import tqdm

from bs4 import BeautifulSoup


readme = '''
# [Codeforces](https://codeforces.com/)
| Rust Code | Problem Link | Tags |
| --------- | ------------ | ---- |
'''
directory = pathlib.Path(__file__).parent
for path in tqdm.tqdm((directory/'src'/'archive').iterdir()):
    with open(path, 'r') as f:
        url = f.readline().lstrip('//').strip()
        soup = BeautifulSoup(requests.get(url).content, 'lxml')
        tags = tuple(f'`{tag.text.strip()}`' for tag in soup.find_all(class_='tag-box'))
        readme += f'| [{path.name}]({path.relative_to(directory).as_posix()}) | {url} | {", ".join(tags)} |\n'
(directory/'README.md').write_text(readme)
