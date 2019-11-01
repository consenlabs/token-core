# Contributing

## Report Bug

While bugs are unfortunate, they're a reality in software. We can't fix what we don't know about, so please report liberally. If you're not sure if something is a bug or not, feel free to file a bug anyway.

```
<short summary of the bug>

I tried this code:

<code sample that causes the bug>

I expected to see this happen: <explanation>

Instead, this happened: <explanation>

## Meta

`rustc --version --verbose`:

Backtrace:
```

## Submit Pull Request

Pull requests are major way to contribute an opensource project. Github has a detailed explaination about [Pull Request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/about-pull-requests).

## Code Style

We expect run `cargo fmt` before code commit. Follow rust community [code style guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md).

CI server already setup to check style on CI build. We recommand you turn on code auto format for you IDE or editor plugin, follow [this](https://github.com/rust-lang/rustfmt#running-rustfmt-from-your-editor) rustfmt document.

## CI

Code submitted must pass all unit tests and static analysis ("lint") checks. We use Travis CI to test code on Linux, macOS.

For failing CI builds, the issue may not be related to the PR itself. Such failures are usually related to flaky tests. These failures can be ignored (authors don't need to fix unrelated issues), but please file a GH issue so the test gets fixed eventually.

## Commit Message

We follow a rough convention for commit message writing.

First line is the subject line, in around 50 charactors or less to describe what changed. And the body of the commit should describe why changed. 

Template:

```
Changes in around 50 charactors or less

Longer explanation of the change in the commit. You can use
multiple sentences here. It's usually best to include content
from the PR description in the final commit message.

issue notices, e.g. "Fixes #42, Resolve #123, See also #456".
```

