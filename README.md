# Tari Labs University

To build and serve the site locally (assuming an Ubuntu operating system):
- Install at least [Ruby 2.7](https://www.ruby-lang.org/en/downloads/) using your preferred method.
- Install build tools: `sudo apt install build-essential`
- Install the bundler: `gem install bundler`
- Navigate to the repository directory
- Install dependencies: `bundle install`
- Serve the site: `bundle exec jekyll serve`
- View the site in a browser using the link provided in your terminal: `http://localhost:4000`

Production builds use [GitHub Pages](https://pages.github.com/) via [GitHub Actions](.github/workflows/github-pages.yml).
