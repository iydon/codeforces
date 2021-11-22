import json
import pathlib
import requests

from bs4 import BeautifulSoup


def extract(path: pathlib.Path) -> dict:
    with open(path, 'r') as f:
        link = f.readline().lstrip('//').strip()
        soup = BeautifulSoup(requests.get(link).content, 'lxml')
        tags = tuple(
            tag.text.strip()
            for tag in soup.find_all(class_='tag-box')
        )
        return {
            'name': path.name,
            'path': path.relative_to(root).as_posix(),
            'link': link,
            'tags': tags,
        }


root = pathlib.Path(__file__).parent
cache_path = root / 'cache.json'
cache = json.loads(cache_path.read_text()) if cache_path.exists() else dict()
readme = '# [Codeforces](https://codeforces.com/)\n## Difficulty\n'
for directory in sorted((root/'src'/'archive').iterdir(), key=lambda p: int(p.name)):
    readme += f'<details>\n<summary>{directory.name}</summary>\n\n' \
        '| ith | Rust Code | Problem Link | Tags |\n' \
        '| --- | --------- | ------------ | ---- |\n'
    for ith, path in enumerate(directory.iterdir()):
        key = path.relative_to(root).relative_to('src', 'archive').as_posix()
        if key not in cache:
            cache[key] = extract(path)
        readme += f'| {ith+1} | [{cache[key]["name"]}]({cache[key]["path"]}) ' \
            f'| {cache[key]["link"]} | `{"`, `".join(cache[key]["tags"])}` |\n'
    readme += '\n</details>\n\n\n'
cache_path.write_text(json.dumps(cache, ensure_ascii=False, indent=4))
(root/'README.md').write_text(readme.rstrip()+'\n')
