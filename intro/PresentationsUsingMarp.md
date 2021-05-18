# Presentations with Marp

The slides in the on the [website](https://tlu.tarilabs.com/) are self hosted in static single file `html` pages. To 
view the rest of the slide decks in the Tari Labs University repo that are not publised on the website, clone the repo 
and open the `html` files in a browser.

All source data for the slide decks are located in the `PITCHME.md` files. The slide decks were generated using 
[Marp](https://marp.app/). This can be done with 
[Marp for VSCode](https://marketplace.visualstudio.com/items?itemName=marp-team.marp-vscode):

- add slide deck metadata
- update image sources to relative paths
- export slide deck to html with marp button
- update iframe source to point to html doc


or with [Marp CLI](https://github.com/marp-team/marp-cli):

- `marp PITCHME.md -o PITCHME.html`

