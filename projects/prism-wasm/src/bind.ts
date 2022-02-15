import * as Prism from 'prismjs';



// Require all needed languages
require('prismjs/components/prism-javascript');
require('prismjs/components/prism-jsx');
require('prismjs/components/prism-typescript');
require('prismjs/components/prism-tsx');
require('prismjs/components/prism-css');
require('prismjs/components/prism-css-extras');
require('prismjs/components/prism-scss');
require('prismjs/components/prism-markup');
require('prismjs/components/prism-markdown');
require('prismjs/components/prism-rust');

// Require all needed plugins
require('prismjs/plugins/line-numbers/prism-line-numbers');

export function prism_render(input: string, lang: string): string {
    return Prism.highlight(input, Prism.languages[lang], lang)
}

