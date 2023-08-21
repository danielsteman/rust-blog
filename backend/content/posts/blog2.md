# BYOB: Build Your Own Blog 🖊️

&nbsp;

_Thu 1 Sep, 2022_

&nbsp;

Maybe you're just like me and you're doing several side projects. Some of them you complete, some of them you put back on the shelf to maybe pick up at a later point in time, or never. Either way, it can be rewarding to look back from time to time and see what you've been doing over the years. At least, that was my motivation. And to get some exposure to frontend dev, which I don't get too often in my daily job.

&nbsp;

Unless you want to invent the wheel yourself (like me), you can go ahead and grab [my project](https://github.com/danielsteman/vite-blog). The instructions in the `README.md` are self explanatory. I didn't think too much about the name of the project, as it started off as a small experiment to see the potential of [Vite](https://vitejs.dev/). To get started:

&nbsp;

```git
git clone https://github.com/danielsteman/vite-blog.git
```

&nbsp;

You'll need to get your [personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token). For local usage, pass it as an environment variable `VITE_GITHUB_TOKEN` or save it in `.env` in the root of the repository. As opposed to Webpack, Vite loads environment variables using `dotenv` [out of the box](https://vitejs.dev/guide/env-and-mode.html), given that the key is prefixed with `VITE_`. If you want to build the Docker image, you'll have to pass it as an argument `PAT`. This means that you also have to set `PAT` in CI builds. I chose a different name because Github doesn't allow secrets with a `GITHUB_` prefix, which causes a problem with the Github action (found in `/.github/workflows`).

&nbsp;

Feel free to open up an issue or pull request! :smiley:
