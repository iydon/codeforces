import pathlib as p
import typing as t

import jinja2
import requests

from bs4 import BeautifulSoup


class Codeforces:
    def __init__(self, link: str) -> None:
        self._link = link
        self._soup = BeautifulSoup(requests.get(link).content, 'lxml')
        self._environment = None

    @property
    def difficulty(self) -> str:
        tags = self._soup.find_all(class_='tag-box')
        return tags[-1].text.strip().replace('*', '')

    @property
    def link(self) -> str:
        return self._link

    @property
    def template(self) -> str:
        __template__ = 'codeforces.template.rs'
        if self._environment is None:
            loader = jinja2.FileSystemLoader(p.Path(__file__).parent)
            self._environment = jinja2.Environment(loader=loader)
            self._environment.globals.update({'repr': repr})
        return self._environment.get_template(__template__).render(codeforces=self)

    @property
    def test_cases(self) -> t.Iterable[t.Tuple[str, str]]:
        f = lambda name: self._soup.find_all(class_=name)
        g = lambda elemenent: elemenent.find('pre').get_text('\n').strip() + '\n'
        for stds in zip(f('input'), f('output')):
            yield tuple(map(g, stds))

    @property
    def title(self) -> str:
        title = self._soup.find(class_='title').text.split(' ', maxsplit=1)[-1]
        return title.replace(' ', '_').replace('\'', '').lower()


if __name__ == '__main__':
    # https://codeforces.com/problemset/problem/270/A
    link = input('(Problem Link) >>> ')
    cf = Codeforces(link)
    directory = p.Path(__file__).parent.parent / 'src'
    # main.rs
    (directory / 'main.rs').write_text(cf.template)
    # archive
    difficulty = directory / 'archive' / cf.difficulty
    difficulty.mkdir(parents=True, exist_ok=True)
    path = difficulty / f'{cf.title}.rs'
    if not path.exists():
        path.write_text(f'// {cf.link}\n')
